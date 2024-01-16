use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{env, fs};

use cosmwasm_schema::cw_serde;
use hpl_interface::types::bech32_decode;
use hyperlane_cosmos::RawCosmosAmount;
use macro_rules_attribute::apply;
use maplit::hashmap;
use tempfile::tempdir;
use tokio;

mod cli;
mod crypto;
mod deploy;
mod link;
mod rpc;
mod source;
mod types;
mod utils;

use rpc::*;
use types::*;
use utils::*;

use crate::cosmos::link::link_networks;
use crate::logging::log;
use crate::metrics::agent_balance_sum;
use crate::program::Program;
use crate::utils::{as_task, concat_path, stop_child, AgentHandles, TaskHandle};
use crate::{fetch_metric, AGENT_BIN_PATH};
use cli::{InjectiveCLI, InjectiveEndpoint};

use self::deploy::deploy_cw_hyperlane;
use self::source::{CLISource, CodeSource};

const INJECTIVE_CLI_GIT: &str = "https://github.com/injective-labs/injective";
const INJECTIVE_CLI_VERSION: &str = "20.5.0";

const KEY_HPL_VALIDATOR: (&str,&str) = ("hpl-validator", "guard evolve region sentence danger sort despair eye deputy brave trim actor left recipe debate document upgrade sustain bus cage afford half demand pigeon");
const KEY_HPL_RELAYER: (&str,&str) = ("hpl-relayer", "moral item damp melt gloom vendor notice head assume balance doctor retire fashion trim find biology saddle undo switch fault cattle toast drip empty");

const KEY_VALIDATOR: (&str,&str) = ("validator", "legend auto stand worry powder idle recall there wet ancient universe badge ability blame hidden body steak april boost thrive room piece city type");
const KEY_ACCOUNTS1: (&str,&str) = ("account1", "stomach employ hidden risk fork parent dream noodle inside banner stable private grain nothing absent brave metal math hybrid amused move affair move muffin");
const KEY_ACCOUNTS2: (&str,&str) = ("account2", "say merry worry steak hedgehog sing spike fold empower pluck feel grass omit finish biology traffic dog sea ozone hint region service one gown");
const KEY_ACCOUNTS3: (&str,&str) = ("account3", "maple often cargo polar eager jaguar eight inflict once nest nice swamp weasel address swift physical valid culture cheese trumpet find dinosaur curve tray");

const ADDR_LENGTH: usize = 32;

fn default_keys<'a>() -> [(&'a str, &'a str); 6] {
    [
        KEY_HPL_RELAYER,
        KEY_VALIDATOR,
        KEY_ACCOUNTS1,
        KEY_ACCOUNTS2,
        KEY_ACCOUNTS3,
        KEY_HPL_VALIDATOR,
    ]
}

const CW_HYPERLANE_GIT: &str = "https://github.com/yorhodes/cw-hyperlane";
const CW_HYPERLANE_VERSION: &str = "0.0.6-rc7";

fn make_target() -> String {
    let os = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        panic!("Current os is not supported by Injective")
    };

    let arch = if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "amd64"
    };

    format!("{}-{}", os, arch)
}

#[cw_serde]
pub struct MockDispatch {
    pub dispatch: MockDispatchInner,
}

#[cw_serde]
pub struct MockDispatchInner {
    pub dest_domain: u32,
    pub recipient_addr: String,
    pub msg_body: String,
    pub hook: Option<String>,
    pub metadata: String,
}

pub fn install_codes(dir: Option<PathBuf>, local: bool) -> BTreeMap<String, PathBuf> {
    let dir_path = match dir {
        Some(path) => path,
        None => tempdir().unwrap().into_path(),
    };

    if !local {
        let dir_path_str = dir_path.to_str().unwrap();

        let release_name = format!("cw-hyperlane-v{CW_HYPERLANE_VERSION}");
        let release_comp = format!("{release_name}.zip");

        log!("Downloading cw-hyperlane v{}", CW_HYPERLANE_VERSION);
        let uri =
            format!("{CW_HYPERLANE_GIT}/releases/download/v{CW_HYPERLANE_VERSION}/{release_comp}");
        download(&release_comp, &uri, dir_path_str);

        log!("Uncompressing cw-hyperlane release");
        unzip(&release_comp, dir_path_str);
    }

    log!("Installing cw-hyperlane in Path: {:?}", dir_path);

    // make contract_name => path map
    fs::read_dir(dir_path)
        .unwrap()
        .map(|v| {
            let entry = v.unwrap();
            (entry.file_name().into_string().unwrap(), entry.path())
        })
        .filter(|(filename, _)| filename.ends_with(".wasm"))
        .map(|v| (v.0.replace(".wasm", ""), v.1))
        .collect()
}

