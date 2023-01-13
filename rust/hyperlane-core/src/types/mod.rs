pub use primitive_types::{H128, H160, H256, H512, U128, U256, U512};
use std::io::{Read, Write};
use std::ops::Add;

pub use announcement::*;
pub use chain_data::*;
pub use checkpoint::*;
pub use log_metadata::*;
pub use message::*;

use crate::{Decode, Encode, HyperlaneProtocolError};

mod announcement;
mod chain_data;
mod checkpoint;
mod log_metadata;
mod message;

/// Unified 32-byte identifier with convenience tooling for handling
/// 20-byte ids (e.g ethereum addresses)
pub mod identifiers;

/// A payment of a message's gas costs.
#[derive(Debug, Copy, Clone)]
pub struct InterchainGasPayment {
    /// The id of the message
    pub message_id: H256,
    /// The amount of native tokens paid.
    pub payment: U256,
    /// The amount of destination gas paid for.
    pub gas_amount: U256,
}

impl Add for InterchainGasPayment {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        assert_eq!(
            self.message_id, rhs.message_id,
            "Cannot add interchain gas payments for different messages"
        );
        Self {
            message_id: self.message_id,
            payment: self.payment + rhs.payment,
            gas_amount: self.gas_amount + rhs.gas_amount,
        }
    }
}

/// Uniquely identifying metadata for an InterchainGasPayment
#[derive(Debug)]
pub struct InterchainGasPaymentMeta {
    /// The transaction hash in which the GasPayment log was emitted
    pub transaction_hash: H256,
    /// The index of the GasPayment log within the transaction's logs
    pub log_index: U256,
}

impl Encode for InterchainGasPaymentMeta {
    fn write_to<W>(&self, writer: &mut W) -> std::io::Result<usize>
    where
        W: Write,
    {
        let mut written = 0;
        written += self.transaction_hash.write_to(writer)?;
        written += self.log_index.write_to(writer)?;
        Ok(written)
    }
}

impl Decode for InterchainGasPaymentMeta {
    fn read_from<R>(reader: &mut R) -> Result<Self, HyperlaneProtocolError>
    where
        R: Read,
        Self: Sized,
    {
        Ok(Self {
            transaction_hash: H256::read_from(reader)?,
            log_index: U256::read_from(reader)?,
        })
    }
}

/// An InterchainGasPayment with metadata to uniquely identify the payment
#[derive(Debug)]
pub struct InterchainGasPaymentWithMeta {
    /// The InterchainGasPayment
    pub payment: InterchainGasPayment,
    /// Metadata for the payment
    pub meta: InterchainGasPaymentMeta,
}

/// A cost estimate for a transaction.
#[derive(Clone, Debug)]
pub struct TxCostEstimate {
    /// The gas limit for the transaction.
    pub gas_limit: U256,
    /// The gas price for the transaction.
    pub gas_price: U256,
}
