#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod codegen {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 19usize] = [
        "System",
        "Timestamp",
        "Babe",
        "Authorship",
        "Restaking",
        "AuthorityDiscovery",
        "Validator",
        "Session",
        "Grandpa",
        "Historical",
        "ImOnline",
        "Balances",
        "TransactionPayment",
        "Sudo",
        "Offences",
        "Nucleus",
        "Assets",
        "Swap",
        "A2A",
    ];
    pub static RUNTIME_APIS: [&str; 0usize] = [];
    #[doc = r" The error type returned when there is a runtime issue."]
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    #[doc = r" The outer event enum."]
    pub type Event = runtime_types::vrs_runtime::RuntimeEvent;
    #[doc = r" The outer extrinsic enum."]
    pub type Call = runtime_types::vrs_runtime::RuntimeCall;
    #[doc = r" The outer error enum representing the DispatchError's Module variant."]
    pub type Error = runtime_types::vrs_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt_core::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {}
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi {
            babe::constants::ConstantsApi
        }
        pub fn restaking(&self) -> restaking::constants::ConstantsApi {
            restaking::constants::ConstantsApi
        }
        pub fn validator(&self) -> validator::constants::ConstantsApi {
            validator::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi {
            im_online::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn nucleus(&self) -> nucleus::constants::ConstantsApi {
            nucleus::constants::ConstantsApi
        }
        pub fn assets(&self) -> assets::constants::ConstantsApi {
            assets::constants::ConstantsApi
        }
        pub fn swap(&self) -> swap::constants::ConstantsApi {
            swap::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }
        pub fn babe(&self) -> babe::storage::StorageApi {
            babe::storage::StorageApi
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi {
            authorship::storage::StorageApi
        }
        pub fn restaking(&self) -> restaking::storage::StorageApi {
            restaking::storage::StorageApi
        }
        pub fn authority_discovery(&self) -> authority_discovery::storage::StorageApi {
            authority_discovery::storage::StorageApi
        }
        pub fn validator(&self) -> validator::storage::StorageApi {
            validator::storage::StorageApi
        }
        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }
        pub fn historical(&self) -> historical::storage::StorageApi {
            historical::storage::StorageApi
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi {
            im_online::storage::StorageApi
        }
        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }
        pub fn offences(&self) -> offences::storage::StorageApi {
            offences::storage::StorageApi
        }
        pub fn nucleus(&self) -> nucleus::storage::StorageApi {
            nucleus::storage::StorageApi
        }
        pub fn assets(&self) -> assets::storage::StorageApi {
            assets::storage::StorageApi
        }
        pub fn swap(&self) -> swap::storage::StorageApi {
            swap::storage::StorageApi
        }
        pub fn a2a(&self) -> a2a::storage::StorageApi {
            a2a::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }
        pub fn babe(&self) -> babe::calls::TransactionApi {
            babe::calls::TransactionApi
        }
        pub fn restaking(&self) -> restaking::calls::TransactionApi {
            restaking::calls::TransactionApi
        }
        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi {
            im_online::calls::TransactionApi
        }
        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }
        pub fn nucleus(&self) -> nucleus::calls::TransactionApi {
            nucleus::calls::TransactionApi
        }
        pub fn assets(&self) -> assets::calls::TransactionApi {
            assets::calls::TransactionApi
        }
        pub fn swap(&self) -> swap::calls::TransactionApi {
            swap::calls::TransactionApi
        }
        pub fn a2a(&self) -> a2a::calls::TransactionApi {
            a2a::calls::TransactionApi
        }
    }
    #[doc = r" check whether the metadata provided is aligned with this statically generated code."]
    pub fn is_codegen_valid_for(metadata: &::subxt_core::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                215u8, 34u8, 1u8, 190u8, 138u8, 42u8, 31u8, 223u8, 52u8, 0u8, 170u8, 172u8, 150u8,
                244u8, 110u8, 120u8, 225u8, 25u8, 222u8, 43u8, 209u8, 62u8, 54u8, 35u8, 103u8,
                38u8, 110u8, 101u8, 121u8, 172u8, 201u8, 121u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::frame_system::pallet::Error;
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Remark {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetHeapPages {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_heap_pages";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetCode {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_code_without_checks";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::subxt_core::alloc::vec::Vec<(
                        ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "set_storage";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::subxt_core::alloc::vec::Vec<
                        ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    >;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for KillStorage {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_storage";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for KillPrefix {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "kill_prefix";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for RemarkWithEvent {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "remark_with_event";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AuthorizeUpgrade {
                    pub code_hash: authorize_upgrade::CodeHash,
                }
                pub mod authorize_upgrade {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt_core::utils::H256;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AuthorizeUpgradeWithoutChecks {
                    pub code_hash: authorize_upgrade_without_checks::CodeHash,
                }
                pub mod authorize_upgrade_without_checks {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt_core::utils::H256;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for AuthorizeUpgradeWithoutChecks {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "authorize_upgrade_without_checks";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ApplyAuthorizedUpgrade {
                    pub code: apply_authorized_upgrade::Code,
                }
                pub mod apply_authorized_upgrade {
                    use super::runtime_types;
                    pub type Code = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ApplyAuthorizedUpgrade {
                    const PALLET: &'static str = "System";
                    const CALL: &'static str = "apply_authorized_upgrade";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Remark> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark",
                        types::Remark { remark },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetHeapPages> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages { pages },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetCode> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code",
                        types::SetCode { code },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetCodeWithoutChecks>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks { code },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetStorage> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage { items },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::KillStorage> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage { keys },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::KillPrefix> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix { prefix, subkeys },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::RemarkWithEvent>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent { remark },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
                pub fn authorize_upgrade(
                    &self,
                    code_hash: types::authorize_upgrade::CodeHash,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgrade>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade",
                        types::AuthorizeUpgrade { code_hash },
                        [
                            4u8, 14u8, 76u8, 107u8, 209u8, 129u8, 9u8, 39u8, 193u8, 17u8, 84u8,
                            254u8, 170u8, 214u8, 24u8, 155u8, 29u8, 184u8, 249u8, 241u8, 109u8,
                            58u8, 145u8, 131u8, 109u8, 63u8, 38u8, 165u8, 107u8, 215u8, 217u8,
                            172u8,
                        ],
                    )
                }
                pub fn authorize_upgrade_without_checks(
                    &self,
                    code_hash: types::authorize_upgrade_without_checks::CodeHash,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::AuthorizeUpgradeWithoutChecks>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "authorize_upgrade_without_checks",
                        types::AuthorizeUpgradeWithoutChecks { code_hash },
                        [
                            126u8, 126u8, 55u8, 26u8, 47u8, 55u8, 66u8, 8u8, 167u8, 18u8, 29u8,
                            136u8, 146u8, 14u8, 189u8, 117u8, 16u8, 227u8, 162u8, 61u8, 149u8,
                            197u8, 104u8, 184u8, 185u8, 161u8, 99u8, 154u8, 80u8, 125u8, 181u8,
                            233u8,
                        ],
                    )
                }
                pub fn apply_authorized_upgrade(
                    &self,
                    code: types::apply_authorized_upgrade::Code,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ApplyAuthorizedUpgrade>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "System",
                        "apply_authorized_upgrade",
                        types::ApplyAuthorizedUpgrade { code },
                        [
                            232u8, 107u8, 127u8, 38u8, 230u8, 29u8, 97u8, 4u8, 160u8, 191u8, 222u8,
                            156u8, 245u8, 102u8, 196u8, 141u8, 44u8, 163u8, 98u8, 68u8, 125u8,
                            32u8, 124u8, 101u8, 108u8, 93u8, 211u8, 52u8, 0u8, 231u8, 33u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt_core::events::StaticEvent for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt_core::events::StaticEvent for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpdated;
            impl ::subxt_core::events::StaticEvent for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt_core::utils::AccountId32;
                pub type Hash = ::subxt_core::utils::H256;
            }
            impl ::subxt_core::events::StaticEvent for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct UpgradeAuthorized {
                pub code_hash: upgrade_authorized::CodeHash,
                pub check_version: upgrade_authorized::CheckVersion,
            }
            pub mod upgrade_authorized {
                use super::runtime_types;
                pub type CodeHash = ::subxt_core::utils::H256;
                pub type CheckVersion = ::core::primitive::bool;
            }
            impl ::subxt_core::events::StaticEvent for UpgradeAuthorized {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod inherents_applied {
                    use super::runtime_types;
                    pub type InherentsApplied = ::core::primitive::bool;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt_core::utils::H256;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u32;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt_core::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::subxt_core::alloc::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::vrs_runtime::RuntimeEvent,
                            ::subxt_core::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics = ::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>;
                    pub type Param0 = ::subxt_core::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
                pub mod authorized_upgrade {
                    use super::runtime_types;
                    pub type AuthorizedUpgrade =
                        runtime_types::frame_system::CodeUpgradeAuthorization;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn account_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        (),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::account::Param0>,
                    types::account::Account,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Account",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicCount",
                        (),
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }
                pub fn inherents_applied(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inherents_applied::InherentsApplied,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "InherentsApplied",
                        (),
                        [
                            132u8, 249u8, 142u8, 252u8, 8u8, 103u8, 80u8, 120u8, 50u8, 6u8, 188u8,
                            223u8, 101u8, 55u8, 165u8, 189u8, 172u8, 249u8, 165u8, 230u8, 183u8,
                            109u8, 34u8, 65u8, 185u8, 150u8, 29u8, 8u8, 186u8, 129u8, 135u8, 239u8,
                        ],
                    )
                }
                pub fn block_weight(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_weight::BlockWeight,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockWeight",
                        (),
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        (),
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::block_hash::BlockHash,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        (),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                pub fn block_hash(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::block_hash::Param0>,
                    types::block_hash::BlockHash,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "BlockHash",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        (),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::extrinsic_data::Param0>,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExtrinsicData",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }
                pub fn number(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::number::Number,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Number",
                        (),
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }
                pub fn parent_hash(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::parent_hash::ParentHash,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ParentHash",
                        (),
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }
                pub fn digest(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::digest::Digest,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Digest",
                        (),
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }
                pub fn events(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::events::Events,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "Events",
                        (),
                        [
                            248u8, 227u8, 6u8, 96u8, 143u8, 154u8, 232u8, 63u8, 202u8, 227u8, 40u8,
                            43u8, 166u8, 247u8, 203u8, 23u8, 249u8, 119u8, 28u8, 83u8, 25u8, 178u8,
                            254u8, 176u8, 233u8, 4u8, 101u8, 155u8, 150u8, 123u8, 36u8, 250u8,
                        ],
                    )
                }
                pub fn event_count(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_count::EventCount,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventCount",
                        (),
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::event_topics::EventTopics,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        (),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                pub fn event_topics(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::event_topics::Param0>,
                    types::event_topics::EventTopics,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "EventTopics",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        (),
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        (),
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        (),
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }
                pub fn execution_phase(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::execution_phase::ExecutionPhase,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "ExecutionPhase",
                        (),
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
                pub fn authorized_upgrade(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorized_upgrade::AuthorizedUpgrade,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "System",
                        "AuthorizedUpgrade",
                        (),
                        [
                            165u8, 97u8, 27u8, 138u8, 2u8, 28u8, 55u8, 92u8, 96u8, 96u8, 168u8,
                            169u8, 55u8, 178u8, 44u8, 127u8, 58u8, 140u8, 206u8, 178u8, 1u8, 37u8,
                            214u8, 213u8, 251u8, 123u8, 5u8, 111u8, 90u8, 148u8, 217u8, 135u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockWeights,
                > {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }
                pub fn block_length(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_system::limits::BlockLength,
                > {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn db_weight(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_weights::RuntimeDbWeight,
                > {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }
                pub fn version(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<
                    runtime_types::sp_version::RuntimeVersion,
                > {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u16>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Set {
                    #[codec(compact)]
                    pub now: set::Now,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Set {
                    const PALLET: &'static str = "Timestamp";
                    const CALL: &'static str = "set";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set(
                    &self,
                    now: types::set::Now,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Set> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Timestamp",
                        "set",
                        types::Set { now },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod now {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                pub mod did_update {
                    use super::runtime_types;
                    pub type DidUpdate = ::core::primitive::bool;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn now(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::now::Now,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "Now",
                        (),
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }
                pub fn did_update(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::did_update::DidUpdate,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Timestamp",
                        "DidUpdate",
                        (),
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod babe {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_babe::pallet::Error;
        pub type Call = runtime_types::pallet_babe::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ReportEquivocation {
                    pub equivocation_proof:
                        ::subxt_core::alloc::boxed::Box<report_equivocation::EquivocationProof>,
                    pub key_owner_proof: report_equivocation::KeyOwnerProof,
                }
                pub mod report_equivocation {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_slots::EquivocationProof<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                            >,
                            runtime_types::sp_consensus_babe::app::Public,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_session::MembershipProof;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ReportEquivocation {
                    const PALLET: &'static str = "Babe";
                    const CALL: &'static str = "report_equivocation";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof: ::subxt_core::alloc::boxed::Box<
                        report_equivocation_unsigned::EquivocationProof,
                    >,
                    pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
                }
                pub mod report_equivocation_unsigned {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_slots::EquivocationProof<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                            >,
                            runtime_types::sp_consensus_babe::app::Public,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_session::MembershipProof;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const PALLET: &'static str = "Babe";
                    const CALL: &'static str = "report_equivocation_unsigned";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct PlanConfigChange {
                    pub config: plan_config_change::Config,
                }
                pub mod plan_config_change {
                    use super::runtime_types;
                    pub type Config =
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for PlanConfigChange {
                    const PALLET: &'static str = "Babe";
                    const CALL: &'static str = "plan_config_change";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: types::report_equivocation::EquivocationProof,
                    key_owner_proof: types::report_equivocation::KeyOwnerProof,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ReportEquivocation>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Babe",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            97u8, 65u8, 136u8, 207u8, 137u8, 113u8, 206u8, 76u8, 166u8, 245u8,
                            231u8, 162u8, 65u8, 47u8, 251u8, 149u8, 68u8, 179u8, 13u8, 123u8,
                            209u8, 146u8, 83u8, 54u8, 14u8, 150u8, 62u8, 195u8, 27u8, 190u8, 76u8,
                            224u8,
                        ],
                    )
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: types::report_equivocation_unsigned::EquivocationProof,
                    key_owner_proof: types::report_equivocation_unsigned::KeyOwnerProof,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ReportEquivocationUnsigned>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Babe",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            184u8, 158u8, 14u8, 168u8, 175u8, 23u8, 10u8, 63u8, 54u8, 15u8, 182u8,
                            163u8, 5u8, 49u8, 223u8, 197u8, 45u8, 204u8, 216u8, 26u8, 126u8, 157u8,
                            242u8, 233u8, 228u8, 203u8, 117u8, 216u8, 185u8, 157u8, 199u8, 117u8,
                        ],
                    )
                }
                pub fn plan_config_change(
                    &self,
                    config: types::plan_config_change::Config,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::PlanConfigChange>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Babe",
                        "plan_config_change",
                        types::PlanConfigChange { config },
                        [
                            227u8, 155u8, 182u8, 231u8, 240u8, 107u8, 30u8, 22u8, 15u8, 52u8,
                            172u8, 203u8, 115u8, 47u8, 6u8, 66u8, 170u8, 231u8, 186u8, 77u8, 19u8,
                            235u8, 91u8, 136u8, 95u8, 149u8, 188u8, 163u8, 161u8, 109u8, 164u8,
                            179u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod epoch_index {
                    use super::runtime_types;
                    pub type EpochIndex = ::core::primitive::u64;
                }
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_babe::app::Public,
                            ::core::primitive::u64,
                        )>;
                }
                pub mod genesis_slot {
                    use super::runtime_types;
                    pub type GenesisSlot = runtime_types::sp_consensus_slots::Slot;
                }
                pub mod current_slot {
                    use super::runtime_types;
                    pub type CurrentSlot = runtime_types::sp_consensus_slots::Slot;
                }
                pub mod randomness {
                    use super::runtime_types;
                    pub type Randomness = [::core::primitive::u8; 32usize];
                }
                pub mod pending_epoch_config_change {
                    use super::runtime_types;
                    pub type PendingEpochConfigChange =
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                }
                pub mod next_randomness {
                    use super::runtime_types;
                    pub type NextRandomness = [::core::primitive::u8; 32usize];
                }
                pub mod next_authorities {
                    use super::runtime_types;
                    pub type NextAuthorities =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_babe::app::Public,
                            ::core::primitive::u64,
                        )>;
                }
                pub mod segment_index {
                    use super::runtime_types;
                    pub type SegmentIndex = ::core::primitive::u32;
                }
                pub mod under_construction {
                    use super::runtime_types;
                    pub type UnderConstruction =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            [::core::primitive::u8; 32usize],
                        >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod initialized {
                    use super::runtime_types;
                    pub type Initialized = ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::PreDigest,
                    >;
                }
                pub mod author_vrf_randomness {
                    use super::runtime_types;
                    pub type AuthorVrfRandomness =
                        ::core::option::Option<[::core::primitive::u8; 32usize]>;
                }
                pub mod epoch_start {
                    use super::runtime_types;
                    pub type EpochStart = (::core::primitive::u32, ::core::primitive::u32);
                }
                pub mod lateness {
                    use super::runtime_types;
                    pub type Lateness = ::core::primitive::u32;
                }
                pub mod epoch_config {
                    use super::runtime_types;
                    pub type EpochConfig = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                }
                pub mod next_epoch_config {
                    use super::runtime_types;
                    pub type NextEpochConfig =
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                }
                pub mod skipped_epochs {
                    use super::runtime_types;
                    pub type SkippedEpochs =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                            ::core::primitive::u64,
                            ::core::primitive::u32,
                        )>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn epoch_index(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::epoch_index::EpochIndex,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "EpochIndex",
                        (),
                        [
                            32u8, 82u8, 130u8, 31u8, 190u8, 162u8, 237u8, 189u8, 104u8, 244u8,
                            30u8, 199u8, 179u8, 0u8, 161u8, 107u8, 72u8, 240u8, 201u8, 222u8,
                            177u8, 222u8, 35u8, 156u8, 81u8, 132u8, 162u8, 118u8, 238u8, 84u8,
                            112u8, 89u8,
                        ],
                    )
                }
                pub fn authorities(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorities::Authorities,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "Authorities",
                        (),
                        [
                            192u8, 157u8, 98u8, 244u8, 104u8, 38u8, 195u8, 114u8, 183u8, 62u8,
                            247u8, 18u8, 31u8, 152u8, 246u8, 206u8, 97u8, 13u8, 118u8, 211u8,
                            104u8, 54u8, 150u8, 152u8, 126u8, 170u8, 228u8, 158u8, 108u8, 129u8,
                            134u8, 44u8,
                        ],
                    )
                }
                pub fn genesis_slot(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::genesis_slot::GenesisSlot,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "GenesisSlot",
                        (),
                        [
                            218u8, 174u8, 152u8, 76u8, 188u8, 214u8, 7u8, 88u8, 253u8, 187u8,
                            139u8, 234u8, 51u8, 28u8, 220u8, 57u8, 73u8, 1u8, 18u8, 205u8, 80u8,
                            160u8, 120u8, 216u8, 139u8, 191u8, 100u8, 108u8, 162u8, 106u8, 175u8,
                            107u8,
                        ],
                    )
                }
                pub fn current_slot(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_slot::CurrentSlot,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "CurrentSlot",
                        (),
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
                pub fn randomness(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::randomness::Randomness,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "Randomness",
                        (),
                        [
                            36u8, 15u8, 52u8, 73u8, 195u8, 177u8, 186u8, 125u8, 134u8, 11u8, 103u8,
                            248u8, 170u8, 237u8, 105u8, 239u8, 168u8, 204u8, 147u8, 52u8, 15u8,
                            226u8, 126u8, 176u8, 133u8, 186u8, 169u8, 241u8, 156u8, 118u8, 67u8,
                            58u8,
                        ],
                    )
                }
                pub fn pending_epoch_config_change(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending_epoch_config_change::PendingEpochConfigChange,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "PendingEpochConfigChange",
                        (),
                        [
                            79u8, 216u8, 84u8, 210u8, 83u8, 149u8, 122u8, 160u8, 159u8, 164u8,
                            16u8, 134u8, 154u8, 104u8, 77u8, 254u8, 139u8, 18u8, 163u8, 59u8, 92u8,
                            9u8, 135u8, 141u8, 147u8, 86u8, 44u8, 95u8, 183u8, 101u8, 11u8, 58u8,
                        ],
                    )
                }
                pub fn next_randomness(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_randomness::NextRandomness,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "NextRandomness",
                        (),
                        [
                            96u8, 191u8, 139u8, 171u8, 144u8, 92u8, 33u8, 58u8, 23u8, 219u8, 164u8,
                            121u8, 59u8, 209u8, 112u8, 244u8, 50u8, 8u8, 14u8, 244u8, 103u8, 125u8,
                            120u8, 210u8, 16u8, 250u8, 54u8, 192u8, 72u8, 8u8, 219u8, 152u8,
                        ],
                    )
                }
                pub fn next_authorities(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_authorities::NextAuthorities,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "NextAuthorities",
                        (),
                        [
                            29u8, 161u8, 79u8, 221u8, 198u8, 101u8, 11u8, 17u8, 20u8, 17u8, 225u8,
                            144u8, 35u8, 150u8, 241u8, 190u8, 106u8, 32u8, 230u8, 14u8, 212u8,
                            126u8, 1u8, 96u8, 73u8, 173u8, 245u8, 39u8, 153u8, 33u8, 205u8, 174u8,
                        ],
                    )
                }
                pub fn segment_index(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::segment_index::SegmentIndex,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "SegmentIndex",
                        (),
                        [
                            145u8, 91u8, 142u8, 240u8, 184u8, 94u8, 68u8, 52u8, 130u8, 3u8, 75u8,
                            175u8, 155u8, 130u8, 66u8, 9u8, 150u8, 242u8, 123u8, 111u8, 124u8,
                            241u8, 100u8, 128u8, 220u8, 133u8, 96u8, 227u8, 164u8, 241u8, 170u8,
                            34u8,
                        ],
                    )
                }
                pub fn under_construction_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::under_construction::UnderConstruction,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "UnderConstruction",
                        (),
                        [
                            120u8, 120u8, 59u8, 247u8, 50u8, 6u8, 220u8, 14u8, 2u8, 76u8, 203u8,
                            244u8, 232u8, 144u8, 253u8, 191u8, 101u8, 35u8, 99u8, 85u8, 111u8,
                            168u8, 31u8, 110u8, 187u8, 124u8, 72u8, 32u8, 43u8, 66u8, 8u8, 215u8,
                        ],
                    )
                }
                pub fn under_construction(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::under_construction::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::under_construction::Param0,
                    >,
                    types::under_construction::UnderConstruction,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "UnderConstruction",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            120u8, 120u8, 59u8, 247u8, 50u8, 6u8, 220u8, 14u8, 2u8, 76u8, 203u8,
                            244u8, 232u8, 144u8, 253u8, 191u8, 101u8, 35u8, 99u8, 85u8, 111u8,
                            168u8, 31u8, 110u8, 187u8, 124u8, 72u8, 32u8, 43u8, 66u8, 8u8, 215u8,
                        ],
                    )
                }
                pub fn initialized(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::initialized::Initialized,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "Initialized",
                        (),
                        [
                            169u8, 217u8, 237u8, 78u8, 186u8, 202u8, 206u8, 213u8, 54u8, 85u8,
                            206u8, 166u8, 22u8, 138u8, 236u8, 60u8, 211u8, 169u8, 12u8, 183u8,
                            23u8, 69u8, 194u8, 236u8, 112u8, 21u8, 62u8, 219u8, 92u8, 131u8, 134u8,
                            145u8,
                        ],
                    )
                }
                pub fn author_vrf_randomness(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::author_vrf_randomness::AuthorVrfRandomness,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "AuthorVrfRandomness",
                        (),
                        [
                            160u8, 157u8, 62u8, 48u8, 196u8, 136u8, 63u8, 132u8, 155u8, 183u8,
                            91u8, 201u8, 146u8, 29u8, 192u8, 142u8, 168u8, 152u8, 197u8, 233u8,
                            5u8, 25u8, 0u8, 154u8, 234u8, 180u8, 146u8, 132u8, 106u8, 164u8, 149u8,
                            63u8,
                        ],
                    )
                }
                pub fn epoch_start(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::epoch_start::EpochStart,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "EpochStart",
                        (),
                        [
                            144u8, 133u8, 140u8, 56u8, 241u8, 203u8, 199u8, 123u8, 244u8, 126u8,
                            196u8, 151u8, 214u8, 204u8, 243u8, 244u8, 210u8, 198u8, 174u8, 126u8,
                            200u8, 236u8, 248u8, 190u8, 181u8, 152u8, 113u8, 224u8, 95u8, 234u8,
                            169u8, 14u8,
                        ],
                    )
                }
                pub fn lateness(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::lateness::Lateness,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "Lateness",
                        (),
                        [
                            229u8, 214u8, 133u8, 149u8, 32u8, 159u8, 26u8, 22u8, 252u8, 131u8,
                            200u8, 191u8, 231u8, 176u8, 178u8, 127u8, 33u8, 212u8, 139u8, 220u8,
                            157u8, 38u8, 4u8, 226u8, 204u8, 32u8, 55u8, 20u8, 205u8, 141u8, 29u8,
                            87u8,
                        ],
                    )
                }
                pub fn epoch_config(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::epoch_config::EpochConfig,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "EpochConfig",
                        (),
                        [
                            151u8, 58u8, 93u8, 2u8, 19u8, 98u8, 41u8, 144u8, 241u8, 70u8, 195u8,
                            37u8, 126u8, 241u8, 111u8, 65u8, 16u8, 228u8, 111u8, 220u8, 241u8,
                            215u8, 179u8, 235u8, 122u8, 88u8, 92u8, 95u8, 131u8, 252u8, 236u8,
                            46u8,
                        ],
                    )
                }
                pub fn next_epoch_config(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_epoch_config::NextEpochConfig,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "NextEpochConfig",
                        (),
                        [
                            65u8, 54u8, 74u8, 141u8, 193u8, 124u8, 130u8, 238u8, 106u8, 27u8,
                            221u8, 189u8, 103u8, 53u8, 39u8, 243u8, 212u8, 216u8, 75u8, 185u8,
                            104u8, 220u8, 70u8, 108u8, 87u8, 172u8, 201u8, 185u8, 39u8, 55u8,
                            145u8, 6u8,
                        ],
                    )
                }
                pub fn skipped_epochs(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::skipped_epochs::SkippedEpochs,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Babe",
                        "SkippedEpochs",
                        (),
                        [
                            120u8, 167u8, 144u8, 97u8, 41u8, 216u8, 103u8, 90u8, 3u8, 86u8, 196u8,
                            35u8, 160u8, 150u8, 144u8, 233u8, 128u8, 35u8, 119u8, 66u8, 6u8, 63u8,
                            114u8, 140u8, 182u8, 228u8, 192u8, 30u8, 50u8, 145u8, 217u8, 108u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn epoch_duration(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Babe",
                        "EpochDuration",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn expected_block_time(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Babe",
                        "ExpectedBlockTime",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn max_authorities(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Babe",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_nominators(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Babe",
                        "MaxNominators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod authorship {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod author {
                    use super::runtime_types;
                    pub type Author = ::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn author(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::author::Author,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Authorship",
                        "Author",
                        (),
                        [
                            247u8, 192u8, 118u8, 227u8, 47u8, 20u8, 203u8, 199u8, 216u8, 87u8,
                            220u8, 50u8, 166u8, 61u8, 168u8, 213u8, 253u8, 62u8, 202u8, 199u8,
                            61u8, 192u8, 237u8, 53u8, 22u8, 148u8, 164u8, 245u8, 99u8, 24u8, 146u8,
                            18u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod restaking {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_restaking::pallet::Error;
        pub type Call = runtime_types::pallet_restaking::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct UpdateValidators {
                    pub payload: update_validators::Payload,
                    pub signature: update_validators::Signature,
                }
                pub mod update_validators {
                    use super::runtime_types;
                    pub type Payload = runtime_types::pallet_restaking::types::ObservationsPayload<
                        runtime_types::sp_runtime::MultiSigner,
                        ::core::primitive::u32,
                    >;
                    pub type Signature = runtime_types::sp_runtime::MultiSignature;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for UpdateValidators {
                    const PALLET: &'static str = "Restaking";
                    const CALL: &'static str = "update_validators";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AddRestakingPlatform {
                    pub platform_source_name: add_restaking_platform::PlatformSourceName,
                    pub url: add_restaking_platform::Url,
                    pub middleware_address: add_restaking_platform::MiddlewareAddress,
                }
                pub mod add_restaking_platform {
                    use super::runtime_types;
                    pub type PlatformSourceName = ::subxt_core::alloc::string::String;
                    pub type Url = ::subxt_core::alloc::string::String;
                    pub type MiddlewareAddress = ::subxt_core::alloc::string::String;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for AddRestakingPlatform {
                    const PALLET: &'static str = "Restaking";
                    const CALL: &'static str = "add_restaking_platform";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetRewardsPrePoint {
                    pub value: set_rewards_pre_point::Value,
                }
                pub mod set_rewards_pre_point {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetRewardsPrePoint {
                    const PALLET: &'static str = "Restaking";
                    const CALL: &'static str = "set_rewards_pre_point";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn update_validators(
                    &self,
                    payload: types::update_validators::Payload,
                    signature: types::update_validators::Signature,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::UpdateValidators>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Restaking",
                        "update_validators",
                        types::UpdateValidators { payload, signature },
                        [
                            208u8, 197u8, 145u8, 30u8, 63u8, 151u8, 201u8, 90u8, 10u8, 108u8,
                            132u8, 180u8, 91u8, 184u8, 117u8, 241u8, 118u8, 194u8, 150u8, 95u8,
                            62u8, 243u8, 127u8, 154u8, 110u8, 113u8, 99u8, 251u8, 255u8, 17u8,
                            14u8, 90u8,
                        ],
                    )
                }
                pub fn add_restaking_platform(
                    &self,
                    platform_source_name: types::add_restaking_platform::PlatformSourceName,
                    url: types::add_restaking_platform::Url,
                    middleware_address: types::add_restaking_platform::MiddlewareAddress,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::AddRestakingPlatform>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Restaking",
                        "add_restaking_platform",
                        types::AddRestakingPlatform {
                            platform_source_name,
                            url,
                            middleware_address,
                        },
                        [
                            48u8, 5u8, 59u8, 152u8, 103u8, 190u8, 187u8, 208u8, 128u8, 221u8,
                            153u8, 117u8, 79u8, 109u8, 48u8, 77u8, 21u8, 192u8, 182u8, 192u8,
                            216u8, 248u8, 33u8, 146u8, 161u8, 129u8, 121u8, 12u8, 150u8, 30u8,
                            38u8, 153u8,
                        ],
                    )
                }
                pub fn set_rewards_pre_point(
                    &self,
                    value: types::set_rewards_pre_point::Value,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetRewardsPrePoint>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Restaking",
                        "set_rewards_pre_point",
                        types::SetRewardsPrePoint { value },
                        [
                            255u8, 253u8, 68u8, 162u8, 145u8, 195u8, 203u8, 132u8, 176u8, 135u8,
                            161u8, 210u8, 47u8, 118u8, 32u8, 14u8, 118u8, 27u8, 48u8, 125u8, 206u8,
                            191u8, 123u8, 1u8, 148u8, 226u8, 217u8, 124u8, 46u8, 38u8, 150u8,
                            126u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_restaking::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct NewPlannedValidators {
                pub set_id: new_planned_validators::SetId,
                pub validators: new_planned_validators::Validators,
            }
            pub mod new_planned_validators {
                use super::runtime_types;
                pub type SetId = ::core::primitive::u32;
                pub type Validators = ::subxt_core::alloc::vec::Vec<(
                    ::subxt_core::utils::AccountId32,
                    ::core::primitive::u128,
                )>;
            }
            impl ::subxt_core::events::StaticEvent for NewPlannedValidators {
                const PALLET: &'static str = "Restaking";
                const EVENT: &'static str = "NewPlannedValidators";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct UnlockFailed {
                pub sender: unlock_failed::Sender,
                pub receiver: unlock_failed::Receiver,
                pub amount: unlock_failed::Amount,
                pub sequence: unlock_failed::Sequence,
            }
            pub mod unlock_failed {
                use super::runtime_types;
                pub type Sender = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Receiver = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type Sequence = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for UnlockFailed {
                const PALLET: &'static str = "Restaking";
                const EVENT: &'static str = "UnlockFailed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct MintNep141Failed {
                pub token_id: mint_nep141_failed::TokenId,
                pub sender: mint_nep141_failed::Sender,
                pub receiver: mint_nep141_failed::Receiver,
                pub amount: mint_nep141_failed::Amount,
                pub sequence: mint_nep141_failed::Sequence,
            }
            pub mod mint_nep141_failed {
                use super::runtime_types;
                pub type TokenId = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Sender = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Receiver = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type Sequence = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for MintNep141Failed {
                const PALLET: &'static str = "Restaking";
                const EVENT: &'static str = "MintNep141Failed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct UnlockNonfungibleFailed {
                pub collection: unlock_nonfungible_failed::Collection,
                pub item: unlock_nonfungible_failed::Item,
                pub sender: unlock_nonfungible_failed::Sender,
                pub receiver: unlock_nonfungible_failed::Receiver,
                pub sequence: unlock_nonfungible_failed::Sequence,
            }
            pub mod unlock_nonfungible_failed {
                use super::runtime_types;
                pub type Collection = ::core::primitive::u128;
                pub type Item = ::core::primitive::u128;
                pub type Sender = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Receiver = ::subxt_core::utils::AccountId32;
                pub type Sequence = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for UnlockNonfungibleFailed {
                const PALLET: &'static str = "Restaking";
                const EVENT: &'static str = "UnlockNonfungibleFailed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct RewardsPerPointUpdated {
                pub value: rewards_per_point_updated::Value,
            }
            pub mod rewards_per_point_updated {
                use super::runtime_types;
                pub type Value = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for RewardsPerPointUpdated {
                const PALLET: &'static str = "Restaking";
                const EVENT: &'static str = "RewardsPerPointUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod is_activated {
                    use super::runtime_types;
                    pub type IsActivated = ::core::primitive::bool;
                }
                pub mod rewards_amount_per_point {
                    use super::runtime_types;
                    pub type RewardsAmountPerPoint = ::core::primitive::u128;
                }
                pub mod next_set_id {
                    use super::runtime_types;
                    pub type NextSetId = ::core::primitive::u32;
                }
                pub mod planned_validators {
                    use super::runtime_types;
                    pub type PlannedValidators = ::subxt_core::alloc::vec::Vec<(
                        ::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                        ::subxt_core::alloc::string::String,
                    )>;
                }
                pub mod validators_source {
                    use super::runtime_types;
                    pub type ValidatorsSource =
                        runtime_types::pallet_restaking::validator_data::ValidatorData;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod next_notification_id {
                    use super::runtime_types;
                    pub type NextNotificationId = ::core::primitive::u32;
                }
                pub mod observations {
                    use super::runtime_types;
                    pub type Observations =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::pallet_restaking::types::Observation<
                                ::subxt_core::utils::AccountId32,
                            >,
                        >;
                    pub type Param0 = runtime_types::pallet_restaking::types::ObservationType;
                    pub type Param1 = ::core::primitive::u32;
                }
                pub mod need_fetch_restaking_validators {
                    use super::runtime_types;
                    pub type NeedFetchRestakingValidators = ::core::primitive::bool;
                }
                pub mod latest_closed_era {
                    use super::runtime_types;
                    pub type LatestClosedEra = ::core::primitive::u32;
                }
                pub mod era_rewards_detail {
                    use super::runtime_types;
                    pub type EraRewardsDetail =
                        runtime_types::pallet_restaking::types::EraRewardDetailsValue;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod total_rewards {
                    use super::runtime_types;
                    pub type TotalRewards = ::core::primitive::u128;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod restaking_platform {
                    use super::runtime_types;
                    pub type RestakingPlatform = (
                        ::subxt_core::alloc::string::String,
                        ::subxt_core::alloc::string::String,
                    );
                    pub type Param0 = ::core::primitive::str;
                }
                pub mod rewards_root {
                    use super::runtime_types;
                    pub type RewardsRoot = ::subxt_core::alloc::string::String;
                }
                pub mod notification_history {
                    use super::runtime_types;
                    pub type NotificationHistory = ::core::option::Option<
                        runtime_types::pallet_restaking::types::NotificationResult,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn is_activated(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::is_activated::IsActivated,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "IsActivated",
                        (),
                        [
                            116u8, 219u8, 77u8, 197u8, 135u8, 165u8, 182u8, 68u8, 25u8, 140u8,
                            241u8, 13u8, 151u8, 221u8, 15u8, 175u8, 54u8, 36u8, 236u8, 63u8, 191u8,
                            244u8, 122u8, 95u8, 66u8, 230u8, 124u8, 134u8, 31u8, 163u8, 86u8, 35u8,
                        ],
                    )
                }
                pub fn rewards_amount_per_point(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::rewards_amount_per_point::RewardsAmountPerPoint,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "RewardsAmountPerPoint",
                        (),
                        [
                            219u8, 180u8, 79u8, 203u8, 41u8, 36u8, 241u8, 50u8, 7u8, 50u8, 85u8,
                            43u8, 169u8, 199u8, 83u8, 86u8, 166u8, 82u8, 181u8, 207u8, 39u8, 225u8,
                            203u8, 113u8, 56u8, 11u8, 143u8, 88u8, 36u8, 166u8, 152u8, 237u8,
                        ],
                    )
                }
                pub fn next_set_id(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_set_id::NextSetId,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "NextSetId",
                        (),
                        [
                            251u8, 93u8, 218u8, 146u8, 95u8, 119u8, 166u8, 235u8, 163u8, 3u8,
                            121u8, 92u8, 43u8, 237u8, 112u8, 241u8, 236u8, 30u8, 111u8, 165u8,
                            63u8, 89u8, 175u8, 213u8, 161u8, 20u8, 51u8, 87u8, 162u8, 227u8, 78u8,
                            252u8,
                        ],
                    )
                }
                pub fn planned_validators(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::planned_validators::PlannedValidators,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "PlannedValidators",
                        (),
                        [
                            101u8, 220u8, 163u8, 47u8, 169u8, 40u8, 158u8, 236u8, 109u8, 39u8,
                            69u8, 108u8, 143u8, 136u8, 46u8, 234u8, 166u8, 60u8, 217u8, 49u8,
                            165u8, 97u8, 103u8, 200u8, 208u8, 93u8, 151u8, 48u8, 77u8, 74u8, 167u8,
                            233u8,
                        ],
                    )
                }
                pub fn validators_source_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::validators_source::ValidatorsSource,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "ValidatorsSource",
                        (),
                        [
                            20u8, 117u8, 174u8, 159u8, 101u8, 204u8, 206u8, 7u8, 8u8, 19u8, 193u8,
                            24u8, 228u8, 154u8, 156u8, 136u8, 48u8, 167u8, 86u8, 35u8, 250u8, 38u8,
                            20u8, 170u8, 50u8, 62u8, 187u8, 114u8, 183u8, 107u8, 232u8, 20u8,
                        ],
                    )
                }
                pub fn validators_source(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::validators_source::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::validators_source::Param0,
                    >,
                    types::validators_source::ValidatorsSource,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "ValidatorsSource",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            20u8, 117u8, 174u8, 159u8, 101u8, 204u8, 206u8, 7u8, 8u8, 19u8, 193u8,
                            24u8, 228u8, 154u8, 156u8, 136u8, 48u8, 167u8, 86u8, 35u8, 250u8, 38u8,
                            20u8, 170u8, 50u8, 62u8, 187u8, 114u8, 183u8, 107u8, 232u8, 20u8,
                        ],
                    )
                }
                pub fn next_notification_id(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_notification_id::NextNotificationId,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "NextNotificationId",
                        (),
                        [
                            214u8, 46u8, 249u8, 129u8, 35u8, 77u8, 148u8, 139u8, 31u8, 17u8, 251u8,
                            169u8, 34u8, 255u8, 180u8, 5u8, 224u8, 234u8, 208u8, 28u8, 138u8,
                            244u8, 203u8, 72u8, 250u8, 205u8, 161u8, 184u8, 167u8, 57u8, 14u8,
                            43u8,
                        ],
                    )
                }
                pub fn observations_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::observations::Observations,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "Observations",
                        (),
                        [
                            156u8, 187u8, 251u8, 187u8, 135u8, 100u8, 95u8, 249u8, 224u8, 14u8,
                            76u8, 20u8, 244u8, 84u8, 175u8, 243u8, 99u8, 22u8, 22u8, 197u8, 186u8,
                            96u8, 152u8, 184u8, 96u8, 137u8, 30u8, 164u8, 218u8, 246u8, 1u8, 13u8,
                        ],
                    )
                }
                pub fn observations_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::observations::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::observations::Param0>,
                    types::observations::Observations,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "Observations",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            156u8, 187u8, 251u8, 187u8, 135u8, 100u8, 95u8, 249u8, 224u8, 14u8,
                            76u8, 20u8, 244u8, 84u8, 175u8, 243u8, 99u8, 22u8, 22u8, 197u8, 186u8,
                            96u8, 152u8, 184u8, 96u8, 137u8, 30u8, 164u8, 218u8, 246u8, 1u8, 13u8,
                        ],
                    )
                }
                pub fn observations(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::observations::Param0>,
                    _1: impl ::core::borrow::Borrow<types::observations::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::observations::Param0,
                        >,
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::observations::Param1,
                        >,
                    ),
                    types::observations::Observations,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "Observations",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            156u8, 187u8, 251u8, 187u8, 135u8, 100u8, 95u8, 249u8, 224u8, 14u8,
                            76u8, 20u8, 244u8, 84u8, 175u8, 243u8, 99u8, 22u8, 22u8, 197u8, 186u8,
                            96u8, 152u8, 184u8, 96u8, 137u8, 30u8, 164u8, 218u8, 246u8, 1u8, 13u8,
                        ],
                    )
                }
                pub fn need_fetch_restaking_validators(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::need_fetch_restaking_validators::NeedFetchRestakingValidators,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "NeedFetchRestakingValidators",
                        (),
                        [
                            249u8, 93u8, 10u8, 85u8, 254u8, 46u8, 42u8, 54u8, 133u8, 194u8, 30u8,
                            238u8, 32u8, 192u8, 13u8, 229u8, 240u8, 91u8, 48u8, 245u8, 89u8, 50u8,
                            167u8, 63u8, 201u8, 88u8, 23u8, 79u8, 57u8, 213u8, 61u8, 255u8,
                        ],
                    )
                }
                pub fn latest_closed_era(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::latest_closed_era::LatestClosedEra,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "LatestClosedEra",
                        (),
                        [
                            216u8, 117u8, 180u8, 222u8, 25u8, 117u8, 103u8, 60u8, 192u8, 125u8,
                            81u8, 255u8, 110u8, 62u8, 69u8, 188u8, 56u8, 223u8, 112u8, 189u8,
                            128u8, 203u8, 221u8, 68u8, 230u8, 253u8, 182u8, 209u8, 23u8, 215u8,
                            180u8, 121u8,
                        ],
                    )
                }
                pub fn era_rewards_detail_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::era_rewards_detail::EraRewardsDetail,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "EraRewardsDetail",
                        (),
                        [
                            8u8, 19u8, 65u8, 170u8, 28u8, 11u8, 229u8, 132u8, 196u8, 228u8, 240u8,
                            151u8, 81u8, 108u8, 118u8, 91u8, 204u8, 196u8, 122u8, 195u8, 44u8,
                            104u8, 111u8, 138u8, 209u8, 131u8, 46u8, 92u8, 112u8, 106u8, 80u8,
                            72u8,
                        ],
                    )
                }
                pub fn era_rewards_detail(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::era_rewards_detail::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::era_rewards_detail::Param0,
                    >,
                    types::era_rewards_detail::EraRewardsDetail,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "EraRewardsDetail",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            8u8, 19u8, 65u8, 170u8, 28u8, 11u8, 229u8, 132u8, 196u8, 228u8, 240u8,
                            151u8, 81u8, 108u8, 118u8, 91u8, 204u8, 196u8, 122u8, 195u8, 44u8,
                            104u8, 111u8, 138u8, 209u8, 131u8, 46u8, 92u8, 112u8, 106u8, 80u8,
                            72u8,
                        ],
                    )
                }
                pub fn total_rewards_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_rewards::TotalRewards,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "TotalRewards",
                        (),
                        [
                            167u8, 16u8, 159u8, 4u8, 129u8, 186u8, 200u8, 245u8, 58u8, 93u8, 50u8,
                            114u8, 125u8, 222u8, 185u8, 140u8, 158u8, 155u8, 42u8, 134u8, 41u8,
                            192u8, 79u8, 72u8, 126u8, 23u8, 37u8, 211u8, 106u8, 177u8, 162u8, 25u8,
                        ],
                    )
                }
                pub fn total_rewards(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::total_rewards::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::total_rewards::Param0>,
                    types::total_rewards::TotalRewards,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "TotalRewards",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            167u8, 16u8, 159u8, 4u8, 129u8, 186u8, 200u8, 245u8, 58u8, 93u8, 50u8,
                            114u8, 125u8, 222u8, 185u8, 140u8, 158u8, 155u8, 42u8, 134u8, 41u8,
                            192u8, 79u8, 72u8, 126u8, 23u8, 37u8, 211u8, 106u8, 177u8, 162u8, 25u8,
                        ],
                    )
                }
                pub fn restaking_platform_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::restaking_platform::RestakingPlatform,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "RestakingPlatform",
                        (),
                        [
                            229u8, 48u8, 143u8, 25u8, 186u8, 82u8, 189u8, 138u8, 37u8, 212u8, 57u8,
                            95u8, 35u8, 164u8, 223u8, 39u8, 51u8, 132u8, 86u8, 139u8, 231u8, 43u8,
                            181u8, 128u8, 198u8, 116u8, 105u8, 70u8, 178u8, 20u8, 84u8, 235u8,
                        ],
                    )
                }
                pub fn restaking_platform(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::restaking_platform::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::restaking_platform::Param0,
                    >,
                    types::restaking_platform::RestakingPlatform,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "RestakingPlatform",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            229u8, 48u8, 143u8, 25u8, 186u8, 82u8, 189u8, 138u8, 37u8, 212u8, 57u8,
                            95u8, 35u8, 164u8, 223u8, 39u8, 51u8, 132u8, 86u8, 139u8, 231u8, 43u8,
                            181u8, 128u8, 198u8, 116u8, 105u8, 70u8, 178u8, 20u8, 84u8, 235u8,
                        ],
                    )
                }
                pub fn rewards_root(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::rewards_root::RewardsRoot,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "RewardsRoot",
                        (),
                        [
                            3u8, 183u8, 58u8, 159u8, 10u8, 47u8, 33u8, 204u8, 43u8, 80u8, 108u8,
                            77u8, 30u8, 76u8, 210u8, 9u8, 118u8, 161u8, 160u8, 209u8, 140u8, 104u8,
                            90u8, 115u8, 62u8, 139u8, 238u8, 78u8, 86u8, 167u8, 84u8, 128u8,
                        ],
                    )
                }
                pub fn notification_history_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::notification_history::NotificationHistory,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "NotificationHistory",
                        (),
                        [
                            201u8, 203u8, 250u8, 181u8, 100u8, 1u8, 220u8, 88u8, 233u8, 157u8,
                            195u8, 189u8, 137u8, 95u8, 134u8, 92u8, 48u8, 169u8, 227u8, 90u8,
                            183u8, 253u8, 144u8, 122u8, 165u8, 220u8, 170u8, 235u8, 58u8, 88u8,
                            189u8, 57u8,
                        ],
                    )
                }
                pub fn notification_history(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::notification_history::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::notification_history::Param0,
                    >,
                    types::notification_history::NotificationHistory,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Restaking",
                        "NotificationHistory",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            201u8, 203u8, 250u8, 181u8, 100u8, 1u8, 220u8, 88u8, 233u8, 157u8,
                            195u8, 189u8, 137u8, 95u8, 134u8, 92u8, 48u8, 169u8, 227u8, 90u8,
                            183u8, 253u8, 144u8, 122u8, 165u8, 220u8, 170u8, 235u8, 58u8, 88u8,
                            189u8, 57u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Restaking",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
                pub fn request_event_limit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Restaking",
                        "RequestEventLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_validators(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Restaking",
                        "MaxValidators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn restaking_enable(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::bool>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Restaking",
                        "RestakingEnable",
                        [
                            165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
                            252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
                            100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod keys {
                    use super::runtime_types;
                    pub type Keys =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::sp_authority_discovery::app::Public,
                        >;
                }
                pub mod next_keys {
                    use super::runtime_types;
                    pub type NextKeys =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::sp_authority_discovery::app::Public,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn keys(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::keys::Keys,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "AuthorityDiscovery",
                        "Keys",
                        (),
                        [
                            35u8, 71u8, 73u8, 255u8, 160u8, 250u8, 38u8, 205u8, 32u8, 139u8, 236u8,
                            83u8, 194u8, 12u8, 20u8, 221u8, 114u8, 94u8, 196u8, 246u8, 136u8,
                            175u8, 70u8, 98u8, 91u8, 50u8, 236u8, 131u8, 131u8, 146u8, 150u8,
                            192u8,
                        ],
                    )
                }
                pub fn next_keys(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_keys::NextKeys,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "AuthorityDiscovery",
                        "NextKeys",
                        (),
                        [
                            54u8, 44u8, 61u8, 196u8, 2u8, 249u8, 185u8, 199u8, 245u8, 154u8, 178u8,
                            109u8, 237u8, 147u8, 72u8, 209u8, 72u8, 196u8, 31u8, 192u8, 217u8,
                            231u8, 71u8, 28u8, 148u8, 138u8, 29u8, 115u8, 247u8, 95u8, 185u8,
                            189u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod validator {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_validator::pallet::Error;
        pub type Event = runtime_types::pallet_validator::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ForceEra {
                pub mode: force_era::Mode,
            }
            pub mod force_era {
                use super::runtime_types;
                pub type Mode = runtime_types::pallet_validator::types::Forcing;
            }
            impl ::subxt_core::events::StaticEvent for ForceEra {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "ForceEra";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct TriggerNewEra;
            impl ::subxt_core::events::StaticEvent for TriggerNewEra {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "TriggerNewEra";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct EraPaid {
                pub era_index: era_paid::EraIndex,
                pub validator_payout: era_paid::ValidatorPayout,
                pub staker_payout: era_paid::StakerPayout,
            }
            pub mod era_paid {
                use super::runtime_types;
                pub type EraIndex = ::core::primitive::u32;
                pub type ValidatorPayout = ::core::primitive::u128;
                pub type StakerPayout = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for EraPaid {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct EraStarted(pub era_started::Field0);
            pub mod era_started {
                use super::runtime_types;
                pub type Field0 = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for EraStarted {
                const PALLET: &'static str = "Validator";
                const EVENT: &'static str = "EraStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod eras_validator_reward {
                    use super::runtime_types;
                    pub type ErasValidatorReward = ::core::primitive::u128;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod max_staked_rewards {
                    use super::runtime_types;
                    pub type MaxStakedRewards = runtime_types::sp_arithmetic::per_things::Percent;
                }
                pub mod era_payout {
                    use super::runtime_types;
                    pub type EraPayout = ::core::primitive::u128;
                }
                pub mod current_planned_session {
                    use super::runtime_types;
                    pub type CurrentPlannedSession = ::core::primitive::u32;
                }
                pub mod current_era {
                    use super::runtime_types;
                    pub type CurrentEra = ::core::primitive::u32;
                }
                pub mod eras_total_stake {
                    use super::runtime_types;
                    pub type ErasTotalStake = ::core::primitive::u128;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod eras_reward_points {
                    use super::runtime_types;
                    pub type ErasRewardPoints = runtime_types::vrs_support::EraRewardPoints<
                        ::subxt_core::utils::AccountId32,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod bonded_eras {
                    use super::runtime_types;
                    pub type BondedEras = ::subxt_core::alloc::vec::Vec<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>;
                }
                pub mod active_era {
                    use super::runtime_types;
                    pub type ActiveEra = runtime_types::pallet_validator::types::ActiveEraInfo;
                }
                pub mod eras_start_session_index {
                    use super::runtime_types;
                    pub type ErasStartSessionIndex = ::core::primitive::u32;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod eras_stakers {
                    use super::runtime_types;
                    pub type ErasStakers = ::core::primitive::u128;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::subxt_core::utils::AccountId32;
                }
                pub mod force_era {
                    use super::runtime_types;
                    pub type ForceEra = runtime_types::pallet_validator::types::Forcing;
                }
                pub mod current_session {
                    use super::runtime_types;
                    pub type CurrentSession = ::core::primitive::u32;
                }
                pub mod era_validators {
                    use super::runtime_types;
                    pub type EraValidators =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod session_offenders {
                    use super::runtime_types;
                    pub type SessionOffenders =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod genesis_validators {
                    use super::runtime_types;
                    pub type GenesisValidators =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn eras_validator_reward_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::eras_validator_reward::ErasValidatorReward,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasValidatorReward",
                        (),
                        [
                            185u8, 85u8, 179u8, 163u8, 178u8, 168u8, 141u8, 200u8, 59u8, 77u8, 2u8,
                            197u8, 36u8, 188u8, 133u8, 117u8, 2u8, 25u8, 105u8, 132u8, 44u8, 75u8,
                            15u8, 82u8, 57u8, 89u8, 242u8, 234u8, 70u8, 244u8, 198u8, 126u8,
                        ],
                    )
                }
                pub fn eras_validator_reward(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::eras_validator_reward::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::eras_validator_reward::Param0,
                    >,
                    types::eras_validator_reward::ErasValidatorReward,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasValidatorReward",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            185u8, 85u8, 179u8, 163u8, 178u8, 168u8, 141u8, 200u8, 59u8, 77u8, 2u8,
                            197u8, 36u8, 188u8, 133u8, 117u8, 2u8, 25u8, 105u8, 132u8, 44u8, 75u8,
                            15u8, 82u8, 57u8, 89u8, 242u8, 234u8, 70u8, 244u8, 198u8, 126u8,
                        ],
                    )
                }
                pub fn max_staked_rewards(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::max_staked_rewards::MaxStakedRewards,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "MaxStakedRewards",
                        (),
                        [
                            115u8, 208u8, 243u8, 76u8, 110u8, 71u8, 154u8, 80u8, 193u8, 138u8,
                            75u8, 11u8, 24u8, 51u8, 112u8, 125u8, 153u8, 151u8, 216u8, 67u8, 214u8,
                            4u8, 94u8, 229u8, 32u8, 186u8, 140u8, 150u8, 100u8, 233u8, 88u8, 53u8,
                        ],
                    )
                }
                pub fn era_payout(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::era_payout::EraPayout,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "EraPayout",
                        (),
                        [
                            10u8, 83u8, 179u8, 33u8, 74u8, 100u8, 32u8, 115u8, 30u8, 162u8, 202u8,
                            22u8, 9u8, 83u8, 223u8, 203u8, 60u8, 232u8, 103u8, 61u8, 67u8, 60u8,
                            155u8, 231u8, 240u8, 84u8, 160u8, 102u8, 251u8, 50u8, 105u8, 162u8,
                        ],
                    )
                }
                pub fn current_planned_session(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_planned_session::CurrentPlannedSession,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "CurrentPlannedSession",
                        (),
                        [
                            12u8, 47u8, 20u8, 104u8, 155u8, 181u8, 35u8, 91u8, 172u8, 97u8, 206u8,
                            135u8, 185u8, 142u8, 46u8, 72u8, 32u8, 118u8, 225u8, 191u8, 28u8,
                            130u8, 7u8, 38u8, 181u8, 233u8, 201u8, 8u8, 160u8, 161u8, 86u8, 204u8,
                        ],
                    )
                }
                pub fn current_era(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_era::CurrentEra,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "CurrentEra",
                        (),
                        [
                            247u8, 239u8, 171u8, 18u8, 137u8, 240u8, 213u8, 3u8, 173u8, 173u8,
                            236u8, 141u8, 202u8, 191u8, 228u8, 120u8, 196u8, 188u8, 13u8, 66u8,
                            253u8, 117u8, 90u8, 8u8, 158u8, 11u8, 236u8, 141u8, 178u8, 44u8, 119u8,
                            25u8,
                        ],
                    )
                }
                pub fn eras_total_stake_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::eras_total_stake::ErasTotalStake,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasTotalStake",
                        (),
                        [
                            8u8, 78u8, 101u8, 62u8, 124u8, 126u8, 66u8, 26u8, 47u8, 126u8, 239u8,
                            204u8, 222u8, 104u8, 19u8, 108u8, 238u8, 160u8, 112u8, 242u8, 56u8,
                            2u8, 250u8, 164u8, 250u8, 213u8, 201u8, 84u8, 193u8, 117u8, 108u8,
                            146u8,
                        ],
                    )
                }
                pub fn eras_total_stake(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::eras_total_stake::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::eras_total_stake::Param0,
                    >,
                    types::eras_total_stake::ErasTotalStake,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasTotalStake",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            8u8, 78u8, 101u8, 62u8, 124u8, 126u8, 66u8, 26u8, 47u8, 126u8, 239u8,
                            204u8, 222u8, 104u8, 19u8, 108u8, 238u8, 160u8, 112u8, 242u8, 56u8,
                            2u8, 250u8, 164u8, 250u8, 213u8, 201u8, 84u8, 193u8, 117u8, 108u8,
                            146u8,
                        ],
                    )
                }
                pub fn eras_reward_points_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::eras_reward_points::ErasRewardPoints,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasRewardPoints",
                        (),
                        [
                            99u8, 244u8, 134u8, 185u8, 156u8, 245u8, 230u8, 55u8, 101u8, 128u8,
                            78u8, 202u8, 180u8, 253u8, 240u8, 166u8, 146u8, 117u8, 138u8, 52u8,
                            99u8, 235u8, 62u8, 10u8, 189u8, 1u8, 34u8, 8u8, 162u8, 101u8, 240u8,
                            100u8,
                        ],
                    )
                }
                pub fn eras_reward_points(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::eras_reward_points::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::eras_reward_points::Param0,
                    >,
                    types::eras_reward_points::ErasRewardPoints,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasRewardPoints",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            99u8, 244u8, 134u8, 185u8, 156u8, 245u8, 230u8, 55u8, 101u8, 128u8,
                            78u8, 202u8, 180u8, 253u8, 240u8, 166u8, 146u8, 117u8, 138u8, 52u8,
                            99u8, 235u8, 62u8, 10u8, 189u8, 1u8, 34u8, 8u8, 162u8, 101u8, 240u8,
                            100u8,
                        ],
                    )
                }
                pub fn bonded_eras(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::bonded_eras::BondedEras,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "BondedEras",
                        (),
                        [
                            20u8, 0u8, 164u8, 169u8, 183u8, 130u8, 242u8, 167u8, 92u8, 254u8,
                            191u8, 206u8, 177u8, 182u8, 219u8, 162u8, 7u8, 116u8, 223u8, 166u8,
                            239u8, 216u8, 140u8, 42u8, 174u8, 237u8, 134u8, 186u8, 180u8, 62u8,
                            175u8, 239u8,
                        ],
                    )
                }
                pub fn active_era(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::active_era::ActiveEra,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ActiveEra",
                        (),
                        [
                            238u8, 29u8, 7u8, 251u8, 119u8, 102u8, 120u8, 162u8, 118u8, 152u8,
                            146u8, 59u8, 8u8, 23u8, 113u8, 136u8, 4u8, 91u8, 110u8, 27u8, 103u8,
                            136u8, 41u8, 132u8, 163u8, 11u8, 132u8, 9u8, 25u8, 146u8, 31u8, 248u8,
                        ],
                    )
                }
                pub fn eras_start_session_index_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::eras_start_session_index::ErasStartSessionIndex,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasStartSessionIndex",
                        (),
                        [
                            104u8, 76u8, 102u8, 20u8, 9u8, 146u8, 55u8, 204u8, 12u8, 15u8, 117u8,
                            22u8, 54u8, 230u8, 98u8, 105u8, 191u8, 136u8, 140u8, 65u8, 48u8, 29u8,
                            19u8, 144u8, 159u8, 241u8, 158u8, 77u8, 4u8, 230u8, 216u8, 52u8,
                        ],
                    )
                }
                pub fn eras_start_session_index(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::eras_start_session_index::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::eras_start_session_index::Param0,
                    >,
                    types::eras_start_session_index::ErasStartSessionIndex,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasStartSessionIndex",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            104u8, 76u8, 102u8, 20u8, 9u8, 146u8, 55u8, 204u8, 12u8, 15u8, 117u8,
                            22u8, 54u8, 230u8, 98u8, 105u8, 191u8, 136u8, 140u8, 65u8, 48u8, 29u8,
                            19u8, 144u8, 159u8, 241u8, 158u8, 77u8, 4u8, 230u8, 216u8, 52u8,
                        ],
                    )
                }
                pub fn eras_stakers_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::eras_stakers::ErasStakers,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasStakers",
                        (),
                        [
                            55u8, 133u8, 123u8, 59u8, 115u8, 98u8, 16u8, 242u8, 239u8, 51u8, 36u8,
                            93u8, 113u8, 160u8, 153u8, 228u8, 202u8, 7u8, 128u8, 95u8, 76u8, 195u8,
                            243u8, 99u8, 27u8, 126u8, 174u8, 210u8, 70u8, 157u8, 223u8, 203u8,
                        ],
                    )
                }
                pub fn eras_stakers_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::eras_stakers::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::eras_stakers::Param0>,
                    types::eras_stakers::ErasStakers,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasStakers",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            55u8, 133u8, 123u8, 59u8, 115u8, 98u8, 16u8, 242u8, 239u8, 51u8, 36u8,
                            93u8, 113u8, 160u8, 153u8, 228u8, 202u8, 7u8, 128u8, 95u8, 76u8, 195u8,
                            243u8, 99u8, 27u8, 126u8, 174u8, 210u8, 70u8, 157u8, 223u8, 203u8,
                        ],
                    )
                }
                pub fn eras_stakers(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::eras_stakers::Param0>,
                    _1: impl ::core::borrow::Borrow<types::eras_stakers::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::eras_stakers::Param0,
                        >,
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::eras_stakers::Param1,
                        >,
                    ),
                    types::eras_stakers::ErasStakers,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ErasStakers",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            55u8, 133u8, 123u8, 59u8, 115u8, 98u8, 16u8, 242u8, 239u8, 51u8, 36u8,
                            93u8, 113u8, 160u8, 153u8, 228u8, 202u8, 7u8, 128u8, 95u8, 76u8, 195u8,
                            243u8, 99u8, 27u8, 126u8, 174u8, 210u8, 70u8, 157u8, 223u8, 203u8,
                        ],
                    )
                }
                pub fn force_era(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::force_era::ForceEra,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "ForceEra",
                        (),
                        [
                            177u8, 148u8, 73u8, 108u8, 136u8, 126u8, 89u8, 18u8, 124u8, 66u8, 30u8,
                            102u8, 133u8, 164u8, 78u8, 214u8, 184u8, 163u8, 75u8, 164u8, 117u8,
                            233u8, 209u8, 158u8, 99u8, 208u8, 21u8, 194u8, 152u8, 82u8, 16u8,
                            222u8,
                        ],
                    )
                }
                pub fn current_session(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_session::CurrentSession,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "CurrentSession",
                        (),
                        [
                            206u8, 79u8, 91u8, 17u8, 25u8, 230u8, 101u8, 217u8, 216u8, 213u8,
                            118u8, 134u8, 133u8, 128u8, 245u8, 237u8, 131u8, 145u8, 7u8, 117u8,
                            157u8, 228u8, 42u8, 119u8, 177u8, 124u8, 188u8, 251u8, 135u8, 118u8,
                            23u8, 4u8,
                        ],
                    )
                }
                pub fn era_validators_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::era_validators::EraValidators,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "EraValidators",
                        (),
                        [
                            11u8, 112u8, 128u8, 103u8, 243u8, 145u8, 85u8, 191u8, 228u8, 218u8,
                            236u8, 54u8, 9u8, 218u8, 206u8, 51u8, 82u8, 199u8, 87u8, 40u8, 211u8,
                            152u8, 119u8, 158u8, 4u8, 221u8, 126u8, 47u8, 208u8, 212u8, 149u8, 5u8,
                        ],
                    )
                }
                pub fn era_validators(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::era_validators::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::era_validators::Param0>,
                    types::era_validators::EraValidators,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "EraValidators",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            11u8, 112u8, 128u8, 103u8, 243u8, 145u8, 85u8, 191u8, 228u8, 218u8,
                            236u8, 54u8, 9u8, 218u8, 206u8, 51u8, 82u8, 199u8, 87u8, 40u8, 211u8,
                            152u8, 119u8, 158u8, 4u8, 221u8, 126u8, 47u8, 208u8, 212u8, 149u8, 5u8,
                        ],
                    )
                }
                pub fn session_offenders_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::session_offenders::SessionOffenders,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "SessionOffenders",
                        (),
                        [
                            173u8, 83u8, 41u8, 232u8, 177u8, 164u8, 9u8, 255u8, 8u8, 241u8, 60u8,
                            145u8, 22u8, 191u8, 168u8, 218u8, 237u8, 64u8, 142u8, 39u8, 176u8,
                            197u8, 150u8, 228u8, 72u8, 175u8, 205u8, 135u8, 213u8, 183u8, 48u8,
                            246u8,
                        ],
                    )
                }
                pub fn session_offenders(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::session_offenders::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::session_offenders::Param0,
                    >,
                    types::session_offenders::SessionOffenders,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "SessionOffenders",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            173u8, 83u8, 41u8, 232u8, 177u8, 164u8, 9u8, 255u8, 8u8, 241u8, 60u8,
                            145u8, 22u8, 191u8, 168u8, 218u8, 237u8, 64u8, 142u8, 39u8, 176u8,
                            197u8, 150u8, 228u8, 72u8, 175u8, 205u8, 135u8, 213u8, 183u8, 48u8,
                            246u8,
                        ],
                    )
                }
                pub fn genesis_validators(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::genesis_validators::GenesisValidators,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Validator",
                        "GenesisValidators",
                        (),
                        [
                            93u8, 121u8, 58u8, 106u8, 92u8, 137u8, 108u8, 130u8, 182u8, 208u8,
                            154u8, 106u8, 220u8, 230u8, 12u8, 100u8, 128u8, 120u8, 7u8, 198u8,
                            18u8, 86u8, 240u8, 21u8, 42u8, 142u8, 125u8, 99u8, 244u8, 120u8, 111u8,
                            28u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn bonding_duration(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Validator",
                        "BondingDuration",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn sessions_per_era(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Validator",
                        "SessionsPerEra",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn history_depth(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Validator",
                        "HistoryDepth",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_session::pallet::Error;
        pub type Call = runtime_types::pallet_session::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetKeys {
                    pub keys: set_keys::Keys,
                    pub proof: set_keys::Proof,
                }
                pub mod set_keys {
                    use super::runtime_types;
                    pub type Keys = runtime_types::vrs_runtime::opaque::SessionKeys;
                    pub type Proof = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetKeys {
                    const PALLET: &'static str = "Session";
                    const CALL: &'static str = "set_keys";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct PurgeKeys;
                impl ::subxt_core::blocks::StaticExtrinsic for PurgeKeys {
                    const PALLET: &'static str = "Session";
                    const CALL: &'static str = "purge_keys";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn set_keys(
                    &self,
                    keys: types::set_keys::Keys,
                    proof: types::set_keys::Proof,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetKeys> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Session",
                        "set_keys",
                        types::SetKeys { keys, proof },
                        [
                            112u8, 178u8, 9u8, 24u8, 201u8, 35u8, 145u8, 2u8, 107u8, 143u8, 109u8,
                            170u8, 66u8, 60u8, 9u8, 169u8, 234u8, 176u8, 189u8, 168u8, 162u8,
                            170u8, 199u8, 83u8, 47u8, 227u8, 95u8, 43u8, 147u8, 168u8, 238u8, 5u8,
                        ],
                    )
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::PurgeKeys> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Session",
                        "purge_keys",
                        types::PurgeKeys {},
                        [
                            215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
                            151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
                            67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
                            209u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct NewSession {
                pub session_index: new_session::SessionIndex,
            }
            pub mod new_session {
                use super::runtime_types;
                pub type SessionIndex = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod validators {
                    use super::runtime_types;
                    pub type Validators =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                }
                pub mod current_index {
                    use super::runtime_types;
                    pub type CurrentIndex = ::core::primitive::u32;
                }
                pub mod queued_changed {
                    use super::runtime_types;
                    pub type QueuedChanged = ::core::primitive::bool;
                }
                pub mod queued_keys {
                    use super::runtime_types;
                    pub type QueuedKeys = ::subxt_core::alloc::vec::Vec<(
                        ::subxt_core::utils::AccountId32,
                        runtime_types::vrs_runtime::opaque::SessionKeys,
                    )>;
                }
                pub mod disabled_validators {
                    use super::runtime_types;
                    pub type DisabledValidators =
                        ::subxt_core::alloc::vec::Vec<::core::primitive::u32>;
                }
                pub mod next_keys {
                    use super::runtime_types;
                    pub type NextKeys = runtime_types::vrs_runtime::opaque::SessionKeys;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod key_owner {
                    use super::runtime_types;
                    pub type KeyOwner = ::subxt_core::utils::AccountId32;
                    pub type Param0 = runtime_types::sp_core::crypto::KeyTypeId;
                    pub type Param1 = [::core::primitive::u8];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn validators(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::validators::Validators,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "Validators",
                        (),
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }
                pub fn current_index(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_index::CurrentIndex,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "CurrentIndex",
                        (),
                        [
                            167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
                            135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
                            134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
                            221u8, 230u8,
                        ],
                    )
                }
                pub fn queued_changed(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::queued_changed::QueuedChanged,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "QueuedChanged",
                        (),
                        [
                            184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
                            198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
                            36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
                            153u8,
                        ],
                    )
                }
                pub fn queued_keys(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::queued_keys::QueuedKeys,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "QueuedKeys",
                        (),
                        [
                            120u8, 161u8, 6u8, 54u8, 111u8, 241u8, 196u8, 162u8, 244u8, 142u8, 1u8,
                            69u8, 227u8, 220u8, 171u8, 145u8, 77u8, 164u8, 112u8, 110u8, 84u8,
                            124u8, 120u8, 102u8, 34u8, 31u8, 71u8, 193u8, 13u8, 162u8, 223u8,
                            198u8,
                        ],
                    )
                }
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::disabled_validators::DisabledValidators,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "DisabledValidators",
                        (),
                        [
                            213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
                            36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
                            183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
                        ],
                    )
                }
                pub fn next_keys_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_keys::NextKeys,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "NextKeys",
                        (),
                        [
                            154u8, 60u8, 230u8, 242u8, 241u8, 104u8, 157u8, 128u8, 10u8, 61u8,
                            77u8, 83u8, 88u8, 250u8, 121u8, 250u8, 183u8, 233u8, 35u8, 13u8, 163u8,
                            42u8, 42u8, 79u8, 212u8, 151u8, 209u8, 170u8, 243u8, 210u8, 178u8,
                            34u8,
                        ],
                    )
                }
                pub fn next_keys(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::next_keys::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::next_keys::Param0>,
                    types::next_keys::NextKeys,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "NextKeys",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            154u8, 60u8, 230u8, 242u8, 241u8, 104u8, 157u8, 128u8, 10u8, 61u8,
                            77u8, 83u8, 88u8, 250u8, 121u8, 250u8, 183u8, 233u8, 35u8, 13u8, 163u8,
                            42u8, 42u8, 79u8, 212u8, 151u8, 209u8, 170u8, 243u8, 210u8, 178u8,
                            34u8,
                        ],
                    )
                }
                pub fn key_owner_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key_owner::KeyOwner,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "KeyOwner",
                        (),
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
                pub fn key_owner_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::key_owner::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::key_owner::Param0>,
                    types::key_owner::KeyOwner,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "KeyOwner",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
                pub fn key_owner(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::key_owner::Param0>,
                    _1: impl ::core::borrow::Borrow<types::key_owner::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<types::key_owner::Param0>,
                        ::subxt_core::storage::address::StaticStorageKey<types::key_owner::Param1>,
                    ),
                    types::key_owner::KeyOwner,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Session",
                        "KeyOwner",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ReportEquivocation {
                    pub equivocation_proof:
                        ::subxt_core::alloc::boxed::Box<report_equivocation::EquivocationProof>,
                    pub key_owner_proof: report_equivocation::KeyOwnerProof,
                }
                pub mod report_equivocation {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt_core::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ReportEquivocation {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof: ::subxt_core::alloc::boxed::Box<
                        report_equivocation_unsigned::EquivocationProof,
                    >,
                    pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
                }
                pub mod report_equivocation_unsigned {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt_core::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "report_equivocation_unsigned";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct NoteStalled {
                    pub delay: note_stalled::Delay,
                    pub best_finalized_block_number: note_stalled::BestFinalizedBlockNumber,
                }
                pub mod note_stalled {
                    use super::runtime_types;
                    pub type Delay = ::core::primitive::u32;
                    pub type BestFinalizedBlockNumber = ::core::primitive::u32;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for NoteStalled {
                    const PALLET: &'static str = "Grandpa";
                    const CALL: &'static str = "note_stalled";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: types::report_equivocation::EquivocationProof,
                    key_owner_proof: types::report_equivocation::KeyOwnerProof,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ReportEquivocation>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            187u8, 224u8, 115u8, 5u8, 236u8, 32u8, 180u8, 155u8, 218u8, 109u8,
                            238u8, 253u8, 30u8, 225u8, 4u8, 225u8, 132u8, 232u8, 243u8, 54u8, 56u8,
                            158u8, 94u8, 192u8, 94u8, 206u8, 189u8, 61u8, 14u8, 49u8, 48u8, 131u8,
                        ],
                    )
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: types::report_equivocation_unsigned::EquivocationProof,
                    key_owner_proof: types::report_equivocation_unsigned::KeyOwnerProof,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ReportEquivocationUnsigned>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::subxt_core::alloc::boxed::Box::new(
                                equivocation_proof,
                            ),
                            key_owner_proof,
                        },
                        [
                            98u8, 103u8, 6u8, 54u8, 0u8, 200u8, 166u8, 163u8, 202u8, 45u8, 131u8,
                            226u8, 114u8, 166u8, 237u8, 174u8, 207u8, 214u8, 2u8, 227u8, 32u8,
                            166u8, 47u8, 83u8, 166u8, 239u8, 232u8, 72u8, 224u8, 242u8, 156u8,
                            44u8,
                        ],
                    )
                }
                pub fn note_stalled(
                    &self,
                    delay: types::note_stalled::Delay,
                    best_finalized_block_number: types::note_stalled::BestFinalizedBlockNumber,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::NoteStalled> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Grandpa",
                        "note_stalled",
                        types::NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            158u8, 25u8, 64u8, 114u8, 131u8, 139u8, 227u8, 132u8, 42u8, 107u8,
                            40u8, 249u8, 18u8, 93u8, 254u8, 86u8, 37u8, 67u8, 250u8, 35u8, 241u8,
                            194u8, 209u8, 20u8, 39u8, 75u8, 186u8, 21u8, 48u8, 124u8, 151u8, 31u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct NewAuthorities {
                pub authority_set: new_authorities::AuthoritySet,
            }
            pub mod new_authorities {
                use super::runtime_types;
                pub type AuthoritySet = ::subxt_core::alloc::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>;
            }
            impl ::subxt_core::events::StaticEvent for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Paused;
            impl ::subxt_core::events::StaticEvent for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Resumed;
            impl ::subxt_core::events::StaticEvent for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod state {
                    use super::runtime_types;
                    pub type State =
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                }
                pub mod pending_change {
                    use super::runtime_types;
                    pub type PendingChange =
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                }
                pub mod next_forced {
                    use super::runtime_types;
                    pub type NextForced = ::core::primitive::u32;
                }
                pub mod stalled {
                    use super::runtime_types;
                    pub type Stalled = (::core::primitive::u32, ::core::primitive::u32);
                }
                pub mod current_set_id {
                    use super::runtime_types;
                    pub type CurrentSetId = ::core::primitive::u64;
                }
                pub mod set_id_session {
                    use super::runtime_types;
                    pub type SetIdSession = ::core::primitive::u32;
                    pub type Param0 = ::core::primitive::u64;
                }
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn state(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::state::State,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "State",
                        (),
                        [
                            73u8, 71u8, 112u8, 83u8, 238u8, 75u8, 44u8, 9u8, 180u8, 33u8, 30u8,
                            121u8, 98u8, 96u8, 61u8, 133u8, 16u8, 70u8, 30u8, 249u8, 34u8, 148u8,
                            15u8, 239u8, 164u8, 157u8, 52u8, 27u8, 144u8, 52u8, 223u8, 109u8,
                        ],
                    )
                }
                pub fn pending_change(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::pending_change::PendingChange,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "PendingChange",
                        (),
                        [
                            32u8, 165u8, 141u8, 100u8, 109u8, 66u8, 58u8, 22u8, 118u8, 84u8, 92u8,
                            164u8, 119u8, 130u8, 104u8, 25u8, 244u8, 111u8, 223u8, 54u8, 184u8,
                            95u8, 196u8, 30u8, 244u8, 129u8, 110u8, 127u8, 200u8, 66u8, 226u8,
                            26u8,
                        ],
                    )
                }
                pub fn next_forced(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_forced::NextForced,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "NextForced",
                        (),
                        [
                            3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
                            141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
                            231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
                        ],
                    )
                }
                pub fn stalled(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::stalled::Stalled,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "Stalled",
                        (),
                        [
                            6u8, 81u8, 205u8, 142u8, 195u8, 48u8, 0u8, 247u8, 108u8, 170u8, 10u8,
                            249u8, 72u8, 206u8, 32u8, 103u8, 109u8, 57u8, 51u8, 21u8, 144u8, 204u8,
                            79u8, 8u8, 191u8, 185u8, 38u8, 34u8, 118u8, 223u8, 75u8, 241u8,
                        ],
                    )
                }
                pub fn current_set_id(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::current_set_id::CurrentSetId,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        (),
                        [
                            234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
                            207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
                            246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
                            115u8, 73u8,
                        ],
                    )
                }
                pub fn set_id_session_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::set_id_session::SetIdSession,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        (),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                pub fn set_id_session(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::set_id_session::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::set_id_session::Param0>,
                    types::set_id_session::SetIdSession,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "SetIdSession",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
                pub fn authorities(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authorities::Authorities,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Grandpa",
                        "Authorities",
                        (),
                        [
                            192u8, 157u8, 98u8, 244u8, 104u8, 38u8, 195u8, 114u8, 183u8, 62u8,
                            247u8, 18u8, 31u8, 152u8, 246u8, 206u8, 97u8, 13u8, 118u8, 211u8,
                            104u8, 54u8, 150u8, 152u8, 126u8, 170u8, 228u8, 158u8, 108u8, 129u8,
                            134u8, 44u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_nominators(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxNominators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod historical {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod historical_sessions {
                    use super::runtime_types;
                    pub type HistoricalSessions =
                        (::subxt_core::utils::H256, ::core::primitive::u32);
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod stored_range {
                    use super::runtime_types;
                    pub type StoredRange = (::core::primitive::u32, ::core::primitive::u32);
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn historical_sessions_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::historical_sessions::HistoricalSessions,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Historical",
                        "HistoricalSessions",
                        (),
                        [
                            9u8, 138u8, 247u8, 141u8, 178u8, 146u8, 124u8, 81u8, 162u8, 211u8,
                            205u8, 149u8, 222u8, 254u8, 253u8, 188u8, 170u8, 242u8, 218u8, 41u8,
                            124u8, 178u8, 109u8, 209u8, 163u8, 125u8, 225u8, 206u8, 249u8, 175u8,
                            117u8, 75u8,
                        ],
                    )
                }
                pub fn historical_sessions(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::historical_sessions::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::historical_sessions::Param0,
                    >,
                    types::historical_sessions::HistoricalSessions,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Historical",
                        "HistoricalSessions",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            9u8, 138u8, 247u8, 141u8, 178u8, 146u8, 124u8, 81u8, 162u8, 211u8,
                            205u8, 149u8, 222u8, 254u8, 253u8, 188u8, 170u8, 242u8, 218u8, 41u8,
                            124u8, 178u8, 109u8, 209u8, 163u8, 125u8, 225u8, 206u8, 249u8, 175u8,
                            117u8, 75u8,
                        ],
                    )
                }
                pub fn stored_range(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::stored_range::StoredRange,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Historical",
                        "StoredRange",
                        (),
                        [
                            134u8, 32u8, 250u8, 13u8, 201u8, 25u8, 54u8, 243u8, 231u8, 81u8, 252u8,
                            231u8, 68u8, 217u8, 235u8, 43u8, 22u8, 223u8, 220u8, 133u8, 198u8,
                            218u8, 95u8, 152u8, 189u8, 87u8, 6u8, 228u8, 242u8, 59u8, 232u8, 59u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod im_online {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_im_online::pallet::Error;
        pub type Call = runtime_types::pallet_im_online::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Heartbeat {
                    pub heartbeat: heartbeat::Heartbeat,
                    pub signature: heartbeat::Signature,
                }
                pub mod heartbeat {
                    use super::runtime_types;
                    pub type Heartbeat =
                        runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>;
                    pub type Signature =
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Signature;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Heartbeat {
                    const PALLET: &'static str = "ImOnline";
                    const CALL: &'static str = "heartbeat";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn heartbeat(
                    &self,
                    heartbeat: types::heartbeat::Heartbeat,
                    signature: types::heartbeat::Signature,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Heartbeat> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "ImOnline",
                        "heartbeat",
                        types::Heartbeat {
                            heartbeat,
                            signature,
                        },
                        [
                            166u8, 59u8, 152u8, 129u8, 84u8, 175u8, 182u8, 208u8, 78u8, 54u8,
                            170u8, 108u8, 255u8, 251u8, 152u8, 91u8, 83u8, 127u8, 16u8, 201u8,
                            12u8, 52u8, 160u8, 199u8, 247u8, 146u8, 10u8, 176u8, 4u8, 207u8, 117u8,
                            114u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct HeartbeatReceived {
                pub authority_id: heartbeat_received::AuthorityId,
            }
            pub mod heartbeat_received {
                use super::runtime_types;
                pub type AuthorityId =
                    runtime_types::pallet_im_online::sr25519::app_sr25519::Public;
            }
            impl ::subxt_core::events::StaticEvent for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AllGood;
            impl ::subxt_core::events::StaticEvent for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct SomeOffline {
                pub offline: some_offline::Offline,
            }
            pub mod some_offline {
                use super::runtime_types;
                pub type Offline = ::subxt_core::alloc::vec::Vec<(
                    ::subxt_core::utils::AccountId32,
                    ::core::primitive::u128,
                )>;
            }
            impl ::subxt_core::events::StaticEvent for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod heartbeat_after {
                    use super::runtime_types;
                    pub type HeartbeatAfter = ::core::primitive::u32;
                }
                pub mod keys {
                    use super::runtime_types;
                    pub type Keys =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                        >;
                }
                pub mod received_heartbeats {
                    use super::runtime_types;
                    pub type ReceivedHeartbeats = ::core::primitive::bool;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::core::primitive::u32;
                }
                pub mod authored_blocks {
                    use super::runtime_types;
                    pub type AuthoredBlocks = ::core::primitive::u32;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn heartbeat_after(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::heartbeat_after::HeartbeatAfter,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "HeartbeatAfter",
                        (),
                        [
                            36u8, 179u8, 76u8, 254u8, 3u8, 184u8, 154u8, 142u8, 70u8, 104u8, 44u8,
                            244u8, 39u8, 97u8, 31u8, 31u8, 93u8, 228u8, 185u8, 224u8, 13u8, 160u8,
                            231u8, 210u8, 110u8, 143u8, 116u8, 29u8, 0u8, 215u8, 217u8, 137u8,
                        ],
                    )
                }
                pub fn keys(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::keys::Keys,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "Keys",
                        (),
                        [
                            35u8, 71u8, 73u8, 255u8, 160u8, 250u8, 38u8, 205u8, 32u8, 139u8, 236u8,
                            83u8, 194u8, 12u8, 20u8, 221u8, 114u8, 94u8, 196u8, 246u8, 136u8,
                            175u8, 70u8, 98u8, 91u8, 50u8, 236u8, 131u8, 131u8, 146u8, 150u8,
                            192u8,
                        ],
                    )
                }
                pub fn received_heartbeats_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::received_heartbeats::ReceivedHeartbeats,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "ReceivedHeartbeats",
                        (),
                        [
                            30u8, 155u8, 42u8, 200u8, 223u8, 48u8, 127u8, 31u8, 253u8, 195u8,
                            234u8, 108u8, 64u8, 27u8, 247u8, 17u8, 187u8, 199u8, 41u8, 138u8, 55u8,
                            163u8, 94u8, 226u8, 10u8, 3u8, 132u8, 129u8, 8u8, 138u8, 137u8, 171u8,
                        ],
                    )
                }
                pub fn received_heartbeats_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::received_heartbeats::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::received_heartbeats::Param0,
                    >,
                    types::received_heartbeats::ReceivedHeartbeats,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "ReceivedHeartbeats",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            30u8, 155u8, 42u8, 200u8, 223u8, 48u8, 127u8, 31u8, 253u8, 195u8,
                            234u8, 108u8, 64u8, 27u8, 247u8, 17u8, 187u8, 199u8, 41u8, 138u8, 55u8,
                            163u8, 94u8, 226u8, 10u8, 3u8, 132u8, 129u8, 8u8, 138u8, 137u8, 171u8,
                        ],
                    )
                }
                pub fn received_heartbeats(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::received_heartbeats::Param0>,
                    _1: impl ::core::borrow::Borrow<types::received_heartbeats::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::received_heartbeats::Param0,
                        >,
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::received_heartbeats::Param1,
                        >,
                    ),
                    types::received_heartbeats::ReceivedHeartbeats,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "ReceivedHeartbeats",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            30u8, 155u8, 42u8, 200u8, 223u8, 48u8, 127u8, 31u8, 253u8, 195u8,
                            234u8, 108u8, 64u8, 27u8, 247u8, 17u8, 187u8, 199u8, 41u8, 138u8, 55u8,
                            163u8, 94u8, 226u8, 10u8, 3u8, 132u8, 129u8, 8u8, 138u8, 137u8, 171u8,
                        ],
                    )
                }
                pub fn authored_blocks_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::authored_blocks::AuthoredBlocks,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "AuthoredBlocks",
                        (),
                        [
                            123u8, 76u8, 230u8, 113u8, 65u8, 255u8, 99u8, 79u8, 131u8, 139u8,
                            218u8, 20u8, 174u8, 191u8, 224u8, 67u8, 137u8, 48u8, 146u8, 209u8,
                            148u8, 69u8, 130u8, 9u8, 173u8, 253u8, 206u8, 196u8, 68u8, 160u8,
                            233u8, 126u8,
                        ],
                    )
                }
                pub fn authored_blocks_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::authored_blocks::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::authored_blocks::Param0,
                    >,
                    types::authored_blocks::AuthoredBlocks,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "AuthoredBlocks",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            123u8, 76u8, 230u8, 113u8, 65u8, 255u8, 99u8, 79u8, 131u8, 139u8,
                            218u8, 20u8, 174u8, 191u8, 224u8, 67u8, 137u8, 48u8, 146u8, 209u8,
                            148u8, 69u8, 130u8, 9u8, 173u8, 253u8, 206u8, 196u8, 68u8, 160u8,
                            233u8, 126u8,
                        ],
                    )
                }
                pub fn authored_blocks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::authored_blocks::Param0>,
                    _1: impl ::core::borrow::Borrow<types::authored_blocks::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::authored_blocks::Param0,
                        >,
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::authored_blocks::Param1,
                        >,
                    ),
                    types::authored_blocks::AuthoredBlocks,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "ImOnline",
                        "AuthoredBlocks",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            123u8, 76u8, 230u8, 113u8, 65u8, 255u8, 99u8, 79u8, 131u8, 139u8,
                            218u8, 20u8, 174u8, 191u8, 224u8, 67u8, 137u8, 48u8, 146u8, 209u8,
                            148u8, 69u8, 130u8, 9u8, 173u8, 253u8, 206u8, 196u8, 68u8, 160u8,
                            233u8, 126u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_priority(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "ImOnline",
                        "UnsignedPriority",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TransferAllowDeath {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_allow_death";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Dest =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TransferAll {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "transfer_all";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceUnreserve {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_unreserve";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for UpgradeAccounts {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "upgrade_accounts";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type NewFree = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceSetBalance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_set_balance";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceAdjustTotalIssuance {
                    pub direction: force_adjust_total_issuance::Direction,
                    #[codec(compact)]
                    pub delta: force_adjust_total_issuance::Delta,
                }
                pub mod force_adjust_total_issuance {
                    use super::runtime_types;
                    pub type Direction = runtime_types::pallet_balances::types::AdjustmentDirection;
                    pub type Delta = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceAdjustTotalIssuance {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "force_adjust_total_issuance";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Burn {
                    #[codec(compact)]
                    pub value: burn::Value,
                    pub keep_alive: burn::KeepAlive,
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Burn {
                    const PALLET: &'static str = "Balances";
                    const CALL: &'static str = "burn";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TransferAllowDeath>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath { dest, value },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { dest, value },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TransferAll> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll { dest, keep_alive },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceUnreserve>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve { who, amount },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::UpgradeAccounts>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts { who },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceSetBalance>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance { who, new_free },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
                pub fn force_adjust_total_issuance(
                    &self,
                    direction: types::force_adjust_total_issuance::Direction,
                    delta: types::force_adjust_total_issuance::Delta,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceAdjustTotalIssuance>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "force_adjust_total_issuance",
                        types::ForceAdjustTotalIssuance { direction, delta },
                        [
                            208u8, 134u8, 56u8, 133u8, 232u8, 164u8, 10u8, 213u8, 53u8, 193u8,
                            190u8, 63u8, 236u8, 186u8, 96u8, 122u8, 104u8, 87u8, 173u8, 38u8, 58u8,
                            176u8, 21u8, 78u8, 42u8, 106u8, 46u8, 248u8, 251u8, 190u8, 150u8,
                            202u8,
                        ],
                    )
                }
                pub fn burn(
                    &self,
                    value: types::burn::Value,
                    keep_alive: types::burn::KeepAlive,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Burn> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Balances",
                        "burn",
                        types::Burn { value, keep_alive },
                        [
                            176u8, 64u8, 7u8, 109u8, 16u8, 44u8, 145u8, 125u8, 147u8, 152u8, 130u8,
                            114u8, 221u8, 201u8, 150u8, 162u8, 118u8, 71u8, 52u8, 92u8, 240u8,
                            116u8, 203u8, 98u8, 5u8, 22u8, 43u8, 102u8, 94u8, 208u8, 101u8, 57u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt_core::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt_core::utils::AccountId32;
                pub type To = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt_core::utils::AccountId32;
                pub type To = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt_core::events::StaticEvent for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Minted {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Minted";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Suspended {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Suspended";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Restored {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Restored";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for Upgraded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Upgraded";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Rescinded {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Rescinded";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Locked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Locked";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Unlocked {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unlocked";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct TotalIssuanceForced {
                pub old: total_issuance_forced::Old,
                pub new: total_issuance_forced::New,
            }
            pub mod total_issuance_forced {
                use super::runtime_types;
                pub type Old = ::core::primitive::u128;
                pub type New = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for TotalIssuanceForced {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "TotalIssuanceForced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u128;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u128;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            runtime_types::vrs_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::frame_support::traits::tokens::misc::IdAmount<
                            runtime_types::vrs_runtime::RuntimeFreezeReason,
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn total_issuance(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::total_issuance::TotalIssuance,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "TotalIssuance",
                        (),
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "InactiveIssuance",
                        (),
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }
                pub fn account_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        (),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::account::Param0>,
                    types::account::Account,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Account",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }
                pub fn locks_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::locks::Locks,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        (),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                pub fn locks(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::locks::Param0>,
                    types::locks::Locks,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Locks",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reserves::Reserves,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        (),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                pub fn reserves(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::reserves::Param0>,
                    types::reserves::Reserves,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Reserves",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            112u8, 10u8, 241u8, 77u8, 64u8, 187u8, 106u8, 159u8, 13u8, 153u8,
                            140u8, 178u8, 182u8, 50u8, 1u8, 55u8, 149u8, 92u8, 196u8, 229u8, 170u8,
                            106u8, 193u8, 88u8, 255u8, 244u8, 2u8, 193u8, 62u8, 235u8, 204u8, 91u8,
                        ],
                    )
                }
                pub fn holds_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::holds::Holds,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        (),
                        [
                            37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
                            117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
                            37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
                        ],
                    )
                }
                pub fn holds(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::holds::Param0>,
                    types::holds::Holds,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Holds",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            37u8, 176u8, 2u8, 18u8, 109u8, 26u8, 66u8, 81u8, 28u8, 104u8, 149u8,
                            117u8, 119u8, 114u8, 196u8, 35u8, 172u8, 155u8, 66u8, 195u8, 98u8,
                            37u8, 134u8, 22u8, 106u8, 221u8, 215u8, 97u8, 25u8, 28u8, 21u8, 206u8,
                        ],
                    )
                }
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::freezes::Freezes,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        (),
                        [
                            170u8, 69u8, 116u8, 92u8, 165u8, 14u8, 129u8, 179u8, 165u8, 6u8, 123u8,
                            156u8, 4u8, 30u8, 25u8, 181u8, 191u8, 29u8, 3u8, 92u8, 96u8, 167u8,
                            102u8, 38u8, 128u8, 140u8, 85u8, 248u8, 114u8, 127u8, 128u8, 40u8,
                        ],
                    )
                }
                pub fn freezes(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::freezes::Param0>,
                    types::freezes::Freezes,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Balances",
                        "Freezes",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            170u8, 69u8, 116u8, 92u8, 165u8, 14u8, 129u8, 179u8, 165u8, 6u8, 123u8,
                            156u8, 4u8, 30u8, 25u8, 181u8, 191u8, 29u8, 3u8, 92u8, 96u8, 167u8,
                            102u8, 38u8, 128u8, 140u8, 85u8, 248u8, 114u8, 127u8, 128u8, 40u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn max_locks(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_reserves(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn max_freezes(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct TransactionFeePaid {
                pub who: transaction_fee_paid::Who,
                pub actual_fee: transaction_fee_paid::ActualFee,
                pub tip: transaction_fee_paid::Tip,
            }
            pub mod transaction_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type ActualFee = ::core::primitive::u128;
                pub type Tip = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for TransactionFeePaid {
                const PALLET: &'static str = "TransactionPayment";
                const EVENT: &'static str = "TransactionFeePaid";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod next_fee_multiplier {
                    use super::runtime_types;
                    pub type NextFeeMultiplier =
                        runtime_types::sp_arithmetic::fixed_point::FixedU128;
                }
                pub mod storage_version {
                    use super::runtime_types;
                    pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_fee_multiplier::NextFeeMultiplier,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        (),
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }
                pub fn storage_version(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::storage_version::StorageVersion,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        (),
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u8>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Sudo {
                    pub call: ::subxt_core::alloc::boxed::Box<sudo::Call>,
                }
                pub mod sudo {
                    use super::runtime_types;
                    pub type Call = runtime_types::vrs_runtime::RuntimeCall;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Sudo {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SudoUncheckedWeight {
                    pub call: ::subxt_core::alloc::boxed::Box<sudo_unchecked_weight::Call>,
                    pub weight: sudo_unchecked_weight::Weight,
                }
                pub mod sudo_unchecked_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::vrs_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_unchecked_weight";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetKey {
                    pub new: set_key::New,
                }
                pub mod set_key {
                    use super::runtime_types;
                    pub type New =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "set_key";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SudoAs {
                    pub who: sudo_as::Who,
                    pub call: ::subxt_core::alloc::boxed::Box<sudo_as::Call>,
                }
                pub mod sudo_as {
                    use super::runtime_types;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Call = runtime_types::vrs_runtime::RuntimeCall;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SudoAs {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "sudo_as";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct RemoveKey;
                impl ::subxt_core::blocks::StaticExtrinsic for RemoveKey {
                    const PALLET: &'static str = "Sudo";
                    const CALL: &'static str = "remove_key";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn sudo(
                    &self,
                    call: types::sudo::Call,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Sudo> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            209u8, 202u8, 86u8, 148u8, 157u8, 137u8, 132u8, 28u8, 14u8, 140u8,
                            87u8, 48u8, 39u8, 42u8, 251u8, 238u8, 2u8, 224u8, 145u8, 50u8, 213u8,
                            139u8, 192u8, 208u8, 0u8, 121u8, 62u8, 231u8, 158u8, 143u8, 27u8, 55u8,
                        ],
                    )
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: types::sudo_unchecked_weight::Call,
                    weight: types::sudo_unchecked_weight::Weight,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SudoUncheckedWeight>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::subxt_core::alloc::boxed::Box::new(call),
                            weight,
                        },
                        [
                            87u8, 172u8, 95u8, 144u8, 54u8, 137u8, 218u8, 180u8, 69u8, 106u8,
                            143u8, 126u8, 105u8, 137u8, 77u8, 156u8, 131u8, 56u8, 156u8, 246u8,
                            19u8, 116u8, 233u8, 211u8, 57u8, 59u8, 46u8, 67u8, 212u8, 19u8, 173u8,
                            127u8,
                        ],
                    )
                }
                pub fn set_key(
                    &self,
                    new: types::set_key::New,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetKey> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey { new },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }
                pub fn sudo_as(
                    &self,
                    who: types::sudo_as::Who,
                    call: types::sudo_as::Call,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SudoAs> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::subxt_core::alloc::boxed::Box::new(call),
                        },
                        [
                            111u8, 101u8, 42u8, 80u8, 233u8, 204u8, 109u8, 228u8, 154u8, 69u8,
                            13u8, 229u8, 34u8, 211u8, 251u8, 220u8, 186u8, 13u8, 10u8, 221u8, 83u8,
                            123u8, 147u8, 73u8, 223u8, 245u8, 49u8, 68u8, 45u8, 82u8, 143u8, 146u8,
                        ],
                    )
                }
                pub fn remove_key(
                    &self,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::RemoveKey> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Sudo",
                        "remove_key",
                        types::RemoveKey {},
                        [
                            133u8, 253u8, 54u8, 175u8, 202u8, 239u8, 5u8, 198u8, 180u8, 138u8,
                            25u8, 28u8, 109u8, 40u8, 30u8, 56u8, 126u8, 100u8, 52u8, 205u8, 250u8,
                            191u8, 61u8, 195u8, 172u8, 142u8, 184u8, 239u8, 247u8, 10u8, 211u8,
                            79u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt_core::events::StaticEvent for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct KeyChanged {
                pub old: key_changed::Old,
                pub new: key_changed::New,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type Old = ::core::option::Option<::subxt_core::utils::AccountId32>;
                pub type New = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct KeyRemoved;
            impl ::subxt_core::events::StaticEvent for KeyRemoved {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyRemoved";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt_core::events::StaticEvent for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod key {
                    use super::runtime_types;
                    pub type Key = ::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn key(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::key::Key,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Sudo",
                        "Key",
                        (),
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod offences {
        use super::root_mod;
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Offence {
                pub kind: offence::Kind,
                pub timeslot: offence::Timeslot,
            }
            pub mod offence {
                use super::runtime_types;
                pub type Kind = [::core::primitive::u8; 16usize];
                pub type Timeslot = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt_core::events::StaticEvent for Offence {
                const PALLET: &'static str = "Offences";
                const EVENT: &'static str = "Offence";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod reports {
                    use super::runtime_types;
                    pub type Reports = runtime_types::sp_staking::offence::OffenceDetails<
                        ::subxt_core::utils::AccountId32,
                        (::subxt_core::utils::AccountId32, ::core::primitive::u128),
                    >;
                    pub type Param0 = ::subxt_core::utils::H256;
                }
                pub mod concurrent_reports_index {
                    use super::runtime_types;
                    pub type ConcurrentReportsIndex =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::H256>;
                    pub type Param0 = [::core::primitive::u8; 16usize];
                    pub type Param1 = [::core::primitive::u8];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn reports_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::reports::Reports,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Offences",
                        "Reports",
                        (),
                        [
                            171u8, 112u8, 78u8, 81u8, 159u8, 18u8, 37u8, 11u8, 198u8, 246u8, 247u8,
                            185u8, 176u8, 78u8, 37u8, 236u8, 168u8, 20u8, 226u8, 36u8, 72u8, 230u8,
                            91u8, 184u8, 247u8, 156u8, 100u8, 82u8, 30u8, 12u8, 108u8, 233u8,
                        ],
                    )
                }
                pub fn reports(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::reports::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::reports::Param0>,
                    types::reports::Reports,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Offences",
                        "Reports",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            171u8, 112u8, 78u8, 81u8, 159u8, 18u8, 37u8, 11u8, 198u8, 246u8, 247u8,
                            185u8, 176u8, 78u8, 37u8, 236u8, 168u8, 20u8, 226u8, 36u8, 72u8, 230u8,
                            91u8, 184u8, 247u8, 156u8, 100u8, 82u8, 30u8, 12u8, 108u8, 233u8,
                        ],
                    )
                }
                pub fn concurrent_reports_index_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::concurrent_reports_index::ConcurrentReportsIndex,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Offences",
                        "ConcurrentReportsIndex",
                        (),
                        [
                            170u8, 186u8, 72u8, 29u8, 251u8, 38u8, 193u8, 195u8, 109u8, 86u8, 0u8,
                            241u8, 20u8, 235u8, 108u8, 126u8, 215u8, 82u8, 73u8, 113u8, 199u8,
                            138u8, 24u8, 58u8, 216u8, 72u8, 221u8, 232u8, 252u8, 244u8, 96u8,
                            247u8,
                        ],
                    )
                }
                pub fn concurrent_reports_index_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::concurrent_reports_index::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::concurrent_reports_index::Param0,
                    >,
                    types::concurrent_reports_index::ConcurrentReportsIndex,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Offences",
                        "ConcurrentReportsIndex",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            170u8, 186u8, 72u8, 29u8, 251u8, 38u8, 193u8, 195u8, 109u8, 86u8, 0u8,
                            241u8, 20u8, 235u8, 108u8, 126u8, 215u8, 82u8, 73u8, 113u8, 199u8,
                            138u8, 24u8, 58u8, 216u8, 72u8, 221u8, 232u8, 252u8, 244u8, 96u8,
                            247u8,
                        ],
                    )
                }
                pub fn concurrent_reports_index(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::concurrent_reports_index::Param0>,
                    _1: impl ::core::borrow::Borrow<types::concurrent_reports_index::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::concurrent_reports_index::Param0,
                        >,
                        ::subxt_core::storage::address::StaticStorageKey<
                            types::concurrent_reports_index::Param1,
                        >,
                    ),
                    types::concurrent_reports_index::ConcurrentReportsIndex,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Offences",
                        "ConcurrentReportsIndex",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            170u8, 186u8, 72u8, 29u8, 251u8, 38u8, 193u8, 195u8, 109u8, 86u8, 0u8,
                            241u8, 20u8, 235u8, 108u8, 126u8, 215u8, 82u8, 73u8, 113u8, 199u8,
                            138u8, 24u8, 58u8, 216u8, 72u8, 221u8, 232u8, 252u8, 244u8, 96u8,
                            247u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod nucleus {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_nucleus::pallet::Error;
        pub type Call = runtime_types::pallet_nucleus::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct CreateNucleus {
                    pub name: create_nucleus::Name,
                    pub energy: create_nucleus::Energy,
                    pub capacity: create_nucleus::Capacity,
                    pub a2a_compatible: create_nucleus::A2aCompatible,
                }
                pub mod create_nucleus {
                    use super::runtime_types;
                    pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Energy = ::core::option::Option<::core::primitive::u128>;
                    pub type Capacity = ::core::primitive::u8;
                    pub type A2aCompatible = ::core::primitive::bool;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for CreateNucleus {
                    const PALLET: &'static str = "Nucleus";
                    const CALL: &'static str = "create_nucleus";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct UploadNucleusWasm {
                    pub nucleus_id: upload_nucleus_wasm::NucleusId,
                    pub to: upload_nucleus_wasm::To,
                    pub hash: upload_nucleus_wasm::Hash,
                }
                pub mod upload_nucleus_wasm {
                    use super::runtime_types;
                    pub type NucleusId = ::subxt_core::utils::AccountId32;
                    pub type To = runtime_types::sp_core::OpaquePeerId;
                    pub type Hash = ::subxt_core::utils::H256;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for UploadNucleusWasm {
                    const PALLET: &'static str = "Nucleus";
                    const CALL: &'static str = "upload_nucleus_wasm";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Register {
                    pub nucleus_id: register::NucleusId,
                    pub signature: register::Signature,
                }
                pub mod register {
                    use super::runtime_types;
                    pub type NucleusId = ::subxt_core::utils::AccountId32;
                    pub type Signature = runtime_types::sp_core::sr25519::vrf::VrfSignature;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Register {
                    const PALLET: &'static str = "Nucleus";
                    const CALL: &'static str = "register";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SubmitWork {
                    pub nucleus_id: submit_work::NucleusId,
                }
                pub mod submit_work {
                    use super::runtime_types;
                    pub type NucleusId = ::subxt_core::utils::AccountId32;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SubmitWork {
                    const PALLET: &'static str = "Nucleus";
                    const CALL: &'static str = "submit_work";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create_nucleus(
                    &self,
                    name: types::create_nucleus::Name,
                    energy: types::create_nucleus::Energy,
                    capacity: types::create_nucleus::Capacity,
                    a2a_compatible: types::create_nucleus::A2aCompatible,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::CreateNucleus>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Nucleus",
                        "create_nucleus",
                        types::CreateNucleus {
                            name,
                            energy,
                            capacity,
                            a2a_compatible,
                        },
                        [
                            137u8, 221u8, 252u8, 224u8, 169u8, 113u8, 255u8, 253u8, 217u8, 99u8,
                            79u8, 62u8, 180u8, 54u8, 101u8, 23u8, 125u8, 242u8, 141u8, 150u8,
                            169u8, 82u8, 101u8, 62u8, 146u8, 157u8, 175u8, 201u8, 71u8, 98u8,
                            160u8, 109u8,
                        ],
                    )
                }
                pub fn upload_nucleus_wasm(
                    &self,
                    nucleus_id: types::upload_nucleus_wasm::NucleusId,
                    to: types::upload_nucleus_wasm::To,
                    hash: types::upload_nucleus_wasm::Hash,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::UploadNucleusWasm>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Nucleus",
                        "upload_nucleus_wasm",
                        types::UploadNucleusWasm {
                            nucleus_id,
                            to,
                            hash,
                        },
                        [
                            73u8, 141u8, 151u8, 40u8, 193u8, 230u8, 42u8, 117u8, 82u8, 17u8, 254u8,
                            147u8, 14u8, 125u8, 56u8, 135u8, 136u8, 193u8, 7u8, 58u8, 54u8, 198u8,
                            120u8, 166u8, 254u8, 98u8, 111u8, 18u8, 105u8, 165u8, 175u8, 61u8,
                        ],
                    )
                }
                pub fn register(
                    &self,
                    nucleus_id: types::register::NucleusId,
                    signature: types::register::Signature,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Register> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Nucleus",
                        "register",
                        types::Register {
                            nucleus_id,
                            signature,
                        },
                        [
                            14u8, 184u8, 48u8, 27u8, 199u8, 115u8, 45u8, 147u8, 217u8, 239u8, 9u8,
                            179u8, 130u8, 208u8, 35u8, 180u8, 101u8, 30u8, 189u8, 44u8, 189u8,
                            96u8, 170u8, 13u8, 12u8, 209u8, 153u8, 246u8, 234u8, 48u8, 16u8, 153u8,
                        ],
                    )
                }
                pub fn submit_work(
                    &self,
                    nucleus_id: types::submit_work::NucleusId,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SubmitWork> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Nucleus",
                        "submit_work",
                        types::SubmitWork { nucleus_id },
                        [
                            80u8, 173u8, 190u8, 138u8, 60u8, 148u8, 221u8, 64u8, 236u8, 230u8,
                            229u8, 19u8, 43u8, 31u8, 76u8, 101u8, 183u8, 51u8, 212u8, 49u8, 160u8,
                            77u8, 41u8, 162u8, 27u8, 56u8, 140u8, 17u8, 164u8, 126u8, 230u8, 82u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_nucleus::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct NucleusCreated {
                pub id: nucleus_created::Id,
                pub name: nucleus_created::Name,
                pub manager: nucleus_created::Manager,
                pub energy: nucleus_created::Energy,
                pub capacity: nucleus_created::Capacity,
                pub public_input: nucleus_created::PublicInput,
            }
            pub mod nucleus_created {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
                pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Manager = ::subxt_core::utils::AccountId32;
                pub type Energy = ::core::primitive::u128;
                pub type Capacity = ::core::primitive::u8;
                pub type PublicInput = ::subxt_core::utils::H256;
            }
            impl ::subxt_core::events::StaticEvent for NucleusCreated {
                const PALLET: &'static str = "Nucleus";
                const EVENT: &'static str = "NucleusCreated";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct NucleusUpgraded {
                pub id: nucleus_upgraded::Id,
                pub wasm_hash: nucleus_upgraded::WasmHash,
                pub wasm_version: nucleus_upgraded::WasmVersion,
                pub wasm_location: nucleus_upgraded::WasmLocation,
            }
            pub mod nucleus_upgraded {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
                pub type WasmHash = ::subxt_core::utils::H256;
                pub type WasmVersion = ::core::primitive::u32;
                pub type WasmLocation = runtime_types::sp_core::OpaquePeerId;
            }
            impl ::subxt_core::events::StaticEvent for NucleusUpgraded {
                const PALLET: &'static str = "Nucleus";
                const EVENT: &'static str = "NucleusUpgraded";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct InstanceRegistered {
                pub id: instance_registered::Id,
                pub controller: instance_registered::Controller,
                pub node_id: instance_registered::NodeId,
            }
            pub mod instance_registered {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
                pub type Controller = ::subxt_core::utils::AccountId32;
                pub type NodeId = ::core::option::Option<runtime_types::sp_core::OpaquePeerId>;
            }
            impl ::subxt_core::events::StaticEvent for InstanceRegistered {
                const PALLET: &'static str = "Nucleus";
                const EVENT: &'static str = "InstanceRegistered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod nuclei {
                    use super::runtime_types;
                    pub type Nuclei = runtime_types::pallet_nucleus::pallet::NucleusEquation<
                        ::subxt_core::utils::AccountId32,
                        ::subxt_core::utils::H256,
                        runtime_types::sp_core::OpaquePeerId,
                    >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod node_controllers {
                    use super::runtime_types;
                    pub type NodeControllers = runtime_types::sp_core::OpaquePeerId;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod instances {
                    use super::runtime_types;
                    pub type Instances =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
                pub mod forced_instances {
                    use super::runtime_types;
                    pub type ForcedInstances =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                }
                pub mod on_creation_nuclei {
                    use super::runtime_types;
                    pub type OnCreationNuclei =
                        ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod registry_submissions {
                    use super::runtime_types;
                    pub type RegistrySubmissions =
                        runtime_types::pallet_nucleus::pallet::NucleusChallenge<
                            ::subxt_core::utils::AccountId32,
                            ::subxt_core::utils::H256,
                        >;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn nuclei_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::nuclei::Nuclei,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "Nuclei",
                        (),
                        [
                            228u8, 15u8, 104u8, 175u8, 27u8, 131u8, 20u8, 210u8, 119u8, 35u8, 67u8,
                            61u8, 228u8, 245u8, 103u8, 26u8, 86u8, 36u8, 180u8, 21u8, 169u8, 61u8,
                            89u8, 51u8, 95u8, 174u8, 12u8, 32u8, 53u8, 205u8, 249u8, 75u8,
                        ],
                    )
                }
                pub fn nuclei(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::nuclei::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::nuclei::Param0>,
                    types::nuclei::Nuclei,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "Nuclei",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            228u8, 15u8, 104u8, 175u8, 27u8, 131u8, 20u8, 210u8, 119u8, 35u8, 67u8,
                            61u8, 228u8, 245u8, 103u8, 26u8, 86u8, 36u8, 180u8, 21u8, 169u8, 61u8,
                            89u8, 51u8, 95u8, 174u8, 12u8, 32u8, 53u8, 205u8, 249u8, 75u8,
                        ],
                    )
                }
                pub fn node_controllers_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::node_controllers::NodeControllers,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "NodeControllers",
                        (),
                        [
                            72u8, 168u8, 225u8, 205u8, 103u8, 194u8, 159u8, 213u8, 249u8, 238u8,
                            41u8, 253u8, 171u8, 177u8, 75u8, 95u8, 221u8, 180u8, 204u8, 208u8,
                            93u8, 3u8, 30u8, 43u8, 238u8, 144u8, 94u8, 14u8, 102u8, 86u8, 153u8,
                            181u8,
                        ],
                    )
                }
                pub fn node_controllers(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::node_controllers::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::node_controllers::Param0,
                    >,
                    types::node_controllers::NodeControllers,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "NodeControllers",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            72u8, 168u8, 225u8, 205u8, 103u8, 194u8, 159u8, 213u8, 249u8, 238u8,
                            41u8, 253u8, 171u8, 177u8, 75u8, 95u8, 221u8, 180u8, 204u8, 208u8,
                            93u8, 3u8, 30u8, 43u8, 238u8, 144u8, 94u8, 14u8, 102u8, 86u8, 153u8,
                            181u8,
                        ],
                    )
                }
                pub fn instances_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::instances::Instances,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "Instances",
                        (),
                        [
                            93u8, 226u8, 25u8, 95u8, 180u8, 191u8, 1u8, 35u8, 199u8, 241u8, 132u8,
                            88u8, 120u8, 65u8, 244u8, 199u8, 139u8, 249u8, 41u8, 1u8, 141u8, 14u8,
                            95u8, 175u8, 237u8, 222u8, 191u8, 34u8, 213u8, 210u8, 94u8, 88u8,
                        ],
                    )
                }
                pub fn instances(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::instances::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::instances::Param0>,
                    types::instances::Instances,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "Instances",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            93u8, 226u8, 25u8, 95u8, 180u8, 191u8, 1u8, 35u8, 199u8, 241u8, 132u8,
                            88u8, 120u8, 65u8, 244u8, 199u8, 139u8, 249u8, 41u8, 1u8, 141u8, 14u8,
                            95u8, 175u8, 237u8, 222u8, 191u8, 34u8, 213u8, 210u8, 94u8, 88u8,
                        ],
                    )
                }
                pub fn forced_instances(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::forced_instances::ForcedInstances,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "ForcedInstances",
                        (),
                        [
                            235u8, 79u8, 112u8, 79u8, 223u8, 157u8, 46u8, 18u8, 195u8, 249u8,
                            182u8, 223u8, 215u8, 149u8, 148u8, 245u8, 235u8, 36u8, 178u8, 231u8,
                            254u8, 99u8, 112u8, 41u8, 160u8, 172u8, 184u8, 213u8, 80u8, 36u8, 40u8,
                            255u8,
                        ],
                    )
                }
                pub fn on_creation_nuclei_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::on_creation_nuclei::OnCreationNuclei,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "OnCreationNuclei",
                        (),
                        [
                            68u8, 239u8, 23u8, 253u8, 15u8, 226u8, 83u8, 6u8, 190u8, 131u8, 177u8,
                            157u8, 223u8, 192u8, 82u8, 50u8, 78u8, 237u8, 91u8, 61u8, 130u8, 21u8,
                            79u8, 56u8, 125u8, 223u8, 94u8, 57u8, 150u8, 216u8, 127u8, 188u8,
                        ],
                    )
                }
                pub fn on_creation_nuclei(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::on_creation_nuclei::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::on_creation_nuclei::Param0,
                    >,
                    types::on_creation_nuclei::OnCreationNuclei,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "OnCreationNuclei",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            68u8, 239u8, 23u8, 253u8, 15u8, 226u8, 83u8, 6u8, 190u8, 131u8, 177u8,
                            157u8, 223u8, 192u8, 82u8, 50u8, 78u8, 237u8, 91u8, 61u8, 130u8, 21u8,
                            79u8, 56u8, 125u8, 223u8, 94u8, 57u8, 150u8, 216u8, 127u8, 188u8,
                        ],
                    )
                }
                pub fn registry_submissions_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::registry_submissions::RegistrySubmissions,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "RegistrySubmissions",
                        (),
                        [
                            82u8, 51u8, 109u8, 150u8, 245u8, 186u8, 132u8, 128u8, 78u8, 185u8,
                            52u8, 41u8, 105u8, 179u8, 152u8, 117u8, 126u8, 75u8, 200u8, 101u8,
                            77u8, 94u8, 79u8, 171u8, 12u8, 1u8, 202u8, 50u8, 24u8, 82u8, 17u8, 3u8,
                        ],
                    )
                }
                pub fn registry_submissions(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::registry_submissions::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<
                        types::registry_submissions::Param0,
                    >,
                    types::registry_submissions::RegistrySubmissions,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Nucleus",
                        "RegistrySubmissions",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            82u8, 51u8, 109u8, 150u8, 245u8, 186u8, 132u8, 128u8, 78u8, 185u8,
                            52u8, 41u8, 105u8, 179u8, 152u8, 117u8, 126u8, 75u8, 200u8, 101u8,
                            77u8, 94u8, 79u8, 171u8, 12u8, 1u8, 202u8, 50u8, 24u8, 82u8, 17u8, 3u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn fee_asset_id(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<
                    runtime_types::vrs_primitives::AssetId,
                > {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Nucleus",
                        "FeeAssetId",
                        [
                            81u8, 164u8, 181u8, 74u8, 47u8, 81u8, 8u8, 148u8, 228u8, 167u8, 47u8,
                            161u8, 126u8, 164u8, 87u8, 84u8, 31u8, 109u8, 217u8, 254u8, 240u8, 3u8,
                            129u8, 252u8, 10u8, 99u8, 152u8, 188u8, 250u8, 60u8, 118u8, 55u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod assets {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_assets::pallet::Error;
        pub type Call = runtime_types::pallet_assets::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Create {
                    pub id: create::Id,
                    pub admin: create::Admin,
                    pub min_balance: create::MinBalance,
                }
                pub mod create {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Admin =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Create {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "create";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceCreate {
                    pub id: force_create::Id,
                    pub owner: force_create::Owner,
                    pub is_sufficient: force_create::IsSufficient,
                    #[codec(compact)]
                    pub min_balance: force_create::MinBalance,
                }
                pub mod force_create {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Owner =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type IsSufficient = ::core::primitive::bool;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceCreate {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_create";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct StartDestroy {
                    pub id: start_destroy::Id,
                }
                pub mod start_destroy {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for StartDestroy {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "start_destroy";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct DestroyAccounts {
                    pub id: destroy_accounts::Id,
                }
                pub mod destroy_accounts {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for DestroyAccounts {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "destroy_accounts";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct DestroyApprovals {
                    pub id: destroy_approvals::Id,
                }
                pub mod destroy_approvals {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for DestroyApprovals {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "destroy_approvals";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct FinishDestroy {
                    pub id: finish_destroy::Id,
                }
                pub mod finish_destroy {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for FinishDestroy {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "finish_destroy";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Mint {
                    pub id: mint::Id,
                    pub beneficiary: mint::Beneficiary,
                    #[codec(compact)]
                    pub amount: mint::Amount,
                }
                pub mod mint {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Beneficiary =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Mint {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "mint";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Burn {
                    pub id: burn::Id,
                    pub who: burn::Who,
                    #[codec(compact)]
                    pub amount: burn::Amount,
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Burn {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "burn";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Transfer {
                    pub id: transfer::Id,
                    pub target: transfer::Target,
                    #[codec(compact)]
                    pub amount: transfer::Amount,
                }
                pub mod transfer {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Target =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Transfer {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TransferKeepAlive {
                    pub id: transfer_keep_alive::Id,
                    pub target: transfer_keep_alive::Target,
                    #[codec(compact)]
                    pub amount: transfer_keep_alive::Amount,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Target =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TransferKeepAlive {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer_keep_alive";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceTransfer {
                    pub id: force_transfer::Id,
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub amount: force_transfer::Amount,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Source =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Dest =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceTransfer {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_transfer";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Freeze {
                    pub id: freeze::Id,
                    pub who: freeze::Who,
                }
                pub mod freeze {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Freeze {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "freeze";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Thaw {
                    pub id: thaw::Id,
                    pub who: thaw::Who,
                }
                pub mod thaw {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Thaw {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "thaw";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct FreezeAsset {
                    pub id: freeze_asset::Id,
                }
                pub mod freeze_asset {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for FreezeAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "freeze_asset";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ThawAsset {
                    pub id: thaw_asset::Id,
                }
                pub mod thaw_asset {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ThawAsset {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "thaw_asset";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TransferOwnership {
                    pub id: transfer_ownership::Id,
                    pub owner: transfer_ownership::Owner,
                }
                pub mod transfer_ownership {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Owner =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TransferOwnership {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer_ownership";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetTeam {
                    pub id: set_team::Id,
                    pub issuer: set_team::Issuer,
                    pub admin: set_team::Admin,
                    pub freezer: set_team::Freezer,
                }
                pub mod set_team {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Issuer =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Admin =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Freezer =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetTeam {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "set_team";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetMetadata {
                    pub id: set_metadata::Id,
                    pub name: set_metadata::Name,
                    pub symbol: set_metadata::Symbol,
                    pub decimals: set_metadata::Decimals,
                }
                pub mod set_metadata {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Symbol = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Decimals = ::core::primitive::u8;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "set_metadata";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ClearMetadata {
                    pub id: clear_metadata::Id,
                }
                pub mod clear_metadata {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ClearMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "clear_metadata";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceSetMetadata {
                    pub id: force_set_metadata::Id,
                    pub name: force_set_metadata::Name,
                    pub symbol: force_set_metadata::Symbol,
                    pub decimals: force_set_metadata::Decimals,
                    pub is_frozen: force_set_metadata::IsFrozen,
                }
                pub mod force_set_metadata {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Symbol = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type Decimals = ::core::primitive::u8;
                    pub type IsFrozen = ::core::primitive::bool;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceSetMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_set_metadata";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceClearMetadata {
                    pub id: force_clear_metadata::Id,
                }
                pub mod force_clear_metadata {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceClearMetadata {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_clear_metadata";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceAssetStatus {
                    pub id: force_asset_status::Id,
                    pub owner: force_asset_status::Owner,
                    pub issuer: force_asset_status::Issuer,
                    pub admin: force_asset_status::Admin,
                    pub freezer: force_asset_status::Freezer,
                    #[codec(compact)]
                    pub min_balance: force_asset_status::MinBalance,
                    pub is_sufficient: force_asset_status::IsSufficient,
                    pub is_frozen: force_asset_status::IsFrozen,
                }
                pub mod force_asset_status {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Owner =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Issuer =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Admin =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Freezer =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type MinBalance = ::core::primitive::u128;
                    pub type IsSufficient = ::core::primitive::bool;
                    pub type IsFrozen = ::core::primitive::bool;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceAssetStatus {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_asset_status";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ApproveTransfer {
                    pub id: approve_transfer::Id,
                    pub delegate: approve_transfer::Delegate,
                    #[codec(compact)]
                    pub amount: approve_transfer::Amount,
                }
                pub mod approve_transfer {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Delegate =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ApproveTransfer {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "approve_transfer";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct CancelApproval {
                    pub id: cancel_approval::Id,
                    pub delegate: cancel_approval::Delegate,
                }
                pub mod cancel_approval {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Delegate =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for CancelApproval {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "cancel_approval";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ForceCancelApproval {
                    pub id: force_cancel_approval::Id,
                    pub owner: force_cancel_approval::Owner,
                    pub delegate: force_cancel_approval::Delegate,
                }
                pub mod force_cancel_approval {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Owner =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Delegate =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for ForceCancelApproval {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "force_cancel_approval";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TransferApproved {
                    pub id: transfer_approved::Id,
                    pub owner: transfer_approved::Owner,
                    pub destination: transfer_approved::Destination,
                    #[codec(compact)]
                    pub amount: transfer_approved::Amount,
                }
                pub mod transfer_approved {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Owner =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Destination =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TransferApproved {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "transfer_approved";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Touch {
                    pub id: touch::Id,
                }
                pub mod touch {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Touch {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "touch";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Refund {
                    pub id: refund::Id,
                    pub allow_burn: refund::AllowBurn,
                }
                pub mod refund {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type AllowBurn = ::core::primitive::bool;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Refund {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "refund";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SetMinBalance {
                    pub id: set_min_balance::Id,
                    pub min_balance: set_min_balance::MinBalance,
                }
                pub mod set_min_balance {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for SetMinBalance {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "set_min_balance";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct TouchOther {
                    pub id: touch_other::Id,
                    pub who: touch_other::Who,
                }
                pub mod touch_other {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for TouchOther {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "touch_other";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct RefundOther {
                    pub id: refund_other::Id,
                    pub who: refund_other::Who,
                }
                pub mod refund_other {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for RefundOther {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "refund_other";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Block {
                    pub id: block::Id,
                    pub who: block::Who,
                }
                pub mod block {
                    use super::runtime_types;
                    pub type Id = runtime_types::vrs_primitives::AssetId;
                    pub type Who =
                        ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Block {
                    const PALLET: &'static str = "Assets";
                    const CALL: &'static str = "block";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create(
                    &self,
                    id: types::create::Id,
                    admin: types::create::Admin,
                    min_balance: types::create::MinBalance,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Create> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "create",
                        types::Create {
                            id,
                            admin,
                            min_balance,
                        },
                        [
                            231u8, 4u8, 206u8, 44u8, 181u8, 191u8, 167u8, 65u8, 2u8, 30u8, 251u8,
                            145u8, 247u8, 51u8, 76u8, 31u8, 243u8, 8u8, 17u8, 104u8, 213u8, 243u8,
                            61u8, 241u8, 124u8, 105u8, 14u8, 66u8, 127u8, 184u8, 20u8, 166u8,
                        ],
                    )
                }
                pub fn force_create(
                    &self,
                    id: types::force_create::Id,
                    owner: types::force_create::Owner,
                    is_sufficient: types::force_create::IsSufficient,
                    min_balance: types::force_create::MinBalance,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceCreate> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_create",
                        types::ForceCreate {
                            id,
                            owner,
                            is_sufficient,
                            min_balance,
                        },
                        [
                            108u8, 6u8, 230u8, 20u8, 55u8, 233u8, 203u8, 216u8, 190u8, 168u8, 5u8,
                            173u8, 1u8, 82u8, 33u8, 250u8, 128u8, 179u8, 186u8, 212u8, 245u8,
                            200u8, 92u8, 216u8, 183u8, 43u8, 232u8, 119u8, 105u8, 7u8, 127u8,
                            157u8,
                        ],
                    )
                }
                pub fn start_destroy(
                    &self,
                    id: types::start_destroy::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::StartDestroy> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "start_destroy",
                        types::StartDestroy { id },
                        [
                            48u8, 212u8, 199u8, 237u8, 192u8, 94u8, 213u8, 169u8, 88u8, 87u8,
                            247u8, 215u8, 214u8, 180u8, 2u8, 189u8, 147u8, 40u8, 222u8, 128u8,
                            237u8, 55u8, 155u8, 9u8, 11u8, 104u8, 6u8, 37u8, 178u8, 28u8, 54u8,
                            183u8,
                        ],
                    )
                }
                pub fn destroy_accounts(
                    &self,
                    id: types::destroy_accounts::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::DestroyAccounts>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "destroy_accounts",
                        types::DestroyAccounts { id },
                        [
                            94u8, 26u8, 5u8, 157u8, 215u8, 87u8, 13u8, 102u8, 204u8, 5u8, 94u8,
                            67u8, 245u8, 130u8, 235u8, 118u8, 84u8, 209u8, 232u8, 225u8, 249u8,
                            170u8, 119u8, 7u8, 176u8, 179u8, 246u8, 30u8, 0u8, 117u8, 196u8, 185u8,
                        ],
                    )
                }
                pub fn destroy_approvals(
                    &self,
                    id: types::destroy_approvals::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::DestroyApprovals>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "destroy_approvals",
                        types::DestroyApprovals { id },
                        [
                            170u8, 11u8, 196u8, 47u8, 181u8, 80u8, 121u8, 201u8, 199u8, 164u8,
                            138u8, 151u8, 74u8, 144u8, 253u8, 41u8, 20u8, 121u8, 25u8, 221u8,
                            162u8, 24u8, 89u8, 247u8, 174u8, 197u8, 243u8, 118u8, 9u8, 161u8,
                            122u8, 11u8,
                        ],
                    )
                }
                pub fn finish_destroy(
                    &self,
                    id: types::finish_destroy::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::FinishDestroy>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "finish_destroy",
                        types::FinishDestroy { id },
                        [
                            209u8, 165u8, 192u8, 152u8, 59u8, 254u8, 16u8, 103u8, 136u8, 144u8,
                            186u8, 186u8, 226u8, 233u8, 22u8, 175u8, 222u8, 30u8, 168u8, 172u8,
                            28u8, 192u8, 20u8, 34u8, 176u8, 246u8, 114u8, 17u8, 30u8, 65u8, 230u8,
                            46u8,
                        ],
                    )
                }
                pub fn mint(
                    &self,
                    id: types::mint::Id,
                    beneficiary: types::mint::Beneficiary,
                    amount: types::mint::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Mint> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "mint",
                        types::Mint {
                            id,
                            beneficiary,
                            amount,
                        },
                        [
                            210u8, 185u8, 73u8, 147u8, 46u8, 114u8, 45u8, 18u8, 213u8, 196u8,
                            117u8, 100u8, 109u8, 41u8, 206u8, 46u8, 131u8, 20u8, 23u8, 185u8,
                            122u8, 61u8, 125u8, 234u8, 226u8, 45u8, 244u8, 183u8, 168u8, 232u8,
                            215u8, 88u8,
                        ],
                    )
                }
                pub fn burn(
                    &self,
                    id: types::burn::Id,
                    who: types::burn::Who,
                    amount: types::burn::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Burn> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "burn",
                        types::Burn { id, who, amount },
                        [
                            148u8, 72u8, 205u8, 48u8, 153u8, 103u8, 214u8, 112u8, 103u8, 159u8,
                            68u8, 3u8, 117u8, 208u8, 199u8, 110u8, 111u8, 12u8, 218u8, 203u8,
                            181u8, 147u8, 36u8, 41u8, 59u8, 26u8, 248u8, 168u8, 180u8, 199u8,
                            126u8, 194u8,
                        ],
                    )
                }
                pub fn transfer(
                    &self,
                    id: types::transfer::Id,
                    target: types::transfer::Target,
                    amount: types::transfer::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Transfer> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer",
                        types::Transfer { id, target, amount },
                        [
                            52u8, 84u8, 202u8, 14u8, 130u8, 19u8, 0u8, 200u8, 85u8, 50u8, 52u8,
                            249u8, 215u8, 179u8, 224u8, 253u8, 13u8, 158u8, 40u8, 71u8, 253u8,
                            65u8, 103u8, 197u8, 103u8, 80u8, 138u8, 163u8, 137u8, 89u8, 194u8,
                            154u8,
                        ],
                    )
                }
                pub fn transfer_keep_alive(
                    &self,
                    id: types::transfer_keep_alive::Id,
                    target: types::transfer_keep_alive::Target,
                    amount: types::transfer_keep_alive::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TransferKeepAlive>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer_keep_alive",
                        types::TransferKeepAlive { id, target, amount },
                        [
                            199u8, 197u8, 198u8, 89u8, 82u8, 12u8, 150u8, 223u8, 231u8, 108u8,
                            141u8, 64u8, 93u8, 117u8, 39u8, 9u8, 115u8, 58u8, 103u8, 231u8, 189u8,
                            87u8, 68u8, 153u8, 233u8, 106u8, 254u8, 125u8, 121u8, 183u8, 0u8,
                            152u8,
                        ],
                    )
                }
                pub fn force_transfer(
                    &self,
                    id: types::force_transfer::Id,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    amount: types::force_transfer::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceTransfer>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_transfer",
                        types::ForceTransfer {
                            id,
                            source,
                            dest,
                            amount,
                        },
                        [
                            11u8, 115u8, 194u8, 93u8, 73u8, 216u8, 191u8, 59u8, 179u8, 165u8,
                            248u8, 255u8, 27u8, 133u8, 210u8, 198u8, 226u8, 55u8, 21u8, 127u8,
                            30u8, 239u8, 107u8, 188u8, 133u8, 141u8, 40u8, 143u8, 226u8, 104u8,
                            63u8, 210u8,
                        ],
                    )
                }
                pub fn freeze(
                    &self,
                    id: types::freeze::Id,
                    who: types::freeze::Who,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Freeze> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "freeze",
                        types::Freeze { id, who },
                        [
                            253u8, 118u8, 252u8, 124u8, 73u8, 101u8, 13u8, 241u8, 21u8, 120u8,
                            91u8, 188u8, 16u8, 158u8, 146u8, 65u8, 114u8, 138u8, 57u8, 212u8, 54u8,
                            237u8, 7u8, 162u8, 71u8, 97u8, 232u8, 116u8, 80u8, 93u8, 8u8, 86u8,
                        ],
                    )
                }
                pub fn thaw(
                    &self,
                    id: types::thaw::Id,
                    who: types::thaw::Who,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Thaw> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "thaw",
                        types::Thaw { id, who },
                        [
                            63u8, 247u8, 101u8, 110u8, 87u8, 114u8, 125u8, 53u8, 69u8, 255u8, 60u8,
                            12u8, 145u8, 243u8, 78u8, 107u8, 71u8, 111u8, 145u8, 130u8, 80u8,
                            239u8, 57u8, 86u8, 194u8, 74u8, 61u8, 171u8, 226u8, 232u8, 60u8, 26u8,
                        ],
                    )
                }
                pub fn freeze_asset(
                    &self,
                    id: types::freeze_asset::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::FreezeAsset> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "freeze_asset",
                        types::FreezeAsset { id },
                        [
                            126u8, 44u8, 64u8, 109u8, 7u8, 84u8, 171u8, 163u8, 54u8, 216u8, 115u8,
                            62u8, 235u8, 66u8, 152u8, 163u8, 226u8, 61u8, 173u8, 2u8, 170u8, 137u8,
                            23u8, 80u8, 155u8, 60u8, 87u8, 230u8, 71u8, 75u8, 119u8, 182u8,
                        ],
                    )
                }
                pub fn thaw_asset(
                    &self,
                    id: types::thaw_asset::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ThawAsset> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "thaw_asset",
                        types::ThawAsset { id },
                        [
                            210u8, 37u8, 160u8, 223u8, 252u8, 232u8, 185u8, 44u8, 103u8, 66u8,
                            240u8, 130u8, 73u8, 25u8, 112u8, 179u8, 72u8, 130u8, 154u8, 146u8,
                            217u8, 59u8, 181u8, 78u8, 232u8, 80u8, 54u8, 2u8, 70u8, 226u8, 224u8,
                            211u8,
                        ],
                    )
                }
                pub fn transfer_ownership(
                    &self,
                    id: types::transfer_ownership::Id,
                    owner: types::transfer_ownership::Owner,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TransferOwnership>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer_ownership",
                        types::TransferOwnership { id, owner },
                        [
                            6u8, 121u8, 171u8, 85u8, 202u8, 232u8, 144u8, 5u8, 208u8, 171u8, 247u8,
                            37u8, 227u8, 153u8, 167u8, 60u8, 44u8, 87u8, 18u8, 225u8, 5u8, 89u8,
                            130u8, 167u8, 249u8, 228u8, 172u8, 253u8, 98u8, 251u8, 188u8, 59u8,
                        ],
                    )
                }
                pub fn set_team(
                    &self,
                    id: types::set_team::Id,
                    issuer: types::set_team::Issuer,
                    admin: types::set_team::Admin,
                    freezer: types::set_team::Freezer,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetTeam> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "set_team",
                        types::SetTeam {
                            id,
                            issuer,
                            admin,
                            freezer,
                        },
                        [
                            186u8, 186u8, 142u8, 107u8, 124u8, 109u8, 65u8, 12u8, 252u8, 193u8,
                            190u8, 221u8, 175u8, 187u8, 255u8, 173u8, 10u8, 163u8, 37u8, 224u8,
                            183u8, 161u8, 240u8, 54u8, 79u8, 104u8, 77u8, 43u8, 50u8, 246u8, 29u8,
                            24u8,
                        ],
                    )
                }
                pub fn set_metadata(
                    &self,
                    id: types::set_metadata::Id,
                    name: types::set_metadata::Name,
                    symbol: types::set_metadata::Symbol,
                    decimals: types::set_metadata::Decimals,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetMetadata> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "set_metadata",
                        types::SetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                        },
                        [
                            92u8, 158u8, 102u8, 161u8, 203u8, 203u8, 26u8, 174u8, 103u8, 191u8,
                            71u8, 122u8, 107u8, 255u8, 53u8, 196u8, 28u8, 122u8, 184u8, 193u8,
                            188u8, 132u8, 213u8, 113u8, 77u8, 217u8, 142u8, 72u8, 186u8, 66u8,
                            137u8, 35u8,
                        ],
                    )
                }
                pub fn clear_metadata(
                    &self,
                    id: types::clear_metadata::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ClearMetadata>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "clear_metadata",
                        types::ClearMetadata { id },
                        [
                            122u8, 92u8, 81u8, 147u8, 172u8, 173u8, 169u8, 15u8, 6u8, 21u8, 190u8,
                            117u8, 188u8, 58u8, 131u8, 84u8, 228u8, 116u8, 32u8, 101u8, 231u8,
                            143u8, 172u8, 202u8, 20u8, 0u8, 232u8, 79u8, 202u8, 218u8, 135u8, 83u8,
                        ],
                    )
                }
                pub fn force_set_metadata(
                    &self,
                    id: types::force_set_metadata::Id,
                    name: types::force_set_metadata::Name,
                    symbol: types::force_set_metadata::Symbol,
                    decimals: types::force_set_metadata::Decimals,
                    is_frozen: types::force_set_metadata::IsFrozen,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceSetMetadata>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_set_metadata",
                        types::ForceSetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                            is_frozen,
                        },
                        [
                            207u8, 182u8, 96u8, 158u8, 99u8, 213u8, 211u8, 93u8, 139u8, 147u8,
                            69u8, 151u8, 135u8, 82u8, 96u8, 102u8, 149u8, 12u8, 31u8, 238u8, 17u8,
                            102u8, 204u8, 68u8, 160u8, 123u8, 66u8, 166u8, 250u8, 189u8, 52u8,
                            240u8,
                        ],
                    )
                }
                pub fn force_clear_metadata(
                    &self,
                    id: types::force_clear_metadata::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceClearMetadata>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_clear_metadata",
                        types::ForceClearMetadata { id },
                        [
                            74u8, 134u8, 63u8, 227u8, 75u8, 4u8, 55u8, 255u8, 14u8, 118u8, 234u8,
                            98u8, 228u8, 236u8, 27u8, 104u8, 234u8, 240u8, 204u8, 108u8, 255u8,
                            222u8, 54u8, 74u8, 88u8, 195u8, 122u8, 239u8, 67u8, 78u8, 2u8, 99u8,
                        ],
                    )
                }
                pub fn force_asset_status(
                    &self,
                    id: types::force_asset_status::Id,
                    owner: types::force_asset_status::Owner,
                    issuer: types::force_asset_status::Issuer,
                    admin: types::force_asset_status::Admin,
                    freezer: types::force_asset_status::Freezer,
                    min_balance: types::force_asset_status::MinBalance,
                    is_sufficient: types::force_asset_status::IsSufficient,
                    is_frozen: types::force_asset_status::IsFrozen,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceAssetStatus>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_asset_status",
                        types::ForceAssetStatus {
                            id,
                            owner,
                            issuer,
                            admin,
                            freezer,
                            min_balance,
                            is_sufficient,
                            is_frozen,
                        },
                        [
                            89u8, 198u8, 134u8, 38u8, 93u8, 49u8, 25u8, 96u8, 247u8, 175u8, 115u8,
                            156u8, 30u8, 117u8, 38u8, 254u8, 186u8, 6u8, 135u8, 201u8, 54u8, 127u8,
                            202u8, 74u8, 73u8, 85u8, 65u8, 21u8, 166u8, 221u8, 160u8, 126u8,
                        ],
                    )
                }
                pub fn approve_transfer(
                    &self,
                    id: types::approve_transfer::Id,
                    delegate: types::approve_transfer::Delegate,
                    amount: types::approve_transfer::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ApproveTransfer>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "approve_transfer",
                        types::ApproveTransfer {
                            id,
                            delegate,
                            amount,
                        },
                        [
                            157u8, 118u8, 112u8, 226u8, 178u8, 181u8, 184u8, 102u8, 220u8, 114u8,
                            223u8, 5u8, 23u8, 44u8, 52u8, 195u8, 246u8, 133u8, 40u8, 231u8, 136u8,
                            119u8, 206u8, 115u8, 164u8, 222u8, 136u8, 255u8, 174u8, 103u8, 92u8,
                            32u8,
                        ],
                    )
                }
                pub fn cancel_approval(
                    &self,
                    id: types::cancel_approval::Id,
                    delegate: types::cancel_approval::Delegate,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::CancelApproval>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "cancel_approval",
                        types::CancelApproval { id, delegate },
                        [
                            231u8, 104u8, 37u8, 160u8, 99u8, 145u8, 2u8, 49u8, 211u8, 185u8, 120u8,
                            26u8, 95u8, 4u8, 125u8, 87u8, 44u8, 63u8, 145u8, 108u8, 37u8, 141u8,
                            85u8, 116u8, 217u8, 11u8, 253u8, 216u8, 117u8, 157u8, 222u8, 243u8,
                        ],
                    )
                }
                pub fn force_cancel_approval(
                    &self,
                    id: types::force_cancel_approval::Id,
                    owner: types::force_cancel_approval::Owner,
                    delegate: types::force_cancel_approval::Delegate,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::ForceCancelApproval>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "force_cancel_approval",
                        types::ForceCancelApproval {
                            id,
                            owner,
                            delegate,
                        },
                        [
                            120u8, 250u8, 71u8, 134u8, 67u8, 34u8, 228u8, 12u8, 231u8, 118u8,
                            111u8, 13u8, 170u8, 146u8, 140u8, 217u8, 175u8, 47u8, 138u8, 21u8,
                            149u8, 118u8, 192u8, 65u8, 153u8, 20u8, 63u8, 131u8, 76u8, 66u8, 150u8,
                            246u8,
                        ],
                    )
                }
                pub fn transfer_approved(
                    &self,
                    id: types::transfer_approved::Id,
                    owner: types::transfer_approved::Owner,
                    destination: types::transfer_approved::Destination,
                    amount: types::transfer_approved::Amount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TransferApproved>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "transfer_approved",
                        types::TransferApproved {
                            id,
                            owner,
                            destination,
                            amount,
                        },
                        [
                            207u8, 110u8, 19u8, 110u8, 13u8, 78u8, 151u8, 20u8, 213u8, 246u8,
                            100u8, 157u8, 178u8, 97u8, 123u8, 137u8, 141u8, 53u8, 182u8, 57u8,
                            13u8, 96u8, 33u8, 35u8, 179u8, 20u8, 219u8, 145u8, 3u8, 14u8, 111u8,
                            255u8,
                        ],
                    )
                }
                pub fn touch(
                    &self,
                    id: types::touch::Id,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Touch> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "touch",
                        types::Touch { id },
                        [
                            178u8, 59u8, 116u8, 98u8, 79u8, 172u8, 134u8, 194u8, 221u8, 11u8,
                            184u8, 24u8, 45u8, 240u8, 216u8, 112u8, 46u8, 124u8, 236u8, 153u8,
                            199u8, 165u8, 17u8, 241u8, 49u8, 187u8, 249u8, 149u8, 96u8, 51u8, 46u8,
                            177u8,
                        ],
                    )
                }
                pub fn refund(
                    &self,
                    id: types::refund::Id,
                    allow_burn: types::refund::AllowBurn,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Refund> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "refund",
                        types::Refund { id, allow_burn },
                        [
                            72u8, 205u8, 89u8, 54u8, 233u8, 45u8, 102u8, 214u8, 255u8, 156u8,
                            185u8, 131u8, 57u8, 254u8, 26u8, 255u8, 205u8, 145u8, 211u8, 234u8,
                            157u8, 159u8, 10u8, 148u8, 175u8, 59u8, 185u8, 164u8, 214u8, 68u8,
                            137u8, 246u8,
                        ],
                    )
                }
                pub fn set_min_balance(
                    &self,
                    id: types::set_min_balance::Id,
                    min_balance: types::set_min_balance::MinBalance,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::SetMinBalance>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "set_min_balance",
                        types::SetMinBalance { id, min_balance },
                        [
                            224u8, 17u8, 150u8, 106u8, 225u8, 82u8, 40u8, 192u8, 154u8, 109u8,
                            150u8, 234u8, 86u8, 216u8, 194u8, 26u8, 249u8, 81u8, 44u8, 63u8, 235u8,
                            235u8, 129u8, 118u8, 220u8, 31u8, 230u8, 209u8, 219u8, 216u8, 177u8,
                            105u8,
                        ],
                    )
                }
                pub fn touch_other(
                    &self,
                    id: types::touch_other::Id,
                    who: types::touch_other::Who,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::TouchOther> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "touch_other",
                        types::TouchOther { id, who },
                        [
                            89u8, 21u8, 186u8, 115u8, 86u8, 59u8, 198u8, 50u8, 231u8, 129u8, 58u8,
                            232u8, 105u8, 248u8, 157u8, 91u8, 176u8, 53u8, 188u8, 206u8, 135u8,
                            193u8, 157u8, 252u8, 46u8, 69u8, 18u8, 159u8, 78u8, 151u8, 200u8,
                            246u8,
                        ],
                    )
                }
                pub fn refund_other(
                    &self,
                    id: types::refund_other::Id,
                    who: types::refund_other::Who,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::RefundOther> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "refund_other",
                        types::RefundOther { id, who },
                        [
                            118u8, 150u8, 100u8, 157u8, 241u8, 7u8, 199u8, 161u8, 228u8, 147u8,
                            248u8, 241u8, 218u8, 236u8, 111u8, 150u8, 155u8, 198u8, 248u8, 119u8,
                            133u8, 208u8, 209u8, 150u8, 194u8, 143u8, 42u8, 23u8, 80u8, 107u8,
                            116u8, 177u8,
                        ],
                    )
                }
                pub fn block(
                    &self,
                    id: types::block::Id,
                    who: types::block::Who,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Block> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Assets",
                        "block",
                        types::Block { id, who },
                        [
                            19u8, 48u8, 18u8, 173u8, 63u8, 104u8, 51u8, 30u8, 192u8, 67u8, 215u8,
                            244u8, 62u8, 93u8, 225u8, 21u8, 121u8, 160u8, 224u8, 158u8, 103u8, 4u8,
                            120u8, 120u8, 29u8, 245u8, 203u8, 37u8, 223u8, 88u8, 80u8, 71u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Created {
                pub asset_id: created::AssetId,
                pub creator: created::Creator,
                pub owner: created::Owner,
            }
            pub mod created {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Creator = ::subxt_core::utils::AccountId32;
                pub type Owner = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for Created {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Created";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Issued {
                pub asset_id: issued::AssetId,
                pub owner: issued::Owner,
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Owner = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Issued {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Issued";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Transferred {
                pub asset_id: transferred::AssetId,
                pub from: transferred::From,
                pub to: transferred::To,
                pub amount: transferred::Amount,
            }
            pub mod transferred {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type From = ::subxt_core::utils::AccountId32;
                pub type To = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Transferred {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Transferred";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Burned {
                pub asset_id: burned::AssetId,
                pub owner: burned::Owner,
                pub balance: burned::Balance,
            }
            pub mod burned {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Owner = ::subxt_core::utils::AccountId32;
                pub type Balance = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Burned {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Burned";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct TeamChanged {
                pub asset_id: team_changed::AssetId,
                pub issuer: team_changed::Issuer,
                pub admin: team_changed::Admin,
                pub freezer: team_changed::Freezer,
            }
            pub mod team_changed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Issuer = ::subxt_core::utils::AccountId32;
                pub type Admin = ::subxt_core::utils::AccountId32;
                pub type Freezer = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for TeamChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TeamChanged";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct OwnerChanged {
                pub asset_id: owner_changed::AssetId,
                pub owner: owner_changed::Owner,
            }
            pub mod owner_changed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Owner = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for OwnerChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "OwnerChanged";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Frozen {
                pub asset_id: frozen::AssetId,
                pub who: frozen::Who,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Who = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for Frozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Frozen";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Thawed {
                pub asset_id: thawed::AssetId,
                pub who: thawed::Who,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Who = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for Thawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Thawed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AssetFrozen {
                pub asset_id: asset_frozen::AssetId,
            }
            pub mod asset_frozen {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for AssetFrozen {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetFrozen";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AssetThawed {
                pub asset_id: asset_thawed::AssetId,
            }
            pub mod asset_thawed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for AssetThawed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetThawed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AccountsDestroyed {
                pub asset_id: accounts_destroyed::AssetId,
                pub accounts_destroyed: accounts_destroyed::AccountsDestroyed,
                pub accounts_remaining: accounts_destroyed::AccountsRemaining,
            }
            pub mod accounts_destroyed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type AccountsDestroyed = ::core::primitive::u32;
                pub type AccountsRemaining = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for AccountsDestroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AccountsDestroyed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ApprovalsDestroyed {
                pub asset_id: approvals_destroyed::AssetId,
                pub approvals_destroyed: approvals_destroyed::ApprovalsDestroyed,
                pub approvals_remaining: approvals_destroyed::ApprovalsRemaining,
            }
            pub mod approvals_destroyed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type ApprovalsDestroyed = ::core::primitive::u32;
                pub type ApprovalsRemaining = ::core::primitive::u32;
            }
            impl ::subxt_core::events::StaticEvent for ApprovalsDestroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalsDestroyed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct DestructionStarted {
                pub asset_id: destruction_started::AssetId,
            }
            pub mod destruction_started {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for DestructionStarted {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "DestructionStarted";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Destroyed {
                pub asset_id: destroyed::AssetId,
            }
            pub mod destroyed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for Destroyed {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Destroyed";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ForceCreated {
                pub asset_id: force_created::AssetId,
                pub owner: force_created::Owner,
            }
            pub mod force_created {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Owner = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for ForceCreated {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ForceCreated";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct MetadataSet {
                pub asset_id: metadata_set::AssetId,
                pub name: metadata_set::Name,
                pub symbol: metadata_set::Symbol,
                pub decimals: metadata_set::Decimals,
                pub is_frozen: metadata_set::IsFrozen,
            }
            pub mod metadata_set {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Symbol = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Decimals = ::core::primitive::u8;
                pub type IsFrozen = ::core::primitive::bool;
            }
            impl ::subxt_core::events::StaticEvent for MetadataSet {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataSet";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct MetadataCleared {
                pub asset_id: metadata_cleared::AssetId,
            }
            pub mod metadata_cleared {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for MetadataCleared {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "MetadataCleared";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ApprovedTransfer {
                pub asset_id: approved_transfer::AssetId,
                pub source: approved_transfer::Source,
                pub delegate: approved_transfer::Delegate,
                pub amount: approved_transfer::Amount,
            }
            pub mod approved_transfer {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Source = ::subxt_core::utils::AccountId32;
                pub type Delegate = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for ApprovedTransfer {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovedTransfer";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ApprovalCancelled {
                pub asset_id: approval_cancelled::AssetId,
                pub owner: approval_cancelled::Owner,
                pub delegate: approval_cancelled::Delegate,
            }
            pub mod approval_cancelled {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Owner = ::subxt_core::utils::AccountId32;
                pub type Delegate = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for ApprovalCancelled {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "ApprovalCancelled";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct TransferredApproved {
                pub asset_id: transferred_approved::AssetId,
                pub owner: transferred_approved::Owner,
                pub delegate: transferred_approved::Delegate,
                pub destination: transferred_approved::Destination,
                pub amount: transferred_approved::Amount,
            }
            pub mod transferred_approved {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Owner = ::subxt_core::utils::AccountId32;
                pub type Delegate = ::subxt_core::utils::AccountId32;
                pub type Destination = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for TransferredApproved {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "TransferredApproved";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AssetStatusChanged {
                pub asset_id: asset_status_changed::AssetId,
            }
            pub mod asset_status_changed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for AssetStatusChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetStatusChanged";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AssetMinBalanceChanged {
                pub asset_id: asset_min_balance_changed::AssetId,
                pub new_min_balance: asset_min_balance_changed::NewMinBalance,
            }
            pub mod asset_min_balance_changed {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type NewMinBalance = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for AssetMinBalanceChanged {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "AssetMinBalanceChanged";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Touched {
                pub asset_id: touched::AssetId,
                pub who: touched::Who,
                pub depositor: touched::Depositor,
            }
            pub mod touched {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Depositor = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for Touched {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Touched";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Blocked {
                pub asset_id: blocked::AssetId,
                pub who: blocked::Who,
            }
            pub mod blocked {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Who = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for Blocked {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Blocked";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Deposited {
                pub asset_id: deposited::AssetId,
                pub who: deposited::Who,
                pub amount: deposited::Amount,
            }
            pub mod deposited {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Deposited {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Deposited";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Withdrawn {
                pub asset_id: withdrawn::AssetId,
                pub who: withdrawn::Who,
                pub amount: withdrawn::Amount,
            }
            pub mod withdrawn {
                use super::runtime_types;
                pub type AssetId = runtime_types::vrs_primitives::AssetId;
                pub type Who = ::subxt_core::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for Withdrawn {
                const PALLET: &'static str = "Assets";
                const EVENT: &'static str = "Withdrawn";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod asset {
                    use super::runtime_types;
                    pub type Asset = runtime_types::pallet_assets::types::AssetDetails<
                        ::core::primitive::u128,
                        ::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = runtime_types::vrs_primitives::AssetId;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::pallet_assets::types::AssetAccount<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                        (),
                        ::subxt_core::utils::AccountId32,
                    >;
                    pub type Param0 = runtime_types::vrs_primitives::AssetId;
                    pub type Param1 = ::subxt_core::utils::AccountId32;
                }
                pub mod approvals {
                    use super::runtime_types;
                    pub type Approvals = runtime_types::pallet_assets::types::Approval<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = runtime_types::vrs_primitives::AssetId;
                    pub type Param1 = ::subxt_core::utils::AccountId32;
                    pub type Param2 = ::subxt_core::utils::AccountId32;
                }
                pub mod metadata {
                    use super::runtime_types;
                    pub type Metadata = runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >;
                    pub type Param0 = runtime_types::vrs_primitives::AssetId;
                }
                pub mod next_asset_id {
                    use super::runtime_types;
                    pub type NextAssetId = runtime_types::vrs_primitives::AssetId;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn asset_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::asset::Asset,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Asset",
                        (),
                        [
                            12u8, 73u8, 197u8, 11u8, 154u8, 222u8, 86u8, 108u8, 105u8, 15u8, 187u8,
                            139u8, 152u8, 237u8, 49u8, 42u8, 182u8, 66u8, 226u8, 109u8, 136u8,
                            29u8, 50u8, 74u8, 25u8, 66u8, 138u8, 251u8, 217u8, 232u8, 202u8, 222u8,
                        ],
                    )
                }
                pub fn asset(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::asset::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::asset::Param0>,
                    types::asset::Asset,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Asset",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            12u8, 73u8, 197u8, 11u8, 154u8, 222u8, 86u8, 108u8, 105u8, 15u8, 187u8,
                            139u8, 152u8, 237u8, 49u8, 42u8, 182u8, 66u8, 226u8, 109u8, 136u8,
                            29u8, 50u8, 74u8, 25u8, 66u8, 138u8, 251u8, 217u8, 232u8, 202u8, 222u8,
                        ],
                    )
                }
                pub fn account_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::account::Account,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Account",
                        (),
                        [
                            38u8, 178u8, 135u8, 173u8, 85u8, 150u8, 100u8, 83u8, 227u8, 76u8, 68u8,
                            86u8, 227u8, 171u8, 59u8, 169u8, 18u8, 38u8, 156u8, 77u8, 234u8, 130u8,
                            71u8, 60u8, 245u8, 119u8, 36u8, 35u8, 152u8, 72u8, 96u8, 116u8,
                        ],
                    )
                }
                pub fn account_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::account::Param0>,
                    types::account::Account,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Account",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            38u8, 178u8, 135u8, 173u8, 85u8, 150u8, 100u8, 83u8, 227u8, 76u8, 68u8,
                            86u8, 227u8, 171u8, 59u8, 169u8, 18u8, 38u8, 156u8, 77u8, 234u8, 130u8,
                            71u8, 60u8, 245u8, 119u8, 36u8, 35u8, 152u8, 72u8, 96u8, 116u8,
                        ],
                    )
                }
                pub fn account(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::account::Param0>,
                    _1: impl ::core::borrow::Borrow<types::account::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<types::account::Param0>,
                        ::subxt_core::storage::address::StaticStorageKey<types::account::Param1>,
                    ),
                    types::account::Account,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Account",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            38u8, 178u8, 135u8, 173u8, 85u8, 150u8, 100u8, 83u8, 227u8, 76u8, 68u8,
                            86u8, 227u8, 171u8, 59u8, 169u8, 18u8, 38u8, 156u8, 77u8, 234u8, 130u8,
                            71u8, 60u8, 245u8, 119u8, 36u8, 35u8, 152u8, 72u8, 96u8, 116u8,
                        ],
                    )
                }
                pub fn approvals_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        (),
                        [
                            129u8, 238u8, 163u8, 45u8, 147u8, 111u8, 158u8, 33u8, 249u8, 20u8,
                            43u8, 19u8, 208u8, 226u8, 59u8, 29u8, 200u8, 73u8, 225u8, 226u8, 39u8,
                            87u8, 53u8, 38u8, 30u8, 67u8, 154u8, 71u8, 15u8, 217u8, 112u8, 217u8,
                        ],
                    )
                }
                pub fn approvals_iter1(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::approvals::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::approvals::Param0>,
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            129u8, 238u8, 163u8, 45u8, 147u8, 111u8, 158u8, 33u8, 249u8, 20u8,
                            43u8, 19u8, 208u8, 226u8, 59u8, 29u8, 200u8, 73u8, 225u8, 226u8, 39u8,
                            87u8, 53u8, 38u8, 30u8, 67u8, 154u8, 71u8, 15u8, 217u8, 112u8, 217u8,
                        ],
                    )
                }
                pub fn approvals_iter2(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::approvals::Param0>,
                    _1: impl ::core::borrow::Borrow<types::approvals::Param1>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<types::approvals::Param0>,
                        ::subxt_core::storage::address::StaticStorageKey<types::approvals::Param1>,
                    ),
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                        ),
                        [
                            129u8, 238u8, 163u8, 45u8, 147u8, 111u8, 158u8, 33u8, 249u8, 20u8,
                            43u8, 19u8, 208u8, 226u8, 59u8, 29u8, 200u8, 73u8, 225u8, 226u8, 39u8,
                            87u8, 53u8, 38u8, 30u8, 67u8, 154u8, 71u8, 15u8, 217u8, 112u8, 217u8,
                        ],
                    )
                }
                pub fn approvals(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::approvals::Param0>,
                    _1: impl ::core::borrow::Borrow<types::approvals::Param1>,
                    _2: impl ::core::borrow::Borrow<types::approvals::Param2>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (
                        ::subxt_core::storage::address::StaticStorageKey<types::approvals::Param0>,
                        ::subxt_core::storage::address::StaticStorageKey<types::approvals::Param1>,
                        ::subxt_core::storage::address::StaticStorageKey<types::approvals::Param2>,
                    ),
                    types::approvals::Approvals,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Approvals",
                        (
                            ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_1.borrow()),
                            ::subxt_core::storage::address::StaticStorageKey::new(_2.borrow()),
                        ),
                        [
                            129u8, 238u8, 163u8, 45u8, 147u8, 111u8, 158u8, 33u8, 249u8, 20u8,
                            43u8, 19u8, 208u8, 226u8, 59u8, 29u8, 200u8, 73u8, 225u8, 226u8, 39u8,
                            87u8, 53u8, 38u8, 30u8, 67u8, 154u8, 71u8, 15u8, 217u8, 112u8, 217u8,
                        ],
                    )
                }
                pub fn metadata_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::metadata::Metadata,
                    (),
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Metadata",
                        (),
                        [
                            74u8, 166u8, 135u8, 161u8, 189u8, 223u8, 221u8, 98u8, 193u8, 169u8,
                            175u8, 199u8, 150u8, 83u8, 78u8, 5u8, 2u8, 235u8, 111u8, 206u8, 239u8,
                            115u8, 130u8, 10u8, 117u8, 122u8, 239u8, 163u8, 192u8, 19u8, 33u8,
                            180u8,
                        ],
                    )
                }
                pub fn metadata(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::metadata::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::metadata::Param0>,
                    types::metadata::Metadata,
                    ::subxt_core::utils::Yes,
                    ::subxt_core::utils::Yes,
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "Metadata",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            74u8, 166u8, 135u8, 161u8, 189u8, 223u8, 221u8, 98u8, 193u8, 169u8,
                            175u8, 199u8, 150u8, 83u8, 78u8, 5u8, 2u8, 235u8, 111u8, 206u8, 239u8,
                            115u8, 130u8, 10u8, 117u8, 122u8, 239u8, 163u8, 192u8, 19u8, 33u8,
                            180u8,
                        ],
                    )
                }
                pub fn next_asset_id(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::next_asset_id::NextAssetId,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Assets",
                        "NextAssetId",
                        (),
                        [
                            60u8, 91u8, 232u8, 68u8, 209u8, 62u8, 132u8, 141u8, 226u8, 3u8, 79u8,
                            23u8, 198u8, 39u8, 237u8, 37u8, 237u8, 211u8, 100u8, 199u8, 115u8,
                            198u8, 187u8, 120u8, 181u8, 173u8, 128u8, 135u8, 238u8, 124u8, 73u8,
                            110u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn remove_items_limit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "RemoveItemsLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
                pub fn asset_deposit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "AssetDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn asset_account_deposit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "AssetAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "MetadataDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "MetadataDepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn approval_deposit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "ApprovalDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn string_limit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u32>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Assets",
                        "StringLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod swap {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_swap::pallet::Error;
        pub type Call = runtime_types::pallet_swap::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct CreateExchange {
                    pub asset_id: create_exchange::AssetId,
                    pub currency_amount: create_exchange::CurrencyAmount,
                    pub token_amount: create_exchange::TokenAmount,
                }
                pub mod create_exchange {
                    use super::runtime_types;
                    pub type AssetId = runtime_types::vrs_primitives::AssetId;
                    pub type CurrencyAmount = ::core::primitive::u128;
                    pub type TokenAmount = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for CreateExchange {
                    const PALLET: &'static str = "Swap";
                    const CALL: &'static str = "create_exchange";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AddLiquidity {
                    pub asset_id: add_liquidity::AssetId,
                    pub currency_amount: add_liquidity::CurrencyAmount,
                    pub min_liquidity: add_liquidity::MinLiquidity,
                    pub max_tokens: add_liquidity::MaxTokens,
                }
                pub mod add_liquidity {
                    use super::runtime_types;
                    pub type AssetId = runtime_types::vrs_primitives::AssetId;
                    pub type CurrencyAmount = ::core::primitive::u128;
                    pub type MinLiquidity = ::core::primitive::u128;
                    pub type MaxTokens = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for AddLiquidity {
                    const PALLET: &'static str = "Swap";
                    const CALL: &'static str = "add_liquidity";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct RemoveLiquidity {
                    pub asset_id: remove_liquidity::AssetId,
                    pub liquidity_amount: remove_liquidity::LiquidityAmount,
                    pub min_currency: remove_liquidity::MinCurrency,
                    pub min_tokens: remove_liquidity::MinTokens,
                }
                pub mod remove_liquidity {
                    use super::runtime_types;
                    pub type AssetId = runtime_types::vrs_primitives::AssetId;
                    pub type LiquidityAmount = ::core::primitive::u128;
                    pub type MinCurrency = ::core::primitive::u128;
                    pub type MinTokens = ::core::primitive::u128;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for RemoveLiquidity {
                    const PALLET: &'static str = "Swap";
                    const CALL: &'static str = "remove_liquidity";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct CurrencyToAsset {
                    pub asset_id: currency_to_asset::AssetId,
                    pub amount: currency_to_asset::Amount,
                    pub recipient: currency_to_asset::Recipient,
                }
                pub mod currency_to_asset {
                    use super::runtime_types;
                    pub type AssetId = runtime_types::vrs_primitives::AssetId;
                    pub type Amount = runtime_types::pallet_swap::pallet::TradeAmount<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Recipient = ::core::option::Option<::subxt_core::utils::AccountId32>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for CurrencyToAsset {
                    const PALLET: &'static str = "Swap";
                    const CALL: &'static str = "currency_to_asset";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AssetToCurrency {
                    pub asset_id: asset_to_currency::AssetId,
                    pub amount: asset_to_currency::Amount,
                    pub recipient: asset_to_currency::Recipient,
                }
                pub mod asset_to_currency {
                    use super::runtime_types;
                    pub type AssetId = runtime_types::vrs_primitives::AssetId;
                    pub type Amount = runtime_types::pallet_swap::pallet::TradeAmount<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Recipient = ::core::option::Option<::subxt_core::utils::AccountId32>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for AssetToCurrency {
                    const PALLET: &'static str = "Swap";
                    const CALL: &'static str = "asset_to_currency";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AssetToAsset {
                    pub sold_asset_id: asset_to_asset::SoldAssetId,
                    pub bought_asset_id: asset_to_asset::BoughtAssetId,
                    pub amount: asset_to_asset::Amount,
                    pub recipient: asset_to_asset::Recipient,
                }
                pub mod asset_to_asset {
                    use super::runtime_types;
                    pub type SoldAssetId = runtime_types::vrs_primitives::AssetId;
                    pub type BoughtAssetId = runtime_types::vrs_primitives::AssetId;
                    pub type Amount = runtime_types::pallet_swap::pallet::TradeAmount<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Recipient = ::core::option::Option<::subxt_core::utils::AccountId32>;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for AssetToAsset {
                    const PALLET: &'static str = "Swap";
                    const CALL: &'static str = "asset_to_asset";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create_exchange(
                    &self,
                    asset_id: types::create_exchange::AssetId,
                    currency_amount: types::create_exchange::CurrencyAmount,
                    token_amount: types::create_exchange::TokenAmount,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::CreateExchange>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Swap",
                        "create_exchange",
                        types::CreateExchange {
                            asset_id,
                            currency_amount,
                            token_amount,
                        },
                        [
                            239u8, 6u8, 55u8, 49u8, 175u8, 215u8, 250u8, 69u8, 134u8, 124u8, 212u8,
                            93u8, 46u8, 136u8, 123u8, 93u8, 27u8, 67u8, 127u8, 61u8, 86u8, 245u8,
                            40u8, 29u8, 132u8, 112u8, 187u8, 5u8, 114u8, 176u8, 40u8, 68u8,
                        ],
                    )
                }
                pub fn add_liquidity(
                    &self,
                    asset_id: types::add_liquidity::AssetId,
                    currency_amount: types::add_liquidity::CurrencyAmount,
                    min_liquidity: types::add_liquidity::MinLiquidity,
                    max_tokens: types::add_liquidity::MaxTokens,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::AddLiquidity> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Swap",
                        "add_liquidity",
                        types::AddLiquidity {
                            asset_id,
                            currency_amount,
                            min_liquidity,
                            max_tokens,
                        },
                        [
                            187u8, 153u8, 19u8, 30u8, 251u8, 40u8, 97u8, 44u8, 192u8, 169u8, 107u8,
                            125u8, 193u8, 159u8, 135u8, 101u8, 181u8, 153u8, 158u8, 178u8, 129u8,
                            4u8, 216u8, 186u8, 173u8, 143u8, 192u8, 193u8, 227u8, 180u8, 233u8,
                            184u8,
                        ],
                    )
                }
                pub fn remove_liquidity(
                    &self,
                    asset_id: types::remove_liquidity::AssetId,
                    liquidity_amount: types::remove_liquidity::LiquidityAmount,
                    min_currency: types::remove_liquidity::MinCurrency,
                    min_tokens: types::remove_liquidity::MinTokens,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::RemoveLiquidity>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Swap",
                        "remove_liquidity",
                        types::RemoveLiquidity {
                            asset_id,
                            liquidity_amount,
                            min_currency,
                            min_tokens,
                        },
                        [
                            100u8, 95u8, 31u8, 122u8, 39u8, 138u8, 112u8, 149u8, 6u8, 67u8, 119u8,
                            53u8, 227u8, 98u8, 82u8, 227u8, 227u8, 103u8, 140u8, 62u8, 215u8, 90u8,
                            51u8, 70u8, 140u8, 112u8, 195u8, 40u8, 212u8, 176u8, 74u8, 32u8,
                        ],
                    )
                }
                pub fn currency_to_asset(
                    &self,
                    asset_id: types::currency_to_asset::AssetId,
                    amount: types::currency_to_asset::Amount,
                    recipient: types::currency_to_asset::Recipient,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::CurrencyToAsset>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Swap",
                        "currency_to_asset",
                        types::CurrencyToAsset {
                            asset_id,
                            amount,
                            recipient,
                        },
                        [
                            115u8, 169u8, 158u8, 226u8, 44u8, 24u8, 132u8, 52u8, 205u8, 178u8, 1u8,
                            214u8, 112u8, 189u8, 205u8, 37u8, 119u8, 230u8, 8u8, 48u8, 229u8, 89u8,
                            182u8, 64u8, 191u8, 230u8, 166u8, 10u8, 227u8, 212u8, 228u8, 244u8,
                        ],
                    )
                }
                pub fn asset_to_currency(
                    &self,
                    asset_id: types::asset_to_currency::AssetId,
                    amount: types::asset_to_currency::Amount,
                    recipient: types::asset_to_currency::Recipient,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::AssetToCurrency>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Swap",
                        "asset_to_currency",
                        types::AssetToCurrency {
                            asset_id,
                            amount,
                            recipient,
                        },
                        [
                            223u8, 33u8, 96u8, 28u8, 9u8, 56u8, 101u8, 100u8, 51u8, 20u8, 190u8,
                            131u8, 69u8, 188u8, 72u8, 61u8, 18u8, 125u8, 200u8, 147u8, 58u8, 165u8,
                            4u8, 17u8, 228u8, 140u8, 119u8, 147u8, 130u8, 52u8, 134u8, 60u8,
                        ],
                    )
                }
                pub fn asset_to_asset(
                    &self,
                    sold_asset_id: types::asset_to_asset::SoldAssetId,
                    bought_asset_id: types::asset_to_asset::BoughtAssetId,
                    amount: types::asset_to_asset::Amount,
                    recipient: types::asset_to_asset::Recipient,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::AssetToAsset> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Swap",
                        "asset_to_asset",
                        types::AssetToAsset {
                            sold_asset_id,
                            bought_asset_id,
                            amount,
                            recipient,
                        },
                        [
                            158u8, 83u8, 57u8, 211u8, 73u8, 104u8, 157u8, 119u8, 1u8, 244u8, 60u8,
                            104u8, 9u8, 16u8, 146u8, 172u8, 234u8, 35u8, 51u8, 64u8, 174u8, 213u8,
                            221u8, 78u8, 171u8, 104u8, 147u8, 168u8, 221u8, 229u8, 9u8, 194u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_swap::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ExchangeCreated(pub exchange_created::Field0, pub exchange_created::Field1);
            pub mod exchange_created {
                use super::runtime_types;
                pub type Field0 = runtime_types::vrs_primitives::AssetId;
                pub type Field1 = runtime_types::vrs_primitives::AssetId;
            }
            impl ::subxt_core::events::StaticEvent for ExchangeCreated {
                const PALLET: &'static str = "Swap";
                const EVENT: &'static str = "ExchangeCreated";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct LiquidityAdded(
                pub liquidity_added::Field0,
                pub liquidity_added::Field1,
                pub liquidity_added::Field2,
                pub liquidity_added::Field3,
                pub liquidity_added::Field4,
            );
            pub mod liquidity_added {
                use super::runtime_types;
                pub type Field0 = ::subxt_core::utils::AccountId32;
                pub type Field1 = runtime_types::vrs_primitives::AssetId;
                pub type Field2 = ::core::primitive::u128;
                pub type Field3 = ::core::primitive::u128;
                pub type Field4 = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for LiquidityAdded {
                const PALLET: &'static str = "Swap";
                const EVENT: &'static str = "LiquidityAdded";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct LiquidityRemoved(
                pub liquidity_removed::Field0,
                pub liquidity_removed::Field1,
                pub liquidity_removed::Field2,
                pub liquidity_removed::Field3,
                pub liquidity_removed::Field4,
            );
            pub mod liquidity_removed {
                use super::runtime_types;
                pub type Field0 = ::subxt_core::utils::AccountId32;
                pub type Field1 = runtime_types::vrs_primitives::AssetId;
                pub type Field2 = ::core::primitive::u128;
                pub type Field3 = ::core::primitive::u128;
                pub type Field4 = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for LiquidityRemoved {
                const PALLET: &'static str = "Swap";
                const EVENT: &'static str = "LiquidityRemoved";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct CurrencyTradedForAsset(
                pub currency_traded_for_asset::Field0,
                pub currency_traded_for_asset::Field1,
                pub currency_traded_for_asset::Field2,
                pub currency_traded_for_asset::Field3,
                pub currency_traded_for_asset::Field4,
            );
            pub mod currency_traded_for_asset {
                use super::runtime_types;
                pub type Field0 = runtime_types::vrs_primitives::AssetId;
                pub type Field1 = ::subxt_core::utils::AccountId32;
                pub type Field2 = ::subxt_core::utils::AccountId32;
                pub type Field3 = ::core::primitive::u128;
                pub type Field4 = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for CurrencyTradedForAsset {
                const PALLET: &'static str = "Swap";
                const EVENT: &'static str = "CurrencyTradedForAsset";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AssetTradedForCurrency(
                pub asset_traded_for_currency::Field0,
                pub asset_traded_for_currency::Field1,
                pub asset_traded_for_currency::Field2,
                pub asset_traded_for_currency::Field3,
                pub asset_traded_for_currency::Field4,
            );
            pub mod asset_traded_for_currency {
                use super::runtime_types;
                pub type Field0 = runtime_types::vrs_primitives::AssetId;
                pub type Field1 = ::subxt_core::utils::AccountId32;
                pub type Field2 = ::subxt_core::utils::AccountId32;
                pub type Field3 = ::core::primitive::u128;
                pub type Field4 = ::core::primitive::u128;
            }
            impl ::subxt_core::events::StaticEvent for AssetTradedForCurrency {
                const PALLET: &'static str = "Swap";
                const EVENT: &'static str = "AssetTradedForCurrency";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod exchanges {
                    use super::runtime_types;
                    pub type Exchanges = runtime_types::pallet_swap::pallet::Exchange<
                        runtime_types::vrs_primitives::AssetId,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = runtime_types::vrs_primitives::AssetId;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn exchanges_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::exchanges::Exchanges,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Swap",
                        "Exchanges",
                        (),
                        [
                            92u8, 38u8, 239u8, 152u8, 68u8, 217u8, 86u8, 164u8, 34u8, 12u8, 206u8,
                            199u8, 18u8, 226u8, 111u8, 7u8, 118u8, 97u8, 121u8, 166u8, 184u8, 64u8,
                            115u8, 175u8, 178u8, 255u8, 103u8, 102u8, 176u8, 236u8, 99u8, 86u8,
                        ],
                    )
                }
                pub fn exchanges(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::exchanges::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::exchanges::Param0>,
                    types::exchanges::Exchanges,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "Swap",
                        "Exchanges",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            92u8, 38u8, 239u8, 152u8, 68u8, 217u8, 86u8, 164u8, 34u8, 12u8, 206u8,
                            199u8, 18u8, 226u8, 111u8, 7u8, 118u8, 97u8, 121u8, 166u8, 184u8, 64u8,
                            115u8, 175u8, 178u8, 255u8, 103u8, 102u8, 176u8, 236u8, 99u8, 86u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn pallet_id(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<
                    runtime_types::frame_support::PalletId,
                > {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Swap",
                        "PalletId",
                        [
                            56u8, 243u8, 53u8, 83u8, 154u8, 179u8, 170u8, 80u8, 133u8, 173u8, 61u8,
                            161u8, 47u8, 225u8, 146u8, 21u8, 50u8, 229u8, 248u8, 27u8, 104u8, 58u8,
                            129u8, 197u8, 102u8, 160u8, 168u8, 205u8, 154u8, 42u8, 217u8, 53u8,
                        ],
                    )
                }
                pub fn provider_fee_numerator(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Swap",
                        "ProviderFeeNumerator",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn provider_fee_denominator(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Swap",
                        "ProviderFeeDenominator",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
                pub fn min_deposit(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u128>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Swap",
                        "MinDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod a2a {
        use super::root_mod;
        use super::runtime_types;
        pub type Error = runtime_types::pallet_a2a::pallet::Error;
        pub type Call = runtime_types::pallet_a2a::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Register {
                    pub agent_card: register::AgentCard,
                }
                pub mod register {
                    use super::runtime_types;
                    pub type AgentCard = runtime_types::a2a_rs::AgentCard;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Register {
                    const PALLET: &'static str = "A2A";
                    const CALL: &'static str = "register";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Update {
                    pub agent_id: update::AgentId,
                    pub agent_card: update::AgentCard,
                }
                pub mod update {
                    use super::runtime_types;
                    pub type AgentId = ::subxt_core::utils::AccountId32;
                    pub type AgentCard = runtime_types::a2a_rs::AgentCard;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Update {
                    const PALLET: &'static str = "A2A";
                    const CALL: &'static str = "update";
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Deregister {
                    pub agent_id: deregister::AgentId,
                }
                pub mod deregister {
                    use super::runtime_types;
                    pub type AgentId = ::subxt_core::utils::AccountId32;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for Deregister {
                    const PALLET: &'static str = "A2A";
                    const CALL: &'static str = "deregister";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn register(
                    &self,
                    agent_card: types::register::AgentCard,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Register> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "A2A",
                        "register",
                        types::Register { agent_card },
                        [
                            122u8, 81u8, 52u8, 49u8, 231u8, 255u8, 189u8, 2u8, 121u8, 119u8, 98u8,
                            175u8, 66u8, 241u8, 251u8, 165u8, 156u8, 147u8, 46u8, 81u8, 103u8,
                            166u8, 176u8, 173u8, 214u8, 99u8, 123u8, 0u8, 120u8, 6u8, 51u8, 39u8,
                        ],
                    )
                }
                pub fn update(
                    &self,
                    agent_id: types::update::AgentId,
                    agent_card: types::update::AgentCard,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Update> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "A2A",
                        "update",
                        types::Update {
                            agent_id,
                            agent_card,
                        },
                        [
                            119u8, 101u8, 207u8, 105u8, 192u8, 228u8, 60u8, 102u8, 89u8, 185u8,
                            75u8, 27u8, 179u8, 125u8, 96u8, 72u8, 44u8, 175u8, 165u8, 108u8, 150u8,
                            226u8, 108u8, 205u8, 39u8, 63u8, 128u8, 117u8, 8u8, 112u8, 207u8, 68u8,
                        ],
                    )
                }
                pub fn deregister(
                    &self,
                    agent_id: types::deregister::AgentId,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::Deregister> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "A2A",
                        "deregister",
                        types::Deregister { agent_id },
                        [
                            192u8, 211u8, 121u8, 60u8, 12u8, 188u8, 212u8, 37u8, 33u8, 176u8,
                            147u8, 127u8, 10u8, 29u8, 134u8, 54u8, 60u8, 195u8, 5u8, 173u8, 169u8,
                            235u8, 255u8, 233u8, 76u8, 129u8, 248u8, 27u8, 47u8, 76u8, 219u8, 52u8,
                        ],
                    )
                }
            }
        }
        pub type Event = runtime_types::pallet_a2a::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentRegistered {
                pub id: agent_registered::Id,
                pub owner: agent_registered::Owner,
            }
            pub mod agent_registered {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
                pub type Owner = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for AgentRegistered {
                const PALLET: &'static str = "A2A";
                const EVENT: &'static str = "AgentRegistered";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentUpdated {
                pub id: agent_updated::Id,
                pub owner: agent_updated::Owner,
            }
            pub mod agent_updated {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
                pub type Owner = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for AgentUpdated {
                const PALLET: &'static str = "A2A";
                const EVENT: &'static str = "AgentUpdated";
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentDeregistered {
                pub id: agent_deregistered::Id,
            }
            pub mod agent_deregistered {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
            }
            impl ::subxt_core::events::StaticEvent for AgentDeregistered {
                const PALLET: &'static str = "A2A";
                const EVENT: &'static str = "AgentDeregistered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod agents {
                    use super::runtime_types;
                    pub type Agents =
                        runtime_types::a2a_rs::AgentInfo<::subxt_core::utils::AccountId32>;
                    pub type Param0 = ::subxt_core::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn agents_iter(
                    &self,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    (),
                    types::agents::Agents,
                    (),
                    (),
                    ::subxt_core::utils::Yes,
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "A2A",
                        "Agents",
                        (),
                        [
                            180u8, 48u8, 159u8, 226u8, 32u8, 112u8, 178u8, 134u8, 39u8, 185u8,
                            222u8, 38u8, 112u8, 116u8, 197u8, 179u8, 67u8, 78u8, 189u8, 137u8,
                            104u8, 145u8, 55u8, 84u8, 11u8, 96u8, 50u8, 64u8, 13u8, 41u8, 217u8,
                            82u8,
                        ],
                    )
                }
                pub fn agents(
                    &self,
                    _0: impl ::core::borrow::Borrow<types::agents::Param0>,
                ) -> ::subxt_core::storage::address::StaticAddress<
                    ::subxt_core::storage::address::StaticStorageKey<types::agents::Param0>,
                    types::agents::Agents,
                    ::subxt_core::utils::Yes,
                    (),
                    (),
                > {
                    ::subxt_core::storage::address::StaticAddress::new_static(
                        "A2A",
                        "Agents",
                        ::subxt_core::storage::address::StaticStorageKey::new(_0.borrow()),
                        [
                            180u8, 48u8, 159u8, 226u8, 32u8, 112u8, 178u8, 134u8, 39u8, 185u8,
                            222u8, 38u8, 112u8, 116u8, 197u8, 179u8, 67u8, 78u8, 189u8, 137u8,
                            104u8, 145u8, 55u8, 84u8, 11u8, 96u8, 50u8, 64u8, 13u8, 41u8, 217u8,
                            82u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod a2a_rs {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentCapabilities {
                pub streaming: ::core::option::Option<::core::primitive::bool>,
                pub push_notifications: ::core::option::Option<::core::primitive::bool>,
                pub state_transition_history: ::core::option::Option<::core::primitive::bool>,
                pub extensions: ::core::option::Option<
                    ::subxt_core::alloc::vec::Vec<runtime_types::a2a_rs::AgentExtension>,
                >,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentCard {
                pub name: ::subxt_core::alloc::string::String,
                pub description: ::subxt_core::alloc::string::String,
                pub url: ::subxt_core::alloc::string::String,
                pub icon_url: ::core::option::Option<::subxt_core::alloc::string::String>,
                pub provider: ::core::option::Option<runtime_types::a2a_rs::AgentProvider>,
                pub version: ::subxt_core::alloc::string::String,
                pub documentation_url: ::core::option::Option<::subxt_core::alloc::string::String>,
                pub capabilities: runtime_types::a2a_rs::AgentCapabilities,
                pub security_schemes: ::core::option::Option<
                    ::subxt_core::utils::KeyedVec<
                        ::subxt_core::alloc::string::String,
                        ::subxt_core::alloc::string::String,
                    >,
                >,
                pub security: ::core::option::Option<
                    ::subxt_core::alloc::vec::Vec<
                        ::subxt_core::utils::KeyedVec<
                            ::subxt_core::alloc::string::String,
                            ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                        >,
                    >,
                >,
                pub default_input_modes:
                    ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                pub default_output_modes:
                    ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                pub skills: ::subxt_core::alloc::vec::Vec<runtime_types::a2a_rs::AgentSkill>,
                pub supports_authenticated_extended_card:
                    ::core::option::Option<::core::primitive::bool>,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentExtension {
                pub uri: ::subxt_core::alloc::string::String,
                pub description: ::core::option::Option<::subxt_core::alloc::string::String>,
                pub required: ::core::option::Option<::core::primitive::bool>,
                pub params: ::core::option::Option<::subxt_core::alloc::string::String>,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentInfo<_0> {
                pub agent_id: _0,
                pub owner_id: _0,
                pub agent_card: runtime_types::a2a_rs::AgentCard,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentProvider {
                pub organization: ::subxt_core::alloc::string::String,
                pub url: ::subxt_core::alloc::string::String,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AgentSkill {
                pub id: ::subxt_core::alloc::string::String,
                pub name: ::subxt_core::alloc::string::String,
                pub description: ::subxt_core::alloc::string::String,
                pub tags: ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                pub examples: ::core::option::Option<
                    ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                >,
                pub input_modes: ::core::option::Option<
                    ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                >,
                pub output_modes: ::core::option::Option<
                    ::subxt_core::alloc::vec::Vec<::subxt_core::alloc::string::String>,
                >,
            }
        }
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::subxt_core::alloc::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::subxt_core::alloc::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt_core :: ext :: codec :: Decode,
                            :: subxt_core :: ext :: codec :: Encode,
                            :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt_core :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                        #[derive(
                            :: subxt_core :: ext :: codec :: Decode,
                            :: subxt_core :: ext :: codec :: Encode,
                            :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt_core :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                        pub struct IdAmount<_0, _1> {
                            pub id: _0,
                            pub amount: _1,
                        }
                    }
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    remark {
                        remark: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    set_code {
                        code: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    set_code_without_checks {
                        code: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_storage {
                        items: ::subxt_core::alloc::vec::Vec<(
                            ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                            ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    kill_storage {
                        keys: ::subxt_core::alloc::vec::Vec<
                            ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        >,
                    },
                    #[codec(index = 6)]
                    kill_prefix {
                        prefix: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    remark_with_event {
                        remark: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    authorize_upgrade {
                        code_hash: ::subxt_core::utils::H256,
                    },
                    #[codec(index = 10)]
                    authorize_upgrade_without_checks {
                        code_hash: ::subxt_core::utils::H256,
                    },
                    #[codec(index = 11)]
                    apply_authorized_upgrade {
                        code: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    CallFiltered,
                    #[codec(index = 6)]
                    MultiBlockMigrationsOngoing,
                    #[codec(index = 7)]
                    NothingAuthorized,
                    #[codec(index = 8)]
                    Unauthorized,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount {
                        account: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    KilledAccount {
                        account: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    Remarked {
                        sender: ::subxt_core::utils::AccountId32,
                        hash: ::subxt_core::utils::H256,
                    },
                    #[codec(index = 6)]
                    UpgradeAuthorized {
                        code_hash: ::subxt_core::utils::H256,
                        check_version: ::core::primitive::bool,
                    },
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct CodeUpgradeAuthorization {
                pub code_hash: ::subxt_core::utils::H256,
                pub check_version: ::core::primitive::bool,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::subxt_core::alloc::vec::Vec<_1>,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::subxt_core::alloc::string::String,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod pallet_a2a {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    register {
                        agent_card: runtime_types::a2a_rs::AgentCard,
                    },
                    #[codec(index = 1)]
                    update {
                        agent_id: ::subxt_core::utils::AccountId32,
                        agent_card: runtime_types::a2a_rs::AgentCard,
                    },
                    #[codec(index = 2)]
                    deregister {
                        agent_id: ::subxt_core::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    NotAuthorized,
                    #[codec(index = 1)]
                    AgentAlreadyExists,
                    #[codec(index = 2)]
                    AgentNotFound,
                    #[codec(index = 3)]
                    AgentNameImmutable,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    AgentRegistered {
                        id: ::subxt_core::utils::AccountId32,
                        owner: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    AgentUpdated {
                        id: ::subxt_core::utils::AccountId32,
                        owner: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    AgentDeregistered {
                        id: ::subxt_core::utils::AccountId32,
                    },
                }
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    create {
                        id: runtime_types::vrs_primitives::AssetId,
                        admin:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    force_create {
                        id: runtime_types::vrs_primitives::AssetId,
                        owner:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    start_destroy {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 3)]
                    destroy_accounts {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 4)]
                    destroy_approvals {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 5)]
                    finish_destroy {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 6)]
                    mint {
                        id: runtime_types::vrs_primitives::AssetId,
                        beneficiary:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    burn {
                        id: runtime_types::vrs_primitives::AssetId,
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    transfer {
                        id: runtime_types::vrs_primitives::AssetId,
                        target:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    transfer_keep_alive {
                        id: runtime_types::vrs_primitives::AssetId,
                        target:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    force_transfer {
                        id: runtime_types::vrs_primitives::AssetId,
                        source:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        dest:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    freeze {
                        id: runtime_types::vrs_primitives::AssetId,
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 12)]
                    thaw {
                        id: runtime_types::vrs_primitives::AssetId,
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 13)]
                    freeze_asset {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 14)]
                    thaw_asset {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 15)]
                    transfer_ownership {
                        id: runtime_types::vrs_primitives::AssetId,
                        owner:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 16)]
                    set_team {
                        id: runtime_types::vrs_primitives::AssetId,
                        issuer:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        admin:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        freezer:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 17)]
                    set_metadata {
                        id: runtime_types::vrs_primitives::AssetId,
                        name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 18)]
                    clear_metadata {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 19)]
                    force_set_metadata {
                        id: runtime_types::vrs_primitives::AssetId,
                        name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 20)]
                    force_clear_metadata {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 21)]
                    force_asset_status {
                        id: runtime_types::vrs_primitives::AssetId,
                        owner:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        issuer:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        admin:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        freezer:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 22)]
                    approve_transfer {
                        id: runtime_types::vrs_primitives::AssetId,
                        delegate:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    cancel_approval {
                        id: runtime_types::vrs_primitives::AssetId,
                        delegate:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 24)]
                    force_cancel_approval {
                        id: runtime_types::vrs_primitives::AssetId,
                        owner:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        delegate:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 25)]
                    transfer_approved {
                        id: runtime_types::vrs_primitives::AssetId,
                        owner:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        destination:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    touch {
                        id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 27)]
                    refund {
                        id: runtime_types::vrs_primitives::AssetId,
                        allow_burn: ::core::primitive::bool,
                    },
                    #[codec(index = 28)]
                    set_min_balance {
                        id: runtime_types::vrs_primitives::AssetId,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 29)]
                    touch_other {
                        id: runtime_types::vrs_primitives::AssetId,
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 30)]
                    refund_other {
                        id: runtime_types::vrs_primitives::AssetId,
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 31)]
                    block {
                        id: runtime_types::vrs_primitives::AssetId,
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    NoAccount,
                    #[codec(index = 2)]
                    NoPermission,
                    #[codec(index = 3)]
                    Unknown,
                    #[codec(index = 4)]
                    Frozen,
                    #[codec(index = 5)]
                    InUse,
                    #[codec(index = 6)]
                    BadWitness,
                    #[codec(index = 7)]
                    MinBalanceZero,
                    #[codec(index = 8)]
                    UnavailableConsumer,
                    #[codec(index = 9)]
                    BadMetadata,
                    #[codec(index = 10)]
                    Unapproved,
                    #[codec(index = 11)]
                    WouldDie,
                    #[codec(index = 12)]
                    AlreadyExists,
                    #[codec(index = 13)]
                    NoDeposit,
                    #[codec(index = 14)]
                    WouldBurn,
                    #[codec(index = 15)]
                    LiveAsset,
                    #[codec(index = 16)]
                    AssetNotLive,
                    #[codec(index = 17)]
                    IncorrectStatus,
                    #[codec(index = 18)]
                    NotFrozen,
                    #[codec(index = 19)]
                    CallbackFailed,
                    #[codec(index = 20)]
                    BadAssetId,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    Created {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        creator: ::subxt_core::utils::AccountId32,
                        owner: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    Issued {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        owner: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transferred {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        from: ::subxt_core::utils::AccountId32,
                        to: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    Burned {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        owner: ::subxt_core::utils::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    TeamChanged {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        issuer: ::subxt_core::utils::AccountId32,
                        admin: ::subxt_core::utils::AccountId32,
                        freezer: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    OwnerChanged {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        owner: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    Frozen {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        who: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    Thawed {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        who: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    AssetFrozen {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 9)]
                    AssetThawed {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 10)]
                    AccountsDestroyed {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        accounts_destroyed: ::core::primitive::u32,
                        accounts_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    ApprovalsDestroyed {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        approvals_destroyed: ::core::primitive::u32,
                        approvals_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    DestructionStarted {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 13)]
                    Destroyed {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 14)]
                    ForceCreated {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        owner: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    MetadataSet {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        symbol: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    MetadataCleared {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 17)]
                    ApprovedTransfer {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        source: ::subxt_core::utils::AccountId32,
                        delegate: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    ApprovalCancelled {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        owner: ::subxt_core::utils::AccountId32,
                        delegate: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 19)]
                    TransferredApproved {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        owner: ::subxt_core::utils::AccountId32,
                        delegate: ::subxt_core::utils::AccountId32,
                        destination: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    AssetStatusChanged {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                    },
                    #[codec(index = 21)]
                    AssetMinBalanceChanged {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        new_min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 22)]
                    Touched {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        who: ::subxt_core::utils::AccountId32,
                        depositor: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 23)]
                    Blocked {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        who: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 24)]
                    Deposited {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 25)]
                    Withdrawn {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum AccountStatus {
                    #[codec(index = 0)]
                    Liquid,
                    #[codec(index = 1)]
                    Frozen,
                    #[codec(index = 2)]
                    Blocked,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AssetAccount<_0, _1, _2, _3> {
                    pub balance: _0,
                    pub status: runtime_types::pallet_assets::types::AccountStatus,
                    pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0, _3>,
                    pub extra: _2,
                    #[codec(skip)]
                    pub __ignore: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _2,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub status: runtime_types::pallet_assets::types::AssetStatus,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum AssetStatus {
                    #[codec(index = 0)]
                    Live,
                    #[codec(index = 1)]
                    Frozen,
                    #[codec(index = 2)]
                    Destroying,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum ExistenceReason<_0, _1> {
                    #[codec(index = 0)]
                    Consumer,
                    #[codec(index = 1)]
                    Sufficient,
                    #[codec(index = 2)]
                    DepositHeld(_0),
                    #[codec(index = 3)]
                    DepositRefunded,
                    #[codec(index = 4)]
                    DepositFrom(_1, _0),
                }
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                    #[codec(index = 3)]
                    InvalidConfiguration,
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer_allow_death {
                        dest:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        dest:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    upgrade_accounts {
                        who: ::subxt_core::alloc::vec::Vec<::subxt_core::utils::AccountId32>,
                    },
                    #[codec(index = 8)]
                    force_set_balance {
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    force_adjust_total_issuance {
                        direction: runtime_types::pallet_balances::types::AdjustmentDirection,
                        #[codec(compact)]
                        delta: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    burn {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        keep_alive: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    Expendability,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                    #[codec(index = 8)]
                    TooManyHolds,
                    #[codec(index = 9)]
                    TooManyFreezes,
                    #[codec(index = 10)]
                    IssuanceDeactivated,
                    #[codec(index = 11)]
                    DeltaZero,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt_core::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt_core::utils::AccountId32,
                        to: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt_core::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt_core::utils::AccountId32,
                        to: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    Minted {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    Burned {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    Suspended {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    Restored {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    Upgraded {
                        who: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    Locked {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    Unlocked {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    Frozen {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    Thawed {
                        who: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 21)]
                    TotalIssuanceForced {
                        old: ::core::primitive::u128,
                        new: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum AdjustmentDirection {
                    #[codec(index = 0)]
                    Increase,
                    #[codec(index = 1)]
                    Decrease,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt_core::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::subxt_core::alloc::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt_core::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities {
                        authority_set: ::subxt_core::alloc::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived {
                        authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    },
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline {
                        offline: ::subxt_core::alloc::vec::Vec<(
                            ::subxt_core::utils::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct Public(pub [::core::primitive::u8; 32usize]);
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct Signature(pub [::core::primitive::u8; 64usize]);
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub session_index: ::core::primitive::u32,
                pub authority_index: ::core::primitive::u32,
                pub validators_len: ::core::primitive::u32,
            }
        }
        pub mod pallet_nucleus {
            use super::runtime_types;
            pub mod check_nonce {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    create_nucleus {
                        name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        energy: ::core::option::Option<::core::primitive::u128>,
                        capacity: ::core::primitive::u8,
                        a2a_compatible: ::core::primitive::bool,
                    },
                    #[codec(index = 1)]
                    upload_nucleus_wasm {
                        nucleus_id: ::subxt_core::utils::AccountId32,
                        to: runtime_types::sp_core::OpaquePeerId,
                        hash: ::subxt_core::utils::H256,
                    },
                    #[codec(index = 2)]
                    register {
                        nucleus_id: ::subxt_core::utils::AccountId32,
                        signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
                    },
                    #[codec(index = 3)]
                    submit_work {
                        nucleus_id: ::subxt_core::utils::AccountId32,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    NucleusIdAlreadyExists,
                    #[codec(index = 1)]
                    NucleusNotFound,
                    #[codec(index = 2)]
                    NotAuthorized,
                    #[codec(index = 3)]
                    InvalidVrfProof,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    NucleusCreated {
                        id: ::subxt_core::utils::AccountId32,
                        name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        manager: ::subxt_core::utils::AccountId32,
                        energy: ::core::primitive::u128,
                        capacity: ::core::primitive::u8,
                        public_input: ::subxt_core::utils::H256,
                    },
                    #[codec(index = 1)]
                    NucleusUpgraded {
                        id: ::subxt_core::utils::AccountId32,
                        wasm_hash: ::subxt_core::utils::H256,
                        wasm_version: ::core::primitive::u32,
                        wasm_location: runtime_types::sp_core::OpaquePeerId,
                    },
                    #[codec(index = 2)]
                    InstanceRegistered {
                        id: ::subxt_core::utils::AccountId32,
                        controller: ::subxt_core::utils::AccountId32,
                        node_id: ::core::option::Option<runtime_types::sp_core::OpaquePeerId>,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct NucleusChallenge<_0, _1> {
                    pub submissions: ::subxt_core::alloc::vec::Vec<(_0, ::core::primitive::u64)>,
                    pub public_input: _1,
                    pub requires: ::core::primitive::u8,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct NucleusEquation<_0, _1, _2> {
                    pub name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub manager: _0,
                    pub a2a_compatible: ::core::primitive::bool,
                    pub wasm_hash: _1,
                    pub wasm_version: ::core::primitive::u32,
                    pub wasm_location: ::core::option::Option<_2>,
                    pub energy: ::core::primitive::u128,
                    pub current_event: ::core::primitive::u64,
                    pub root_state: _1,
                    pub capacity: ::core::primitive::u8,
                }
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    Offence {
                        kind: [::core::primitive::u8; 16usize],
                        timeslot: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                }
            }
        }
        pub mod pallet_restaking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    update_validators {
                        payload: runtime_types::pallet_restaking::types::ObservationsPayload<
                            runtime_types::sp_runtime::MultiSigner,
                            ::core::primitive::u32,
                        >,
                        signature: runtime_types::sp_runtime::MultiSignature,
                    },
                    #[codec(index = 1)]
                    add_restaking_platform {
                        platform_source_name: ::subxt_core::alloc::string::String,
                        url: ::subxt_core::alloc::string::String,
                        middleware_address: ::subxt_core::alloc::string::String,
                    },
                    #[codec(index = 2)]
                    set_rewards_pre_point { value: ::core::primitive::u128 },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    WrongSetId,
                    #[codec(index = 1)]
                    InvalidNotificationId,
                    #[codec(index = 2)]
                    NotValidator,
                    #[codec(index = 3)]
                    NextNotificationIdOverflow,
                    #[codec(index = 4)]
                    InvalidActiveTotalStake,
                    #[codec(index = 5)]
                    NotActivated,
                    #[codec(index = 6)]
                    InvalidReceiverId,
                    #[codec(index = 7)]
                    NextSetIdOverflow,
                    #[codec(index = 8)]
                    ObservationsExceededLimit,
                    #[codec(index = 9)]
                    InvalidRewardsPerPoint,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    NewPlannedValidators {
                        set_id: ::core::primitive::u32,
                        validators: ::subxt_core::alloc::vec::Vec<(
                            ::subxt_core::utils::AccountId32,
                            ::core::primitive::u128,
                        )>,
                    },
                    #[codec(index = 1)]
                    UnlockFailed {
                        sender: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        sequence: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    MintNep141Failed {
                        token_id: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        sender: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt_core::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        sequence: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    UnlockNonfungibleFailed {
                        collection: ::core::primitive::u128,
                        item: ::core::primitive::u128,
                        sender: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        receiver: ::subxt_core::utils::AccountId32,
                        sequence: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    RewardsPerPointUpdated { value: ::core::primitive::u128 },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct BurnEvent<_0> {
                    pub index: ::core::primitive::u32,
                    pub sender_id: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub receiver: _0,
                    pub amount: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct BurnNftEvent<_0> {
                    pub index: ::core::primitive::u32,
                    pub sender_id: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub receiver: _0,
                    pub collection: ::core::primitive::u128,
                    pub item: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct EraRewardDetailsValue {
                    pub total: ::core::primitive::u128,
                    pub timestamp: ::core::primitive::u64,
                    pub details: ::subxt_core::alloc::vec::Vec<
                        runtime_types::pallet_restaking::types::OperatorReward,
                    >,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct LockAssetEvent<_0> {
                    pub index: ::core::primitive::u32,
                    pub token_id: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub sender_id: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub receiver: _0,
                    pub amount: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum NotificationResult {
                    #[codec(index = 0)]
                    Success,
                    #[codec(index = 1)]
                    UnlockFailed,
                    #[codec(index = 2)]
                    AssetMintFailed,
                    #[codec(index = 3)]
                    AssetGetFailed,
                    #[codec(index = 4)]
                    NftUnlockFailed,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Observation<_0> {
                    #[codec(index = 0)]
                    UpdateValidatorSet(runtime_types::pallet_restaking::types::ValidatorSet<_0>),
                    #[codec(index = 1)]
                    LockAsset(runtime_types::pallet_restaking::types::LockAssetEvent<_0>),
                    #[codec(index = 2)]
                    Burn(runtime_types::pallet_restaking::types::BurnEvent<_0>),
                    #[codec(index = 3)]
                    BurnNft(runtime_types::pallet_restaking::types::BurnNftEvent<_0>),
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum ObservationType {
                    #[codec(index = 0)]
                    UpdateValidatorSet,
                    #[codec(index = 1)]
                    Burn,
                    #[codec(index = 2)]
                    LockAsset,
                    #[codec(index = 3)]
                    BurnNft,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ObservationsPayload<_0, _1> {
                    pub public: _0,
                    pub key_data: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub block_number: _1,
                    pub observations: ::subxt_core::alloc::vec::Vec<
                        runtime_types::pallet_restaking::validator_data::ValidatorData,
                    >,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct OperatorReward {
                    pub validator: runtime_types::pallet_restaking::validator_data::ValidatorData,
                    pub amount: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Validator<_0> {
                    pub validator_id_in_appchain: _0,
                    pub total_stake: ::core::primitive::u128,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ValidatorSet<_0> {
                    pub set_id: ::core::primitive::u32,
                    pub validators: ::subxt_core::alloc::vec::Vec<
                        runtime_types::pallet_restaking::types::Validator<_0>,
                    >,
                }
            }
            pub mod validator_data {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ValidatorData {
                    pub operator: [::core::primitive::u8; 20usize],
                    pub stake: ::core::primitive::u128,
                    pub key: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub strategies: ::subxt_core::alloc::vec::Vec<[::core::primitive::u8; 20usize]>,
                    pub source: ::subxt_core::alloc::string::String,
                }
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::vrs_runtime::opaque::SessionKeys,
                        proof: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::subxt_core::alloc::boxed::Box<
                            runtime_types::vrs_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::subxt_core::alloc::boxed::Box<
                            runtime_types::vrs_runtime::RuntimeCall,
                        >,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who:
                            ::subxt_core::utils::MultiAddress<::subxt_core::utils::AccountId32, ()>,
                        call: ::subxt_core::alloc::boxed::Box<
                            runtime_types::vrs_runtime::RuntimeCall,
                        >,
                    },
                    #[codec(index = 4)]
                    remove_key,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        old: ::core::option::Option<::subxt_core::utils::AccountId32>,
                        new: ::subxt_core::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    KeyRemoved,
                    #[codec(index = 3)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_swap {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    create_exchange {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        currency_amount: ::core::primitive::u128,
                        token_amount: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    add_liquidity {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        currency_amount: ::core::primitive::u128,
                        min_liquidity: ::core::primitive::u128,
                        max_tokens: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    remove_liquidity {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        liquidity_amount: ::core::primitive::u128,
                        min_currency: ::core::primitive::u128,
                        min_tokens: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    currency_to_asset {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        amount: runtime_types::pallet_swap::pallet::TradeAmount<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                        recipient: ::core::option::Option<::subxt_core::utils::AccountId32>,
                    },
                    #[codec(index = 4)]
                    asset_to_currency {
                        asset_id: runtime_types::vrs_primitives::AssetId,
                        amount: runtime_types::pallet_swap::pallet::TradeAmount<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                        recipient: ::core::option::Option<::subxt_core::utils::AccountId32>,
                    },
                    #[codec(index = 5)]
                    asset_to_asset {
                        sold_asset_id: runtime_types::vrs_primitives::AssetId,
                        bought_asset_id: runtime_types::vrs_primitives::AssetId,
                        amount: runtime_types::pallet_swap::pallet::TradeAmount<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                        recipient: ::core::option::Option<::subxt_core::utils::AccountId32>,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    AssetNotFound,
                    #[codec(index = 1)]
                    ExchangeAlreadyExists,
                    #[codec(index = 2)]
                    TokenIdTaken,
                    #[codec(index = 3)]
                    BalanceTooLow,
                    #[codec(index = 4)]
                    NotEnoughTokens,
                    #[codec(index = 5)]
                    ProviderLiquidityTooLow,
                    #[codec(index = 6)]
                    ExchangeNotFound,
                    #[codec(index = 7)]
                    TradeAmountIsZero,
                    #[codec(index = 8)]
                    TokenAmountIsZero,
                    #[codec(index = 9)]
                    MaxTokensIsZero,
                    #[codec(index = 10)]
                    CurrencyAmountIsZero,
                    #[codec(index = 11)]
                    CurrencyAmountTooHigh,
                    #[codec(index = 12)]
                    CurrencyAmountTooLow,
                    #[codec(index = 13)]
                    MinLiquidityIsZero,
                    #[codec(index = 14)]
                    MaxTokensTooLow,
                    #[codec(index = 15)]
                    MinLiquidityTooHigh,
                    #[codec(index = 16)]
                    LiquidityAmountIsZero,
                    #[codec(index = 17)]
                    MinCurrencyIsZero,
                    #[codec(index = 18)]
                    MinTokensIsZero,
                    #[codec(index = 19)]
                    MinCurrencyTooHigh,
                    #[codec(index = 20)]
                    MinTokensTooHigh,
                    #[codec(index = 21)]
                    MaxCurrencyTooLow,
                    #[codec(index = 22)]
                    MinBoughtTokensTooHigh,
                    #[codec(index = 23)]
                    MaxSoldTokensTooLow,
                    #[codec(index = 24)]
                    NotEnoughLiquidity,
                    #[codec(index = 25)]
                    Overflow,
                    #[codec(index = 26)]
                    Underflow,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    ExchangeCreated(
                        runtime_types::vrs_primitives::AssetId,
                        runtime_types::vrs_primitives::AssetId,
                    ),
                    #[codec(index = 1)]
                    LiquidityAdded(
                        ::subxt_core::utils::AccountId32,
                        runtime_types::vrs_primitives::AssetId,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    LiquidityRemoved(
                        ::subxt_core::utils::AccountId32,
                        runtime_types::vrs_primitives::AssetId,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    CurrencyTradedForAsset(
                        runtime_types::vrs_primitives::AssetId,
                        ::subxt_core::utils::AccountId32,
                        ::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    AssetTradedForCurrency(
                        runtime_types::vrs_primitives::AssetId,
                        ::subxt_core::utils::AccountId32,
                        ::subxt_core::utils::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Exchange<_0, _1, _2> {
                    pub asset_id: _0,
                    pub currency_reserve: _1,
                    pub token_reserve: _2,
                    pub liquidity_token_id: _0,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum TradeAmount<_0, _1> {
                    #[codec(index = 0)]
                    FixedInput { input_amount: _0, min_output: _1 },
                    #[codec(index = 1)]
                    FixedOutput { max_input: _0, output_amount: _1 },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    TransactionFeePaid {
                        who: ::subxt_core::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_validator {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Error {}
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Event {
                    #[codec(index = 0)]
                    ForceEra {
                        mode: runtime_types::pallet_validator::types::Forcing,
                    },
                    #[codec(index = 1)]
                    TriggerNewEra,
                    #[codec(index = 2)]
                    EraPaid {
                        era_index: ::core::primitive::u32,
                        validator_payout: ::core::primitive::u128,
                        staker_payout: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    EraStarted(::core::primitive::u32),
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct ActiveEraInfo {
                    pub index: ::core::primitive::u32,
                    pub set_id: ::core::primitive::u32,
                    pub start: ::core::option::Option<::core::primitive::u64>,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum Forcing {
                    #[codec(index = 0)]
                    NotForcing,
                    #[codec(index = 1)]
                    ForceNew,
                    #[codec(index = 2)]
                    ForceNone,
                    #[codec(index = 3)]
                    ForceAlways,
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: CompactAs,
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Percent(pub ::core::primitive::u8);
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub enum PreDigest {
                    #[codec(index = 1)]
                    Primary(runtime_types::sp_consensus_babe::digests::PrimaryPreDigest),
                    #[codec(index = 2)]
                    SecondaryPlain(
                        runtime_types::sp_consensus_babe::digests::SecondaryPlainPreDigest,
                    ),
                    #[codec(index = 3)]
                    SecondaryVRF(runtime_types::sp_consensus_babe::digests::SecondaryVRFPreDigest),
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct PrimaryPreDigest {
                    pub authority_index: ::core::primitive::u32,
                    pub slot: runtime_types::sp_consensus_slots::Slot,
                    pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SecondaryPlainPreDigest {
                    pub authority_index: ::core::primitive::u32,
                    pub slot: runtime_types::sp_consensus_slots::Slot,
                }
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SecondaryVRFPreDigest {
                    pub authority_index: ::core::primitive::u32,
                    pub slot: runtime_types::sp_consensus_slots::Slot,
                    pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: CompactAs,
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod vrf {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct VrfSignature {
                        pub pre_output: [::core::primitive::u8; 32usize],
                        pub proof: [::core::primitive::u8; 64usize],
                    }
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct OpaquePeerId(pub ::subxt_core::alloc::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs: ::subxt_core::alloc::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem,
                        >,
                    }
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::subxt_core::alloc::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt_core :: ext :: codec :: Decode,
                        :: subxt_core :: ext :: codec :: Encode,
                        :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                        :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt_core :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt_core::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt_core::utils::H256,
                        pub extrinsics_root: ::subxt_core::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 64usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 64usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 65usize]),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519([::core::primitive::u8; 32usize]),
                #[codec(index = 1)]
                Sr25519([::core::primitive::u8; 32usize]),
                #[codec(index = 2)]
                Ecdsa([::core::primitive::u8; 33usize]),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::subxt_core::alloc::vec::Vec<
                    ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                >,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: ::subxt_core::alloc::vec::Vec<_0>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::subxt_core::alloc::string::String,
                pub impl_name: ::subxt_core::alloc::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis: ::subxt_core::alloc::vec::Vec<(
                    [::core::primitive::u8; 8usize],
                    ::core::primitive::u32,
                )>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod vrs_primitives {
            use super::runtime_types;
            pub mod keys {
                use super::runtime_types;
                pub mod restaking {
                    use super::runtime_types;
                    pub mod app_sr25519 {
                        use super::runtime_types;
                        #[derive(
                            :: subxt_core :: ext :: codec :: Decode,
                            :: subxt_core :: ext :: codec :: Encode,
                            :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt_core :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                        pub struct Public(pub [::core::primitive::u8; 32usize]);
                    }
                }
                pub mod vrf {
                    use super::runtime_types;
                    pub mod app_sr25519 {
                        use super::runtime_types;
                        #[derive(
                            :: subxt_core :: ext :: codec :: Decode,
                            :: subxt_core :: ext :: codec :: Encode,
                            :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                            :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                            Debug,
                        )]
                        # [codec (crate = :: subxt_core :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                        pub struct Public(pub [::core::primitive::u8; 32usize]);
                    }
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct AssetId(pub ::subxt_core::alloc::string::String);
        }
        pub mod vrs_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    :: subxt_core :: ext :: codec :: Decode,
                    :: subxt_core :: ext :: codec :: Encode,
                    :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                    :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt_core :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
                pub struct SessionKeys {
                    pub babe: runtime_types::sp_consensus_babe::app::Public,
                    pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
                    pub authority: runtime_types::sp_authority_discovery::app::Public,
                    pub restaking:
                        runtime_types::vrs_primitives::keys::restaking::app_sr25519::Public,
                    pub vrf: runtime_types::vrs_primitives::keys::vrf::app_sr25519::Public,
                    pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                }
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 2)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 4)]
                Restaking(runtime_types::pallet_restaking::pallet::Call),
                #[codec(index = 7)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 8)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 10)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 11)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 13)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 15)]
                Nucleus(runtime_types::pallet_nucleus::pallet::Call),
                #[codec(index = 16)]
                Assets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 17)]
                Swap(runtime_types::pallet_swap::pallet::Call),
                #[codec(index = 18)]
                A2A(runtime_types::pallet_a2a::pallet::Call),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 2)]
                Babe(runtime_types::pallet_babe::pallet::Error),
                #[codec(index = 4)]
                Restaking(runtime_types::pallet_restaking::pallet::Error),
                #[codec(index = 6)]
                Validator(runtime_types::pallet_validator::pallet::Error),
                #[codec(index = 7)]
                Session(runtime_types::pallet_session::pallet::Error),
                #[codec(index = 8)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 10)]
                ImOnline(runtime_types::pallet_im_online::pallet::Error),
                #[codec(index = 11)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 13)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 15)]
                Nucleus(runtime_types::pallet_nucleus::pallet::Error),
                #[codec(index = 16)]
                Assets(runtime_types::pallet_assets::pallet::Error),
                #[codec(index = 17)]
                Swap(runtime_types::pallet_swap::pallet::Error),
                #[codec(index = 18)]
                A2A(runtime_types::pallet_a2a::pallet::Error),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Restaking(runtime_types::pallet_restaking::pallet::Event),
                #[codec(index = 6)]
                Validator(runtime_types::pallet_validator::pallet::Event),
                #[codec(index = 7)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 8)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 10)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 11)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 12)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 13)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 14)]
                Offences(runtime_types::pallet_offences::pallet::Event),
                #[codec(index = 15)]
                Nucleus(runtime_types::pallet_nucleus::pallet::Event),
                #[codec(index = 16)]
                Assets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 17)]
                Swap(runtime_types::pallet_swap::pallet::Event),
                #[codec(index = 18)]
                A2A(runtime_types::pallet_a2a::pallet::Event),
            }
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeFreezeReason {}
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {}
        }
        pub mod vrs_support {
            use super::runtime_types;
            #[derive(
                :: subxt_core :: ext :: codec :: Decode,
                :: subxt_core :: ext :: codec :: Encode,
                :: subxt_core :: ext :: scale_decode :: DecodeAsType,
                :: subxt_core :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt_core :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt_core :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt_core :: ext :: scale_encode")]
            pub struct EraRewardPoints<_0> {
                pub total: ::core::primitive::u128,
                pub individual: ::subxt_core::utils::KeyedVec<_0, ::core::primitive::u128>,
            }
        }
    }
}
