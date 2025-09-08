use crate::migrations::v0;
use a2a_rs::AgentInfo;
use frame_support::traits::UncheckedOnRuntimeUpgrade;
use sp_core::Get;

pub struct InnerMigrateV0ToV1<T: crate::Config>(core::marker::PhantomData<T>);

impl<T: crate::Config> UncheckedOnRuntimeUpgrade for InnerMigrateV0ToV1<T> {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        let mut times = 1;
        let now = frame_system::Pallet::<T>::block_number().into().as_u32();
        for x in v0::Agents::<T>::iter() {
            let new_agent_info = AgentInfo {
                agent_id: x.1.agent_id,
                owner_id: x.1.owner_id,
                url_verified: true,
                price_rate: 100,
                agent_card: x.1.agent_card,
            };
            crate::Agents::<T>::insert(x.0.clone(), new_agent_info.clone());
            times += 1;
        }
        T::DbWeight::get().reads_writes(times, 2 * times)
    }
}

pub type MigrateV0ToV1<T> = frame_support::migrations::VersionedMigration<
    0,
    1,
    InnerMigrateV0ToV1<T>,
    crate::pallet::Pallet<T>,
    <T as frame_system::Config>::DbWeight,
>;
