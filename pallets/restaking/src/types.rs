use super::*;
use crate::validator_data::ValidatorData;
use frame_support::Serialize;
use frame_system::pallet_prelude::BlockNumberFor;
use serde_json::Value;

fn account_deserialize_from_hex_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: Decode,
    D: Deserializer<'de>,
{
    let account_id_str: String = Deserialize::deserialize(deserializer)?;
    let account_id_hex =
        hex::decode(&account_id_str[2..]).map_err(|e| de::Error::custom(e.to_string()))?;
    S::decode(&mut &account_id_hex[..]).map_err(|e| de::Error::custom(e.to_string()))
}

fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: sp_std::str::FromStr,
    D: Deserializer<'de>,
    <S as sp_std::str::FromStr>::Err: ToString,
{
    let amount_str: String = Deserialize::deserialize(deserializer)?;
    amount_str
        .parse::<S>()
        .map_err(|e| de::Error::custom(e.to_string()))
}

/// Validator of appchain.
#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Validator<AccountId> {
    /// The validator's id.
    #[serde(deserialize_with = "account_deserialize_from_hex_str")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    pub validator_id_in_appchain: AccountId,
    /// The total stake of this validator in mainchain's staking system.
    #[serde(deserialize_with = "deserialize_from_str")]
    pub total_stake: u128,
}

#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ValidatorSet<AccountId> {
    /// The anchor era that this set belongs to.
    pub set_id: u32,
    /// Validators in this set.
    #[serde(bound(deserialize = "AccountId: Decode"))]
    pub validators: Vec<Validator<AccountId>>,
}

/// Appchain token burn event.
#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct BurnEvent<AccountId> {
    #[serde(default)]
    pub index: u32,
    #[serde(rename = "sender_id_in_near")]
    #[serde(with = "serde_bytes")]
    pub sender_id: Vec<u8>,
    #[serde(rename = "receiver_id_in_appchain")]
    #[serde(deserialize_with = "account_deserialize_from_hex_str")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    pub receiver: AccountId,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub amount: u128,
}

#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct LockAssetEvent<AccountId> {
    #[serde(default)]
    pub index: u32,
    #[serde(rename = "contract_account")]
    #[serde(with = "serde_bytes")]
    pub token_id: Vec<u8>,
    #[serde(rename = "sender_id_in_near")]
    #[serde(with = "serde_bytes")]
    pub sender_id: Vec<u8>,
    #[serde(rename = "receiver_id_in_appchain")]
    #[serde(deserialize_with = "account_deserialize_from_hex_str")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    pub receiver: AccountId,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub amount: u128,
}

/// Appchain token lock event.
#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct BurnNftEvent<AccountId> {
    #[serde(default)]
    pub index: u32,
    #[serde(rename = "sender_id_in_near")]
    #[serde(with = "serde_bytes")]
    pub sender_id: Vec<u8>,
    #[serde(rename = "receiver_id_in_appchain")]
    #[serde(deserialize_with = "account_deserialize_from_hex_str")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    pub receiver: AccountId,
    #[serde(rename = "class_id")]
    #[serde(deserialize_with = "deserialize_from_str")]
    pub collection: u128,
    #[serde(rename = "token_id")]
    #[serde(deserialize_with = "deserialize_from_str")]
    pub item: u128,
}

#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum AppchainNotification<AccountId> {
    #[serde(rename = "NearFungibleTokenLocked")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    LockAsset(LockAssetEvent<AccountId>),

    #[serde(rename = "WrappedAppchainTokenBurnt")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    Burn(BurnEvent<AccountId>),

    // #[serde(rename = "WrappedNonFungibleTokenBurnt")]
    #[serde(rename = "WrappedAppchainNFTLocked")]
    #[serde(bound(deserialize = "AccountId: Decode"))]
    BurnNft(BurnNftEvent<AccountId>),
}

#[derive(Deserialize, RuntimeDebug)]
pub struct AppchainNotificationHistory<AccountId> {
    #[serde(bound(deserialize = "AccountId: Decode"))]
    pub appchain_notification: AppchainNotification<AccountId>,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub index: u32,
}

#[derive(PartialEq, Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
pub enum NotificationResult {
    Success,
    UnlockFailed,
    AssetMintFailed,
    AssetGetFailed,
    NftUnlockFailed,
}

#[derive(Deserialize, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Observation<AccountId> {
    #[serde(bound(deserialize = "AccountId: Decode"))]
    UpdateValidatorSet(ValidatorSet<AccountId>),
    #[serde(bound(deserialize = "AccountId: Decode"))]
    LockAsset(LockAssetEvent<AccountId>),
    #[serde(bound(deserialize = "AccountId: Decode"))]
    Burn(BurnEvent<AccountId>),
    #[serde(bound(deserialize = "AccountId: Decode"))]
    BurnNft(BurnNftEvent<AccountId>),
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum ObservationType {
    UpdateValidatorSet,
    Burn,
    LockAsset,
    BurnNft,
}

impl<AccountId> Observation<AccountId> {
    pub fn observation_index(&self) -> u32 {
        match self {
            Observation::UpdateValidatorSet(set) => set.set_id,
            Observation::LockAsset(event) => event.index,
            Observation::Burn(event) => event.index,
            Observation::BurnNft(event) => event.index,
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ObservationsPayload<Public, BlockNumber> {
    pub public: Public,
    pub key_data: Vec<u8>,
    pub block_number: BlockNumber,
    pub observations: Vec<ValidatorData>,
}

impl<T: SigningTypes> SignedPayload<T> for ObservationsPayload<T::Public, BlockNumberFor<T>> {
    fn public(&self) -> T::Public {
        self.public.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonRequest {
    pub id: u32,
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Value>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TxParams {
    #[serde(rename = "accessList")]
    pub access_list: Vec<u32>,
    pub data: String,
    pub to: String,
    #[serde(rename = "type")]
    pub ntype: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct JsonResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: String,
}

pub struct OperatorDirectedRewardSubmission {
    pub operator_rewards: Vec<OperatorReward>,
    pub start_timestamp: u32,
    pub duration: u32,
    pub description: String,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct OperatorReward {
    pub validator: ValidatorData,
    pub amount: u128,
}

#[derive(PartialEq, Encode, Decode, TypeInfo)]
pub struct EraRewardDetailsValue {
    pub total: u128,
    pub timestamp: u64,
    pub details: Vec<OperatorReward>,
}

impl Default for EraRewardDetailsValue {
    fn default() -> Self {
        EraRewardDetailsValue {
            total: 0,
            timestamp: 0,
            details: vec![],
        }
    }
}