#[allow(dead_code)]
pub fn install_cosmos(
    cli_dir: Option<PathBuf>,
    cli_src: Option<CLISource>,
    codes_dir: Option<PathBuf>,
    _codes_src: Option<CodeSource>,
) -> (PathBuf, BTreeMap<String, PathBuf>) {
    let injectived = cli_src
        .unwrap_or(CLISource::Remote {
            url: INJECTIVE_CLI_GIT.to_string(),
            version: INJECTIVE_CLI_VERSION.to_string(),
        })
        .install(cli_dir);
    let codes = install_codes(codes_dir, false);

    (injectived, codes)
}

#[derive(Clone)]
pub struct CosmosConfig {
    pub cli_path: PathBuf,
    pub home_path: Option<PathBuf>,

    pub codes: BTreeMap<String, PathBuf>,

    pub node_addr_base: String,
    pub node_port_base: u32,

    pub moniker: String,
    pub chain_id: String,
    pub cli_chain_id: String,
}

pub struct CosmosResp {
    pub node: AgentHandles,
    pub endpoint: InjectiveEndpoint,
    pub codes: Codes,
    pub home_path: PathBuf,
}

impl CosmosResp {
    pub fn cli(&self, bin: &Path) -> InjectiveCLI {
        InjectiveCLI::new(bin.to_path_buf(), self.home_path.to_str().unwrap())
    }
}

pub struct CosmosNetwork {
    pub launch_resp: CosmosResp,
    pub deployments: Deployments,
    pub chain_id: String,
    pub metrics_port: u32,
    pub domain: u32,
}

impl Drop for CosmosNetwork {
    fn drop(&mut self) {
        stop_child(&mut self.launch_resp.node.1);
    }
}

impl From<(CosmosResp, Deployments, String, u32, u32)> for CosmosNetwork {
    fn from(v: (CosmosResp, Deployments, String, u32, u32)) -> Self {
        Self {
            launch_resp: v.0,
            deployments: v.1,
            chain_id: v.2,
            metrics_port: v.3,
            domain: v.4,
        }
    }
}
pub struct CosmosHyperlaneStack {
    pub validators: Vec<AgentHandles>,
    pub relayer: AgentHandles,
}

impl Drop for CosmosHyperlaneStack {
    fn drop(&mut self) {
        for v in &mut self.validators {
            stop_child(&mut v.1);
        }
        stop_child(&mut self.relayer.1);
    }
}

async fn launch_cosmos_node(config: CosmosConfig) -> CosmosResp {
    let home_path = match config.home_path {
        Some(v) => v,
        None => tempdir().unwrap().into_path(),
    };
    let cli = InjectiveCLI::new(config.cli_path, home_path.to_str().unwrap());

    cli.init(&config.moniker, &config.cli_chain_id);

    println!("~~~ starting node");
    let (node, endpoint) = cli.start(config.node_addr_base, config.node_port_base);
    sleep(Duration::from_secs(10));
    // tokio::time::sleep(Duration::from_secs(5)).await;
    println!("~~~ node started");
    let codes = cli.store_codes(&endpoint, "validator", config.codes).await;

    CosmosResp {
        node,
        endpoint,
        codes,
        home_path,
    }
}

