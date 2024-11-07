#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    traits::{DisabledValidators, FindAuthor, Get, OnTimestampSet, OneSessionHandler},
    BoundedSlice, BoundedVec, ConsensusEngineId, Parameter,
};
// use log;
use sp_consensus_aura::{AuthorityIndex, ConsensusLog, Slot, AURA_ENGINE_ID};
use sp_runtime::{
    generic::DigestItem,
    traits::{IsMember, Member, SaturatedConversion, Saturating, Zero},
    RuntimeAppPublic,
};

// pub mod migrations;
// mod mock;
// mod tests;

pub use pallet::*;

// const LOG_TARGET: &str = "runtime::aura";

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: pallet_timestamp::Config + frame_system::Config {
        /// The identifier type for an authority.
        type AuthorityId: Member
            + Parameter
            + RuntimeAppPublic
            + MaybeSerializeDeserialize
            + MaxEncodedLen;

        // /// The maximum number of authorities that the pallet can hold.
        // type MaxAuthorities: Get<u32>;

        // /// A way to check whether a given validator is disabled and should not be authoring blocks.
        // /// Blocks authored by a disabled validator will lead to a panic as part of this module's
        // /// initialization.
        // type DisabledValidators: DisabledValidators;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(core::marker::PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_: BlockNumberFor<T>) -> Weight {
            T::DbWeight::get().reads(1)
        }
    }

    // /// The current authority set.
    // #[pallet::storage]
    // pub type Authorities<T: Config> =
    //     StorageValue<_, BoundedVec<T::AuthorityId, T::MaxAuthorities>, ValueQuery>;

    // /// The current slot of this block.
    // ///
    // /// This will be set in `on_initialize`.
    // #[pallet::storage]
    // pub type CurrentSlot<T: Config> = StorageValue<_, Slot, ValueQuery>;

    // #[pallet::genesis_config]
    // #[derive(frame_support::DefaultNoBound)]
    // pub struct GenesisConfig<T: Config> {
    //     pub authorities: Vec<T::AuthorityId>,
    // }

    // #[pallet::genesis_build]
    // impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
    //     fn build(&self) {
    //         Pallet::<T>::initialize_authorities(&self.authorities);
    //     }
    // }
}

// impl<T: Config> Pallet<T> {
// }

impl<T: Config> sp_runtime::BoundToRuntimeAppPublic for Pallet<T> {
    type Public = T::AuthorityId;
}

impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
	type Key = T::AuthorityId;

	fn on_genesis_session<'a, I: 'a>(validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{
		// let authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
		// Self::initialize_authorities(&authorities);
	}

	fn on_new_session<'a, I: 'a>(changed: bool, validators: I, _queued_validators: I)
	where
		I: Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
	{
		// instant changes
		// if changed {
		// 	let next_authorities = validators.map(|(_, k)| k).collect::<Vec<_>>();
		// 	let last_authorities = Authorities::<T>::get();
		// 	if last_authorities != next_authorities {
		// 		if next_authorities.len() as u32 > T::MaxAuthorities::get() {
		// 			log::warn!(
		// 				target: LOG_TARGET,
		// 				"next authorities list larger than {}, truncating",
		// 				T::MaxAuthorities::get(),
		// 			);
		// 		}
		// 		let bounded = <BoundedVec<_, T::MaxAuthorities>>::truncate_from(next_authorities);
		// 		Self::change_authorities(bounded);
		// 	}
		// }
	}

	fn on_disabled(i: u32) {
		// let log = DigestItem::Consensus(
		// 	AURA_ENGINE_ID,
		// 	ConsensusLog::<T::AuthorityId>::OnDisabled(i as AuthorityIndex).encode(),
		// );

		// <frame_system::Pallet<T>>::deposit_log(log);
	}
}


