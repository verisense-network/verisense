use crate::{Call, Config, Pallet};
use codec::{Decode, Encode};
use frame_support::pallet_prelude::TypeInfo;
use frame_system::offchain::{SendUnsignedTransaction, SignedPayload, Signer, SigningTypes};
use frame_system::pallet_prelude::BlockNumberFor;
use log::warn;
use sp_core::RuntimeDebug;
use sp_std::prelude::*;

impl<T: Config> Pallet<T> {
    pub fn submit_unsigned_transaction(
        block_number: BlockNumberFor<T>,
        public: <T as SigningTypes>::Public,
        key_data: Vec<u8>,
        verified_mcps: Vec<T::AccountId>,
    ) -> Result<(), &'static str> {
        let result = Signer::<T, T::AppCrypto>::all_accounts()
            .with_filter(vec![public])
            .send_unsigned_transaction(
                |account| VerifiedMCPPayload {
                    public: account.public.clone(),
                    key_data: key_data.clone(),
                    block_number,
                    observations: verified_mcps.clone(),
                },
                |payload, signature| Call::after_dns_verify { payload, signature },
            );
        if result.len() != 1 {
            return Err("No account found");
        }
        if result[0].1.is_err() {
            warn!("Failed to update_validators: {:?}", result[0].1);
            return Err("Failed to update_validators");
        }
        Ok(())
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct VerifiedMCPPayload<Public, AccountId, BlockNumber> {
    pub public: Public,
    pub key_data: Vec<u8>,
    pub block_number: BlockNumber,
    pub observations: Vec<AccountId>,
}

impl<T: SigningTypes> SignedPayload<T>
    for VerifiedMCPPayload<T::Public, T::AccountId, BlockNumberFor<T>>
{
    fn public(&self) -> T::Public {
        self.public.clone()
    }
}