#[apply(as_task)]
fn launch_cosmos_validator(
    agent_config: AgentConfig,
    agent_config_path: PathBuf,
    debug: bool,
) -> AgentHandles {
    let validator_bin = concat_path(format!("../../{AGENT_BIN_PATH}"), "validator");
    let validator_base = tempdir().expect("Failed to create a temp dir").into_path();
    let validator_base_db = concat_path(&validator_base, "db");

    fs::create_dir_all(&validator_base_db).unwrap();
    println!("Validator DB: {:?}", validator_base_db);

    let checkpoint_path = concat_path(&validator_base, "checkpoint");
    let signature_path = concat_path(&validator_base, "signature");

    let validator = Program::default()
        .bin(validator_bin)
        .working_dir("../../")
        .env("CONFIG_FILES", agent_config_path.to_str().unwrap())
        .env(
            "MY_VALIDATOR_SIGNATURE_DIRECTORY",
            signature_path.to_str().unwrap(),
        )
        .env("RUST_BACKTRACE", "1")
        .hyp_env("CHECKPOINTSYNCER_PATH", checkpoint_path.to_str().unwrap())
        .hyp_env("CHECKPOINTSYNCER_TYPE", "localStorage")
        .hyp_env("ORIGINCHAINNAME", agent_config.name)
        .hyp_env("DB", validator_base_db.to_str().unwrap())
        .hyp_env("METRICSPORT", agent_config.metrics_port.to_string())
        .hyp_env("VALIDATOR_SIGNER_TYPE", agent_config.signer.typ)
        .hyp_env("VALIDATOR_KEY", agent_config.signer.key.clone())
        .hyp_env("VALIDATOR_PREFIX", "inj")
        .hyp_env("SIGNER_SIGNER_TYPE", "hexKey")
        .hyp_env("SIGNER_KEY", agent_config.signer.key)
        .hyp_env("TRACING_LEVEL", if debug { "debug" } else { "info" })
        .spawn("VAL");

    validator
}

#[apply(as_task)]
fn launch_cosmos_relayer(
    agent_config_path: PathBuf,
    relay_chains: Vec<String>,
    metrics: u32,
    debug: bool,
) -> AgentHandles {
    let relayer_bin = concat_path(format!("../../{AGENT_BIN_PATH}"), "relayer");
    let relayer_base = tempdir().unwrap();

    let relayer = Program::default()
        .bin(relayer_bin)
        .working_dir("../../")
        .env("CONFIG_FILES", agent_config_path.to_str().unwrap())
        .env("RUST_BACKTRACE", "1")
        .hyp_env("RELAYCHAINS", relay_chains.join(","))
        .hyp_env("DB", relayer_base.as_ref().to_str().unwrap())
        .hyp_env("ALLOWLOCALCHECKPOINTSYNCERS", "true")
        .hyp_env("TRACING_LEVEL", if debug { "debug" } else { "info" })
        .hyp_env("GASPAYMENTENFORCEMENT", "[{\"type\": \"none\"}]")
        .hyp_env("METRICSPORT", metrics.to_string())
        .spawn("RLY");

    relayer
}

const ENV_CLI_PATH_KEY: &str = "E2E_INJECTIVE_CLI_PATH";
const ENV_CW_HYPERLANE_PATH_KEY: &str = "E2E_CW_HYPERLANE_PATH";

