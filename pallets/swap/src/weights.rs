
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_swap`.
pub trait WeightInfo {
    fn create_exchange() -> Weight;
    fn add_liquidity() -> Weight;
    fn remove_liquidity() -> Weight;
    fn currency_to_asset() -> Weight;
    fn asset_to_currency() -> Weight;
    fn asset_to_asset() -> Weight;
}

/// Weight functions for `pallet_dex`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T>  {
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:3 w:3)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    fn create_exchange() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `393`
        //  Estimated: `22191`
        // Minimum execution time: 112_544_000 picoseconds.
        Weight::from_parts(130_557_000, 0)
            .saturating_add(Weight::from_parts(0, 22191))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(7))
    }
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:3 w:3)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn add_liquidity() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `969`
        //  Estimated: `22191`
        // Minimum execution time: 98_087_000 picoseconds.
        Weight::from_parts(100_291_000, 0)
            .saturating_add(Weight::from_parts(0, 22191))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(7))
    }
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:3 w:3)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn remove_liquidity() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `969`
        //  Estimated: `22191`
        // Minimum execution time: 104_010_000 picoseconds.
        Weight::from_parts(106_517_000, 0)
            .saturating_add(Weight::from_parts(0, 22191))
            .saturating_add(T::DbWeight::get().reads(7))
            .saturating_add(T::DbWeight::get().writes(7))
    }
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:1 w:1)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:2 w:2)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    fn currency_to_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `726`
        //  Estimated: `16929`
        // Minimum execution time: 76_673_000 picoseconds.
        Weight::from_parts(78_526_000, 0)
            .saturating_add(Weight::from_parts(0, 16929))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:1 w:1)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:2 w:2)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn asset_to_currency() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `726`
        //  Estimated: `16929`
        // Minimum execution time: 77_788_000 picoseconds.
        Weight::from_parts(79_399_000, 0)
            .saturating_add(Weight::from_parts(0, 16929))
            .saturating_add(T::DbWeight::get().reads(5))
            .saturating_add(T::DbWeight::get().writes(5))
    }
    /// Storage: Dex Exchanges (r:2 w:2)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:4 w:4)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    fn asset_to_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1146`
        //  Estimated: `23702`
        // Minimum execution time: 106_608_000 picoseconds.
        Weight::from_parts(109_049_000, 0)
            .saturating_add(Weight::from_parts(0, 23702))
            .saturating_add(T::DbWeight::get().reads(8))
            .saturating_add(T::DbWeight::get().writes(8))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:3 w:3)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    fn create_exchange() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `393`
        //  Estimated: `22191`
        // Minimum execution time: 112_544_000 picoseconds.
        Weight::from_parts(130_557_000, 0)
            .saturating_add(Weight::from_parts(0, 22191))
            .saturating_add(RocksDbWeight::get().reads(7))
            .saturating_add(RocksDbWeight::get().writes(7))
    }
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:3 w:3)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn add_liquidity() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `969`
        //  Estimated: `22191`
        // Minimum execution time: 98_087_000 picoseconds.
        Weight::from_parts(100_291_000, 0)
            .saturating_add(Weight::from_parts(0, 22191))
            .saturating_add(RocksDbWeight::get().reads(7))
            .saturating_add(RocksDbWeight::get().writes(7))
    }
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:3 w:3)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn remove_liquidity() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `969`
        //  Estimated: `22191`
        // Minimum execution time: 104_010_000 picoseconds.
        Weight::from_parts(106_517_000, 0)
            .saturating_add(Weight::from_parts(0, 22191))
            .saturating_add(RocksDbWeight::get().reads(7))
            .saturating_add(RocksDbWeight::get().writes(7))
    }
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:1 w:1)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:2 w:2)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    fn currency_to_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `726`
        //  Estimated: `16929`
        // Minimum execution time: 76_673_000 picoseconds.
        Weight::from_parts(78_526_000, 0)
            .saturating_add(Weight::from_parts(0, 16929))
            .saturating_add(RocksDbWeight::get().reads(5))
            .saturating_add(RocksDbWeight::get().writes(5))
    }
    /// Storage: Dex Exchanges (r:1 w:1)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:1 w:1)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:2 w:2)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn asset_to_currency() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `726`
        //  Estimated: `16929`
        // Minimum execution time: 77_788_000 picoseconds.
        Weight::from_parts(79_399_000, 0)
            .saturating_add(Weight::from_parts(0, 16929))
            .saturating_add(RocksDbWeight::get().reads(5))
            .saturating_add(RocksDbWeight::get().writes(5))
    }
    /// Storage: Dex Exchanges (r:2 w:2)
    /// Proof: Dex Exchanges (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    /// Storage: Assets Asset (r:2 w:2)
    /// Proof: Assets Asset (max_values: None, max_size: Some(210), added: 2685, mode: MaxEncodedLen)
    /// Storage: Assets Account (r:4 w:4)
    /// Proof: Assets Account (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
    fn asset_to_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1146`
        //  Estimated: `23702`
        // Minimum execution time: 106_608_000 picoseconds.
        Weight::from_parts(109_049_000, 0)
            .saturating_add(Weight::from_parts(0, 23702))
            .saturating_add(RocksDbWeight::get().reads(8))
            .saturating_add(RocksDbWeight::get().writes(8))
    }
}
