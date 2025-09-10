use crate::migrations::v0;
use crate::McpServerInfo;
use frame_support::traits::UncheckedOnRuntimeUpgrade;
use sp_core::Get;

pub struct InnerMigrateV0ToV1<T: crate::Config>(core::marker::PhantomData<T>);

impl<T: crate::Config> UncheckedOnRuntimeUpgrade for InnerMigrateV0ToV1<T> {
    fn on_runtime_upgrade() -> frame_support::weights::Weight {
        let mut times = 1;
        for x in v0::Servers::<T>::iter() {
            let new_agent_info = McpServerInfo {
                name: x.1.name,
                description: x.1.description,
                url: x.1.url,
                url_verified: true,
                provider: x.1.provider,
                price_rate: 100,
                logo: None,
                provider_website: None,
                provider_name: None,
            };
            crate::Servers::<T>::insert(x.0, new_agent_info);
            times += 1;
        }
        T::DbWeight::get().reads_writes(times, 2 * times)
    }
}

/// [`UncheckedOnRuntimeUpgrade`] implementation [`InnerMigrateV0ToV1`] wrapped in a
/// [`VersionedMigration`](frame_support::migrations::VersionedMigration), which ensures that:
/// - The migration only runs once when the on-chain storage version is 0
/// - The on-chain storage version is updated to `1` after the migration executes
/// - Reads/Writes from checking/settings the on-chain storage version are accounted for
pub type MigrateV0ToV1<T> = frame_support::migrations::VersionedMigration<
    0, // The migration will only execute when the on-chain storage version is 0
    1, // The on-chain storage version will be set to 1 after the migration is complete
    InnerMigrateV0ToV1<T>,
    crate::pallet::Pallet<T>,
    <T as frame_system::Config>::DbWeight,
>;
