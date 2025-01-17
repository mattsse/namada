//! This module contains types necessary for processing vote extensions.

pub mod bridge_pool_roots;
pub mod ethereum_events;
pub mod validator_set_update;

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

use crate::proto::Signed;

/// This type represents the data we pass to the extension of
/// a vote at the PreCommit phase of Tendermint.
#[derive(
    Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema,
)]
pub struct VoteExtension {
    /// Vote extension data related with Ethereum events.
    pub ethereum_events: Option<Signed<ethereum_events::Vext>>,
    /// A signature of the Ethereum bridge pool root and nonce.
    pub bridge_pool_root: Option<bridge_pool_roots::SignedVext>,
    /// Vote extension data related with validator set updates.
    pub validator_set_update: Option<validator_set_update::SignedVext>,
}

/// The digest of the signatures from different validators
/// in [`VoteExtension`] instances.
///
/// From a [`VoteExtensionDigest`] we yield two signed
/// [`crate::types::transaction::protocol::ProtocolTxType`] transactions:
///   - A `ProtocolTxType::EthereumEvents` tx, and
///   - A `ProtocolTxType::ValidatorSetUpdate` tx
#[derive(
    Debug, Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshSchema,
)]
#[cfg(feature = "abcipp")]
pub struct VoteExtensionDigest {
    /// The digest of Ethereum events vote extension signatures.
    pub ethereum_events: Option<ethereum_events::VextDigest>,
    /// A set of signatures for the current Ethereum bridge pool root and
    /// nonce.
    pub bridge_pool_roots: Option<bridge_pool_roots::MultiSignedVext>,
    /// The digest of validator set updates vote extension signatures.
    pub validator_set_update: Option<validator_set_update::VextDigest>,
}
