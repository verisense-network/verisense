#![cfg_attr(not(feature = "std"), no_std)]

mod constrants;

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use codec::Encode;
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_session::historical as session_historical;
use sp_api::impl_runtime_apis;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    traits::{BlakeTwo256, Block as BlockT, NumberFor, One, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, Vec,
};
use sp_std::prelude::*;

#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use crate::constrants::time::{
    EPOCH_DURATION_IN_BLOCKS, MILLISECS_PER_BLOCK, SESSION_IN_BLOCKS, SESSION_PER_ERA,
    SLOT_DURATION,
};
use crate::constrants::PRIMARY_PROBABILITY;
use frame_support::__private::log;
use frame_support::traits::EnsureOriginWithArg;
pub use frame_support::{
    construct_runtime, derive_impl, parameter_types,
    traits::{
        ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
        StorageInfo,
    },
    weights::{
        constants::{
            BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
        },
        IdentityFee, Weight,
    },
    StorageValue,
};
use frame_support::{
    genesis_builder_helper::{build_state, get_preset},
    traits::VariantCountOf,
    PalletId,
};
pub use frame_system::Call as SystemCall;
use frame_system::EnsureRoot;
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};
use sp_runtime::generic::Era;
use sp_runtime::traits::{ConvertInto, Extrinsic, OpaqueKeys};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{app_crypto, BoundToRuntimeAppPublic, Perbill, Permill};
pub use vrs_primitives::*;

pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
    sp_consensus_babe::BabeEpochConfiguration {
        c: PRIMARY_PROBABILITY,
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub babe: Babe,
            pub grandpa: Grandpa,
            pub authority: AuthorityDiscovery,
            pub restaking: Restaking,
            pub vrf: Nucleus,
            pub im_online: ImOnline,
        }
    }
}

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("verisense"),
    impl_name: create_runtime_str!("verisense"),
    authoring_version: 1,
    // The version of the runtime specification. A full node will not attempt to use its native
    //   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
    //   `spec_version`, and `authoring_version` are the same between Wasm and native.
    // This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
    //   the compatible custom types.
    spec_version: 105,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    pub const Version: RuntimeVersion = VERSION;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::with_sensible_defaults(
            Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
            NORMAL_DISPATCH_RATIO,
        );
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u8 = 137;
    pub const MaxValidators: u32 = 256;
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`SoloChainDefaultConfig`](`struct@frame_system::config_preludes::SolochainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::SolochainDefaultConfig)]
impl frame_system::Config for Runtime {
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The block type for the runtime.
    type Block = Block;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The maximum length of a block (in bytes).
    type BlockLength = BlockLength;
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = BlockWeights;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    /// The type for storing how many extrinsics an account has signed.
    type Nonce = Nonce;
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
    /// Version of the runtime.
    type Version = Version;
}

impl pallet_babe::Config for Runtime {
    type DisabledValidators = Session;
    // session module is the trigger
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type EpochDuration = EpochDuration;
    type EquivocationReportSystem =
        pallet_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
    type ExpectedBlockTime = ExpectedBlockTime;
    type KeyOwnerProof = sp_session::MembershipProof;
    type MaxAuthorities = MaxAuthorities;
    type MaxNominators = ConstU32<0>;
    type WeightInfo = ();
}

impl pallet_grandpa::Config for Runtime {
    type EquivocationReportSystem = ();
    type KeyOwnerProof = sp_core::Void;
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

impl pallet_authority_discovery::Config for Runtime {
    type MaxAuthorities = MaxValidators;
}

impl pallet_timestamp::Config for Runtime {
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Babe;
    type WeightInfo = ();
}

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 600;

impl pallet_balances::Config for Runtime {
    type AccountStore = System;
    /// The type for recording an account's balance.
    type Balance = Balance;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type RuntimeFreezeReason = RuntimeHoldReason;
    type RuntimeHoldReason = RuntimeHoldReason;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
    type LengthToFee = IdentityFee<Balance>;
    type OnChargeTransaction = FungibleAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type RuntimeEvent = RuntimeEvent;
    type WeightToFee = IdentityFee<Balance>;
}

impl pallet_sudo::Config for Runtime {
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
}

pub const NUCLEUS_FEE_COLLECTOR: AccountId = AccountId::new(hex_literal::hex!(
    "36e5fc3abd178f8823ec53a94fb03873779fa85d61f03a95901a4bde1eca1626"
));

use scale_info::prelude::string::String;
parameter_types! {
    pub RegistryDuration: BlockNumber = 10;
    pub const NucleusFeeCollector: AccountId = NUCLEUS_FEE_COLLECTOR;
    pub FeeAssetId: AssetId = AssetId::try_from(String::from("FEE")).unwrap();
}

impl pallet_nucleus::Config for Runtime {
    type AgentRegistry = A2A;
    type Assets = Assets;
    type AuthorityId = vrs_primitives::keys::vrf::AuthorityId;
    type FeeAssetId = FeeAssetId;
    type FeeCollector = NucleusFeeCollector;
    type NodeId = NodeId;
    type NucleusId = NucleusId;
    type RegistryDuration = RegistryDuration;
    type RuntimeEvent = RuntimeEvent;
    type Validators = Validator;
    type Weight = pallet_nucleus::weights::SubstrateWeight<Runtime>;
}

impl pallet_a2a::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Weight = pallet_a2a::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
    type FullIdentification = u128;
    type FullIdentificationOf = pallet_validator::types::ExposureOf<Runtime>;
}

pub struct ValidatorIdOf;
impl sp_runtime::traits::Convert<AccountId, Option<AccountId>> for ValidatorIdOf {
    fn convert(a: AccountId) -> Option<AccountId> {
        Some(a)
    }
}

parameter_types! {
    pub const Period: u32 = SESSION_IN_BLOCKS;
    pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
    type Keys = opaque::SessionKeys;
    type NextSessionRotation = Babe;
    type RuntimeEvent = RuntimeEvent;
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Validator>;
    type ShouldEndSession = Babe;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ConvertInto;
    type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const SessionsPerEra: sp_staking::SessionIndex = SESSION_PER_ERA;
    pub const BondingDuration: u32 = 24 * 21;
    pub const BlocksPerEra: u32 = SESSION_IN_BLOCKS * SessionsPerEra::get();
    pub const HistoryDepth: u32 = 100;
}

impl pallet_validator::Config for Runtime {
    type BondingDuration = BondingDuration;
    type FullIdentification = u128;
    type FullIdentificationOf = pallet_validator::types::ExposureOf<Runtime>;
    type HistoryDepth = HistoryDepth;
    type RestakingInterface = Restaking;
    type RuntimeEvent = RuntimeEvent;
    type SessionInterface = Self;
    type SessionsPerEra = SessionsPerEra;
    type UnixTime = Timestamp;
    type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
    type EventHandler = (Validator, ImOnline);
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
}

pub struct VerisenseRestakingAppCrypto;

impl frame_system::offchain::AppCrypto<<Signature as Verify>::Signer, Signature>
    for VerisenseRestakingAppCrypto
{
    type GenericPublic = sp_core::sr25519::Public;
    type GenericSignature = sp_core::sr25519::Signature;
    type RuntimeAppPublic = vrs_primitives::keys::restaking::AuthorityId;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    RuntimeCall: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = RuntimeCall;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    RuntimeCall: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: RuntimeCall,
        public: <Signature as Verify>::Signer,
        account: AccountId,
        nonce: u32,
    ) -> Option<(
        RuntimeCall,
        <UncheckedExtrinsic as Extrinsic>::SignaturePayload,
    )> {
        use sp_runtime::traits::StaticLookup;
        use sp_runtime::SaturatedConversion;
        let tip = 0;
        // take the biggest period possible.
        let period = BlockHashCount::get()
            .checked_next_power_of_two()
            .map(|c| c / 2)
            .unwrap_or(2) as u64;
        let current_block = System::block_number()
            .saturated_into::<u64>()
            // The `System::block_number` is initialized with `n+1`,
            // so the actual block number is `n`.
            .saturating_sub(1);
        let era = Era::mortal(period, current_block);
        let extra = (
            frame_system::CheckSpecVersion::<Runtime>::new(),
            frame_system::CheckTxVersion::<Runtime>::new(),
            pallet_nucleus::check_nonce::CheckNonce::<Runtime>::from(nonce),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckMortality::<Runtime>::from(era),
            frame_system::CheckWeight::<Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
        );
        let raw_payload = SignedPayload::new(call, extra)
            .map_err(|e| {
                log::warn!("Unable to create signed payload: {:?}", e);
            })
            .ok()?;
        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
        let address = <Runtime as frame_system::Config>::Lookup::unlookup(account); //TODO Self to Runtime
        let (call, extra, _) = raw_payload.deconstruct();
        Some((call, (address, signature.into(), extra)))
    }
}

parameter_types! {
    pub const MaxAuthorities: u32 = 100;
    pub const MaxKeys: u32 = 10_000;
    pub const MaxPeerInHeartbeats: u32 = 10_000;
    pub const MaxPeerDataEncodingSize: u32 = 1_000;
    pub const RequestEventLimit: u32 = 10;
    pub const RestakingUnsignedPriority: u64 = 1 << 21;

    pub const ImOnlineUnsignedPriority: u64 = 1 << 22;
    pub const RestakingEnable: bool = true;
}

parameter_types! {
    // NOTE: Currently it is not possible to change the epoch duration after the chain has started.
    //       Attempting to do so will brick block production.
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_BLOCKS as u64;
    pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK;
    pub const ReportLongevity: u64 =
        BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
}

impl pallet_restaking::Config for Runtime {
    type AppCrypto = VerisenseRestakingAppCrypto;
    type AuthorityId = vrs_primitives::keys::restaking::AuthorityId;
    type Currency = Balances;
    type MaxValidators = MaxAuthorities;
    type RequestEventLimit = RequestEventLimit;
    type RestakingEnable = RestakingEnable;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type UnixTime = Timestamp;
    type UnsignedPriority = RestakingUnsignedPriority;
    type ValidatorsInterface = Validator;
}

pub struct NoAssetCreators;
impl EnsureOriginWithArg<RuntimeOrigin, AssetId> for NoAssetCreators {
    type Success = AccountId;

    fn try_origin(o: RuntimeOrigin, _a: &AssetId) -> Result<Self::Success, RuntimeOrigin> {
        Err(o)
    }
}

impl pallet_im_online::Config for Runtime {
    type AuthorityId = pallet_im_online::sr25519::AuthorityId;
    type MaxKeys = MaxKeys;
    type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
    type NextSessionRotation = Babe;
    type ReportUnresponsiveness = Offences;
    type RuntimeEvent = RuntimeEvent;
    type UnsignedPriority = ImOnlineUnsignedPriority;
    type ValidatorSet = Historical;
    type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
}

impl pallet_assets::Config for Runtime {
    type ApprovalDeposit = ConstU128<0>;
    type AssetAccountDeposit = ConstU128<0>;
    type AssetDeposit = ConstU128<0>;
    type AssetId = AssetId;
    type AssetIdParameter = AssetId;
    type Balance = Balance;
    type CallbackHandle = ();
    type CreateOrigin = NoAssetCreators;
    type Currency = Balances;
    type Extra = ();
    type ForceOrigin = EnsureRoot<AccountId>;
    type Freezer = ();
    type MetadataDepositBase = ConstU128<0>;
    type MetadataDepositPerByte = ConstU128<0>;
    type RemoveItemsLimit = ConstU32<5>;
    type RuntimeEvent = RuntimeEvent;
    type StringLimit = ConstU32<50>;
    type WeightInfo = ();
}

impl pallet_offences::Config for Runtime {
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = ();
    type RuntimeEvent = RuntimeEvent;
}

parameter_types! {
    pub const SwapPalletId: PalletId = pallet_swap::PALLET_ID;

}
impl pallet_swap::Config for Runtime {
    type AssetBalance = Balance;
    type AssetId = AssetId;
    type AssetRegistry = Assets;
    type AssetToCurrencyBalance = ConvertInto;
    type Assets = Assets;
    type Currency = Balances;
    type CurrencyToAssetBalance = ConvertInto;
    type MinDeposit = ConstU128<1>;
    type PalletId = SwapPalletId;
    type ProviderFeeDenominator = ConstU128<100>;
    type ProviderFeeNumerator = ConstU128<1>;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

// Create the runtime by composing the FRAME pallets that were previously configured.
#[frame_support::runtime]
mod runtime {
    #[runtime::runtime]
    #[runtime::derive(
        RuntimeCall,
        RuntimeEvent,
        RuntimeError,
        RuntimeOrigin,
        RuntimeFreezeReason,
        RuntimeHoldReason,
        RuntimeSlashReason,
        RuntimeLockId,
        RuntimeTask
    )]
    pub struct Runtime;

    #[runtime::pallet_index(0)]
    pub type System = frame_system;

    #[runtime::pallet_index(1)]
    pub type Timestamp = pallet_timestamp;

    #[runtime::pallet_index(2)]
    pub type Babe = pallet_babe;

    #[runtime::pallet_index(3)]
    pub type Authorship = pallet_authorship;

    #[runtime::pallet_index(4)]
    pub type Restaking = pallet_restaking;

    #[runtime::pallet_index(5)]
    pub type AuthorityDiscovery = pallet_authority_discovery;

    #[runtime::pallet_index(6)]
    pub type Validator = pallet_validator;

    #[runtime::pallet_index(7)]
    pub type Session = pallet_session;

    #[runtime::pallet_index(8)]
    pub type Grandpa = pallet_grandpa;

    #[runtime::pallet_index(9)]
    pub type Historical = session_historical;

    #[runtime::pallet_index(10)]
    pub type ImOnline = pallet_im_online;

    #[runtime::pallet_index(11)]
    pub type Balances = pallet_balances;

    #[runtime::pallet_index(12)]
    pub type TransactionPayment = pallet_transaction_payment;

    #[runtime::pallet_index(13)]
    pub type Sudo = pallet_sudo;

    #[runtime::pallet_index(14)]
    pub type Offences = pallet_offences;

    #[runtime::pallet_index(15)]
    pub type Nucleus = pallet_nucleus;

    #[runtime::pallet_index(16)]
    pub type Assets = pallet_assets;

    #[runtime::pallet_index(17)]
    pub type Swap = pallet_swap;

    #[runtime::pallet_index(18)]
    pub type A2A = pallet_a2a;
}

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    pallet_nucleus::check_nonce::CheckNonce<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckMortality<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// All migrations of the runtime, aside from the ones declared in the pallets.
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
    Migrations,
>;
use pallet_swap::rpc::RpcResult;
impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block);
        }

        fn initialize_block(header: &<Block as BlockT>::Header) -> sp_runtime::ExtrinsicInclusionMode {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            OpaqueMetadata::new(Runtime::metadata().into())
        }

        fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
            Runtime::metadata_at_version(version)
        }

        fn metadata_versions() -> sp_std::vec::Vec<u32> {
            Runtime::metadata_versions()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
            block_hash: <Block as BlockT>::Hash,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx, block_hash)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeConfiguration {
            let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
            sp_consensus_babe::BabeConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDuration::get().into(),
                c: epoch_config.c,
                authorities: Babe::authorities().to_vec(),
                randomness: Babe::randomness(),
                allowed_slots: epoch_config.allowed_slots,
            }
        }

        fn current_epoch_start() -> sp_consensus_babe::Slot {
            Babe::current_epoch_start()
        }

        fn current_epoch() -> sp_consensus_babe::Epoch {
            Babe::current_epoch()
        }

        fn next_epoch() -> sp_consensus_babe::Epoch {
            Babe::next_epoch()
        }

        fn generate_key_ownership_proof(
            _slot: sp_consensus_babe::Slot,
            authority_id: sp_consensus_babe::AuthorityId,
        ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
            use codec::Encode;

            Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
            key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Babe::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
            )
        }
    }

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl sp_consensus_grandpa::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn current_set_id() -> sp_consensus_grandpa::SetId {
            Grandpa::current_set_id()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: sp_consensus_grandpa::SetId,
            _authority_id: GrandpaId,
        ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
        fn account_nonce(account: AccountId) -> Nonce {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
        for Runtime
    {
        fn query_call_info(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_call_info(call, len)
        }
        fn query_call_fee_details(
            call: RuntimeCall,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_call_fee_details(call, len)
        }
        fn query_weight_to_fee(weight: Weight) -> Balance {
            TransactionPayment::weight_to_fee(weight)
        }
        fn query_length_to_fee(length: u32) -> Balance {
            TransactionPayment::length_to_fee(length)
        }
    }

    impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
        fn authorities() -> Vec<AuthorityDiscoveryId> {
            AuthorityDiscovery::authorities()
        }
    }

    impl vrs_validator_runtime_api::ValidatorRuntimeApi<Block> for Runtime {
        fn lookup_active_validator(id: KeyTypeId, key_data: Vec<u8>) -> Option<AccountId> {
            <Validator as vrs_support::ValidatorsInterface<AccountId>>::lookup_active_validator(id, key_data.as_ref())
        }

        fn get_genesis_validators() -> Vec<AccountId> {
            Validator::genesis_validators()
        }

        fn get_current_validators() -> Vec<AccountId> {
            <Validator as vrs_support::ValidatorsInterface<AccountId>>::validators()
        }
    }

    #[api_version(2)]
    impl vrs_nucleus_runtime_api::NucleusRuntimeApi<Block> for Runtime {
        fn resolve_deploy_tx(uxt: <Block as BlockT>::Extrinsic) -> Option<vrs_nucleus_runtime_api::NucleusUpgradingTxInfo> {
            if let RuntimeCall::Nucleus(pallet_nucleus::Call::upload_nucleus_wasm {
                nucleus_id,
                to,
                hash,
                agent_card,
            }) = uxt.function {
                Some(vrs_nucleus_runtime_api::NucleusUpgradingTxInfo {
                    nucleus_id,
                    wasm_hash: hash,
                    node_id: to,
                    agent_card,
                })
            } else {
                None
            }
        }

        fn get_nucleus_info(nucleus_id: &NucleusId) -> Option<NucleusInfo<AccountId, Hash, NodeId>> {
            Nucleus::get_nucleus_info(nucleus_id)
        }

        fn is_member_of(
            nucleus_id: &NucleusId,
            account_id: &AccountId,
        ) -> bool {
            Nucleus::get_members(nucleus_id).contains(account_id)
        }
    }

    impl vrs_a2a_runtime_api::A2aRuntimeApi<Block> for Runtime {
        fn find_agent(
            agent_id: AccountId,
        ) -> Option<a2a_rs::AgentInfo<AccountId>> {
            <A2A as vrs_support::AgentRegistry<AccountId>>::find_agent(&agent_id)
        }

        fn get_all_agents() -> Vec<a2a_rs::AgentInfo<AccountId>> {
            A2A::get_all_agents()
        }
    }

    impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
        fn build_state(config: Vec<u8>) -> sp_genesis_builder::Result {
            build_state::<RuntimeGenesisConfig>(config)
        }

        fn get_preset(id: &Option<sp_genesis_builder::PresetId>) -> Option<Vec<u8>> {
            get_preset::<RuntimeGenesisConfig>(id, |_| None)
        }

        fn preset_names() -> Vec<sp_genesis_builder::PresetId> {
            vec![]
        }
    }

    impl vrs_tss_runtime_api::VrsTssRuntimeApi<Block> for Runtime {
        fn get_all_validators() -> Vec<sp_runtime::AccountId32> {
            Session::validators().to_vec()
        }
    }

    impl vrs_swap_runtime_api::SwapApi<Block, AssetId, Balance, Balance> for Runtime {

        fn get_currency_to_asset_output_amount(asset_id: AssetId, currency_amount: Balance) -> RpcResult<Balance> {
            Swap::get_currency_to_asset_output_amount(asset_id, currency_amount)
        }
        fn get_currency_to_asset_input_amount(asset_id: AssetId, token_amount: Balance) -> RpcResult<Balance>{
            Swap::get_currency_to_asset_input_amount(asset_id, token_amount)
        }
        fn get_asset_to_currency_output_amount(asset_id: AssetId, token_amount: Balance) -> RpcResult<Balance> {
            Swap::get_asset_to_currency_output_amount(asset_id, token_amount)
        }
        fn get_asset_to_currency_input_amount(asset_id: AssetId, currency_amount: Balance) -> RpcResult<Balance> {
            Swap::get_asset_to_currency_input_amount(asset_id, currency_amount)
        }
    }

}
