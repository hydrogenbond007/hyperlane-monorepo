use async_trait::async_trait;

use cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse;
use hyperlane_core::{
    Announcement, ChainResult, ContractLocator, HyperlaneChain, HyperlaneContract, HyperlaneDomain,
    HyperlaneProvider, SignedType, TxOutcome, ValidatorAnnounce, H256, H512, U256,
};

use crate::{
    grpc::{WasmGrpcProvider, WasmProvider},
    payloads::validator_announce::{
        self, AnnouncementRequest, AnnouncementRequestInner, GetAnnounceStorageLocationsRequest,
        GetAnnounceStorageLocationsRequestInner,
    },
    signers::Signer,
    ConnectionConf,
};

/// A reference to a ValidatorAnnounce contract on some Cosmos chain
#[derive(Debug)]
pub struct CosmosValidatorAnnounce<'a> {
    _conf: &'a ConnectionConf,
    locator: &'a ContractLocator<'a>,
    _signer: &'a Signer,
    provider: Box<WasmGrpcProvider<'a>>,
}

impl<'a> CosmosValidatorAnnounce<'a> {
    /// create a new instance of CosmosValidatorAnnounce
    pub fn new(conf: &'a ConnectionConf, locator: &'a ContractLocator, signer: &'a Signer) -> Self {
        let provider = WasmGrpcProvider::new(conf, locator, signer);

        Self {
            _conf: conf,
            locator,
            _signer: signer,
            provider: Box::new(provider),
        }
    }
}

impl HyperlaneContract for CosmosValidatorAnnounce<'_> {
    fn address(&self) -> H256 {
        self.locator.address
    }
}

impl HyperlaneChain for CosmosValidatorAnnounce<'_> {
    fn domain(&self) -> &HyperlaneDomain {
        self.locator.domain
    }

    fn provider(&self) -> Box<dyn HyperlaneProvider> {
        todo!()
    }
}

#[async_trait]
impl ValidatorAnnounce for CosmosValidatorAnnounce<'_> {
    async fn get_announced_storage_locations(
        &self,
        validators: &[H256],
    ) -> ChainResult<Vec<Vec<String>>> {
        let payload = GetAnnounceStorageLocationsRequest {
            get_announce_storage_locations: GetAnnounceStorageLocationsRequestInner {
                validators: validators
                    .iter()
                    .map(|v| hex::encode(v.as_bytes()))
                    .collect::<Vec<String>>(),
            },
        };

        let data: Vec<u8> = self.provider.wasm_query(payload, None).await?;
        let response: validator_announce::GetAnnounceStorageLocationsResponse =
            serde_json::from_slice(&data)?;

        Ok(response
            .storage_locations
            .into_iter()
            .map(|v| v.1)
            .collect())
    }

    async fn announce(
        &self,
        announcement: SignedType<Announcement>,
        tx_gas_limit: Option<U256>,
    ) -> ChainResult<TxOutcome> {
        let announce_request = AnnouncementRequest {
            announcement: AnnouncementRequestInner {
                validator: announcement.value.validator.to_string(),
                storage_location: announcement.value.storage_location,
                signature: hex::encode(announcement.signature.to_vec()),
            },
        };

        let response: TxResponse = self
            .provider
            .wasm_send(announce_request, tx_gas_limit)
            .await?;
        Ok(TxOutcome {
            transaction_id: H512::from_slice(hex::decode(response.txhash).unwrap().as_slice()),
            executed: response.code == 0,
            gas_used: U256::from(response.gas_used),
            gas_price: U256::from(response.gas_wanted),
        })
    }

    async fn announce_tokens_needed(&self, announcement: SignedType<Announcement>) -> Option<U256> {
        todo!() // not implemented yet
    }
}