async fn run_locally() {
    const TIMEOUT_SECS: u64 = 60 * 10;
    let debug = true;

    log!("Building rust...");
    Program::new("cargo")
        .cmd("build")
        .working_dir("../../")
        .arg("features", "test-utils")
        .arg("bin", "relayer")
        .arg("bin", "validator")
        .arg("bin", "scraper")
        .arg("bin", "init-db")
        .filter_logs(|l| !l.contains("workspace-inheritance"))
        .run()
        .join();

    let cli_src = Some(
        env::var(ENV_CLI_PATH_KEY)
            .as_ref()
            .map(|v| CLISource::local(v))
            .unwrap_or_default(),
    );

    let code_src = Some(
        env::var(ENV_CW_HYPERLANE_PATH_KEY)
            .as_ref()
            .map(|v| CodeSource::local(v))
            .unwrap_or_default(),
    );

    let (injectived, codes) = install_cosmos(None, cli_src, None, code_src);
    let addr_base = "tcp://0.0.0.0";
    let default_config = CosmosConfig {
        cli_path: injectived.clone(),
        home_path: None,

        codes,

        node_addr_base: addr_base.to_string(),
        node_port_base: 26657,

        moniker: "localnet".to_string(),
        chain_id: "local-node".to_string(),
        cli_chain_id: "local-node".to_string(),
    };

    let port_start = 26600u32;
    let metrics_port_start = 9090u32;
    let domain_start = 99990u32;
    let node_count = 2;

    let mut nodes = vec![];
    for i in 0..node_count {
        let cosmos_node = launch_cosmos_node(CosmosConfig {
            node_port_base: port_start + (i * 10),
            chain_id: format!("injective{}", i + domain_start),
            cli_chain_id: format!("injective-{}", i + domain_start),
            ..default_config.clone()
        })
        .await;
        nodes.push((
            cosmos_node,
            format!("injective{}", i + domain_start),
            metrics_port_start + i,
            domain_start + i,
        ));
    }

    let deployer = "validator";
    let linker = "validator";
    let validator = "hpl-validator";
    let _relayer = "hpl-relayer";

    let mut nodes_deployed = vec![];
    for (launch_resp, chain_id, metrics_port, domain) in nodes {
        let deployments = deploy_cw_hyperlane(
            launch_resp.cli(&injectived),
            launch_resp.endpoint.clone(),
            deployer.to_string(),
            launch_resp.codes.clone(),
            domain,
        )
        .await;
        nodes_deployed.push((launch_resp, deployments, chain_id, metrics_port, domain))
    }

    // nodes with base deployments
    let nodes = nodes_deployed
        .into_iter()
        .map(|v| v.into())
        .collect::<Vec<CosmosNetwork>>();

    for (i, node) in nodes.iter().enumerate() {
        let targets = &nodes[(i + 1)..];

        if !targets.is_empty() {
            println!(
                "LINKING NODES: {} -> {:?}",
                node.domain,
                targets.iter().map(|v| v.domain).collect::<Vec<_>>()
            );
        }

        for target in targets {
            link_networks(&injectived, linker, validator, node, target);
        }
    }

    // for debug
    println!(
        "{}",
        serde_json::to_string(
            &nodes
                .iter()
                .map(|v| (v.domain, v.deployments.clone()))
                .collect::<BTreeMap<_, _>>()
        )
        .unwrap()
    );

    let config_dir = tempdir().unwrap();

    // export agent config
    let agent_config_out = AgentConfigOut {
        chains: nodes
            .iter()
            .map(|v| {
                (
                    format!("injective{}", v.domain),
                    AgentConfig::new(injectived.clone(), validator, v),
                )
            })
            .collect::<BTreeMap<String, AgentConfig>>(),
    };
    println!(
        "~~~ validator configs: {:?}",
        agent_config_out.chains.values().collect::<Vec<_>>()
    );

    let json_agent_config = serde_json::to_string_pretty(&agent_config_out).unwrap();
    println!("~~~ json agent config: {:?}", json_agent_config);

    let agent_config_path = concat_path(&config_dir, "config.json");
    fs::write(&agent_config_path, json_agent_config).unwrap();

    let hpl_val = agent_config_out
        .chains
        .clone()
        .into_values()
        .map(|agent_config| launch_cosmos_validator(agent_config, agent_config_path.clone(), debug))
        .collect::<Vec<_>>();
    let hpl_rly_metrics_port = metrics_port_start + node_count + 1u32;
    let hpl_rly = launch_cosmos_relayer(
        agent_config_path,
        agent_config_out.chains.into_keys().collect::<Vec<_>>(),
        hpl_rly_metrics_port,
        debug,
    );

    // give things a chance to fully start.
    sleep(Duration::from_secs(10));

    let starting_relayer_balance: f64 = agent_balance_sum(hpl_rly_metrics_port).unwrap();

    // dispatch messages
    let mut dispatched_messages = 0;

    for node in nodes.iter() {
        let targets = nodes
            .iter()
            .filter(|v| v.domain != node.domain)
            .collect::<Vec<_>>();

        if !targets.is_empty() {
            println!(
                "DISPATCHING MAILBOX: {} -> {:?}",
                node.domain,
                targets.iter().map(|v| v.domain).collect::<Vec<_>>()
            );
        }

        for target in targets {
            dispatched_messages += 1;
            let cli = InjectiveCLI::new(
                injectived.clone(),
                node.launch_resp.home_path.to_str().unwrap(),
            );

            let msg_body: &[u8; 5] = b"hello";

            let recipient_addr = bech32_decode(&target.deployments.mock_receiver).unwrap();
            let left_padded_zeroes = ADDR_LENGTH - recipient_addr.len();
            let recipient_addr = [vec![0u8; left_padded_zeroes], recipient_addr].concat();

            cli.wasm_execute(
                &node.launch_resp.endpoint,
                linker,
                &node.deployments.mailbox,
                MockDispatch {
                    dispatch: MockDispatchInner {
                        dest_domain: target.domain,
                        recipient_addr: hex::encode(recipient_addr),
                        msg_body: hex::encode(msg_body),
                        hook: None,
                        metadata: "".to_string(),
                    },
                },
                vec![RawCosmosAmount {
                    denom: "inj".to_string(),
                    amount: 25_000_000.to_string(),
                }],
            );
        }
    }

    let _stack = CosmosHyperlaneStack {
        validators: hpl_val.into_iter().map(|v| v.join()).collect(),
        relayer: hpl_rly.join(),
    };

    // Mostly copy-pasta from `rust/utils/run-locally/src/main.rs`
    // TODO: refactor to share code
    let loop_start = Instant::now();
    let mut failure_occurred = false;
    loop {
        // look for the end condition.
        if termination_invariants_met(
            hpl_rly_metrics_port,
            dispatched_messages,
            starting_relayer_balance,
        )
        .unwrap_or(false)
        {
            // end condition reached successfully
            break;
        } else if (Instant::now() - loop_start).as_secs() > TIMEOUT_SECS {
            // we ran out of time
            log!("timeout reached before message submission was confirmed");
            failure_occurred = true;
            break;
        }

        sleep(Duration::from_secs(5));
    }

    if failure_occurred {
        panic!("E2E tests failed");
    } else {
        log!("E2E tests passed");
    }
}

