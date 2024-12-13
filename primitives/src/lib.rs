#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    AccountId32, MultiAddress, MultiSignature,
};

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

pub type NucleusId = AccountId32;

pub type NodeId = sp_core::OpaquePeerId;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusInfo<AccountId, Hash, NodeId> {
    pub name: alloc::vec::Vec<u8>,
    pub manager: AccountId,
    pub wasm_hash: Hash,
    pub wasm_version: u32,
    pub wasm_location: Option<NodeId>,
    pub current_event: u64,
    pub root_state: Hash,
    pub peers: alloc::vec::Vec<NodeId>,
}

use sp_core::{
    crypto::{Pair, UncheckedFrom},
    ecdsa, ed25519, sr25519,
};
mod app_sr25519 {
    use sp_application_crypto::{app_crypto, sr25519};
    use sp_core::crypto::KeyTypeId;
    app_crypto!(sr25519, KeyTypeId(*b"fuba"));
}

sp_application_crypto::with_pair! {
    /// An i'm online keypair using sr25519 as its crypto.
    pub type AuthorityPair = app_sr25519::Pair;
}

/// An i'm online signature using sr25519 as its crypto.
pub type AuthoritySignature = app_sr25519::Signature;

/// An i'm online identifier using sr25519 as its crypto.
pub type CryptoApproach = app_sr25519::Public;
/// An i'm online identifier using sr25519 as its crypto.
pub type AppCryptoApproach = sp_application_crypto::sr25519::AppPublic;
