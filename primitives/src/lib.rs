#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    MultiSignature,
};
use sp_std::vec::Vec;

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

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, TypeInfo, Debug)]
pub struct Nucleus<AccountId, Hash> {
    pub name: Vec<u8>,
    pub account: AccountId,
    pub wasm_url: Vec<u8>,
    pub wasm_hash: Hash,
    pub wasm_version: u32,
    pub energy: u128,
    pub current_event: u64,
    pub root_state: Hash,
}
