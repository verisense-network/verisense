
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_template.
pub trait WeightInfo {
    fn do_something() -> Weight;
    fn cause_error() -> Weight;
}

/// Weights for pallet_template using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn do_something() -> Weight {
        Weight::from_parts(9_000_000, 0)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn cause_error() -> Weight {
        Weight::from_parts(6_000_000, 1489)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

impl WeightInfo for () {
    fn do_something() -> Weight {
        Weight::from_parts(9_000_000, 0)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn cause_error() -> Weight {
        Weight::from_parts(6_000_000, 1489)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}