fn termination_invariants_met(
    relayer_metrics_port: u32,
    messages_expected: u32,
    starting_relayer_balance: f64,
) -> eyre::Result<bool> {
    let gas_payments_scraped = fetch_metric(
        &relayer_metrics_port.to_string(),
        "hyperlane_contract_sync_stored_events",
        &hashmap! {"data_type" => "gas_payment"},
    )?
    .iter()
    .sum::<u32>();
    let expected_gas_payments = messages_expected;
    if gas_payments_scraped != expected_gas_payments {
        log!(
            "Scraper has scraped {} gas payments, expected {}",
            gas_payments_scraped,
            expected_gas_payments
        );
        return Ok(false);
    }

    let delivered_messages_scraped = fetch_metric(
        &relayer_metrics_port.to_string(),
        "hyperlane_operations_processed_count",
        &hashmap! {"phase" => "confirmed"},
    )?
    .iter()
    .sum::<u32>();
    if delivered_messages_scraped != messages_expected {
        log!(
            "Relayer confirmed {} submitted messages, expected {}",
            delivered_messages_scraped,
            messages_expected
        );
        return Ok(false);
    }

    let ending_relayer_balance: f64 = agent_balance_sum(relayer_metrics_port).unwrap();

    // Make sure the balance was correctly updated in the metrics.
    // Ideally, make sure that the difference is >= gas_per_tx * gas_cost, set here:
    // https://github.com/hyperlane-xyz/hyperlane-monorepo/blob/c2288eb31734ba1f2f997e2c6ecb30176427bc2c/rust/utils/run-locally/src/cosmos/cli.rs#L55
    // What's stopping this is that the format returned by the `inj` balance query is a surprisingly low number (0.000003999999995184)
    // but then maybe the gas_per_tx is just very low - how can we check that? (maybe by simulating said tx)
    if starting_relayer_balance <= ending_relayer_balance {
        log!(
            "Expected starting relayer balance to be greater than ending relayer balance, but got {} <= {}",
            starting_relayer_balance,
            ending_relayer_balance
        );
        return Ok(false);
    }

    log!("Termination invariants have been meet");
    Ok(true)
}

#[cfg(feature = "cosmos")]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        run_locally().await
    }
}
