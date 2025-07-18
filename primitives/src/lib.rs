#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;

use alloc::string::String;
use codec::{Decode, Encode};
use core::fmt::{Debug, Display, Formatter};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
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

#[derive(Decode, Encode, Debug, Clone, Serialize, Deserialize, Eq, PartialEq, TypeInfo)]
pub struct AssetId(String);

impl codec::MaxEncodedLen for AssetId {
    fn max_encoded_len() -> usize {
        100
    }
}

impl TryFrom<String> for AssetId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.ends_with("_LP") {
            use crate::alloc::string::ToString;
            return Err("asset id can't end with '_LP'".to_string());
        }
        Ok(AssetId(value))
    }
}

impl IntoLiquidityAssetId for AssetId {
    fn into_liquidity_asset_id(&self) -> Self {
        use scale_info::prelude::format;
        if self.0.ends_with("_LP") {
            return self.clone();
        }
        AssetId(format!("{}_LP", self.0.clone()))
    }
}

pub trait IntoLiquidityAssetId {
    fn into_liquidity_asset_id(&self) -> Self;
}

impl Display for AssetId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub struct NucleusInfo<AccountId, Hash, NodeId> {
    pub name: alloc::vec::Vec<u8>,
    pub manager: AccountId,
    pub wasm_hash: Hash,
    pub wasm_version: u32,
    pub wasm_location: Option<NodeId>,
    pub current_event: u64,
    pub root_state: Hash,
    pub validators: alloc::vec::Vec<AccountId>,
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
