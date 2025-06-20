#![cfg_attr(not(feature = "std"), no_std)]
use codec::Codec;
pub use pallet_swap::rpc::{RpcError, RpcResult};
use sp_runtime::traits::MaybeDisplay;

sp_api::decl_runtime_apis! {
    pub trait SwapApi<AssetIdA, Balance, AssetBalance> where
        AssetIdA: Codec + MaybeDisplay,
        Balance: Codec + MaybeDisplay,
        AssetBalance: Codec + MaybeDisplay,
    {
        fn get_currency_to_asset_output_amount(asset_id: AssetIdA, currency_amount: Balance) -> RpcResult<AssetBalance>;
        fn get_currency_to_asset_input_amount(asset_id: AssetIdA, token_amount: AssetBalance) -> RpcResult<Balance>;
        fn get_asset_to_currency_output_amount(asset_id: AssetIdA, token_amount: AssetBalance) -> RpcResult<Balance>;
        fn get_asset_to_currency_input_amount(asset_id: AssetIdA, currency_amount: Balance) -> RpcResult<AssetBalance>;
    }
}
