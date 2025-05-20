#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::string::String;
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

pub type AssetId = u32;

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

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, TypeInfo)]
pub struct RewardsProof {
    pub proof: alloc::vec::Vec<String>,
    pub amount: String,
}

pub mod keys {
    use sp_core::crypto::KeyTypeId;

    pub const RESTAKING_KEY_TYPE: KeyTypeId = KeyTypeId(*b"rstk");
    pub const NUCLEUS_VRF_KEY_TYPE: KeyTypeId = KeyTypeId(*b"nvrf");

    pub mod restaking {
        mod app_sr25519 {
            use crate::keys::RESTAKING_KEY_TYPE;
            use sp_runtime::app_crypto::{app_crypto, sr25519};
            app_crypto!(sr25519, RESTAKING_KEY_TYPE);
        }

        sp_application_crypto::with_pair! {
            pub type AuthorityPair = app_sr25519::Pair;
        }

        pub type AuthoritySignature = app_sr25519::Signature;

        pub type AuthorityId = app_sr25519::Public;
    }

    pub mod vrf {
        mod app_sr25519 {
            use crate::keys::NUCLEUS_VRF_KEY_TYPE;
            use sp_runtime::app_crypto::{app_crypto, sr25519};
            app_crypto!(sr25519, NUCLEUS_VRF_KEY_TYPE);
        }

        sp_application_crypto::with_pair! {
            pub type AuthorityPair = app_sr25519::Pair;
        }

        pub type AuthoritySignature = app_sr25519::Signature;

        pub type AuthorityId = app_sr25519::Public;
    }
}
