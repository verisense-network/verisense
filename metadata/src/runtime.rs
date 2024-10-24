#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod codegen {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 8usize] = [
        "System",
        "Timestamp",
        "Aura",
        "Grandpa",
        "Balances",
        "TransactionPayment",
        "Sudo",
        "Nucleus",
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

        pub fn aura(&self) -> aura::constants::ConstantsApi {
            aura::constants::ConstantsApi
        }

        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }

        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }

        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
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

        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }

        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
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

        pub fn nucleus(&self) -> nucleus::storage::StorageApi {
            nucleus::storage::StorageApi
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

        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
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
                155u8, 148u8, 135u8, 130u8, 112u8, 76u8, 238u8, 13u8, 50u8, 144u8, 59u8, 1u8, 66u8,
                90u8, 164u8, 255u8, 199u8, 2u8, 109u8, 212u8, 245u8, 234u8, 167u8, 61u8, 156u8,
                244u8, 102u8, 6u8, 169u8, 253u8, 0u8, 175u8,
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
                    const CALL: &'static str = "remark";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "set_heap_pages";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "set_code";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "set_code_without_checks";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "set_storage";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "kill_storage";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "kill_prefix";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "remark_with_event";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "authorize_upgrade";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "authorize_upgrade_without_checks";
                    const PALLET: &'static str = "System";
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
                    const CALL: &'static str = "apply_authorized_upgrade";
                    const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "ExtrinsicSuccess";
                const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "ExtrinsicFailed";
                const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "CodeUpdated";
                const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "NewAccount";
                const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "KilledAccount";
                const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "Remarked";
                const PALLET: &'static str = "System";
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
                const EVENT: &'static str = "UpgradeAuthorized";
                const PALLET: &'static str = "System";
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
                            97u8, 40u8, 90u8, 148u8, 110u8, 215u8, 53u8, 253u8, 129u8, 189u8, 68u8,
                            62u8, 48u8, 110u8, 192u8, 184u8, 114u8, 50u8, 126u8, 207u8, 249u8,
                            255u8, 197u8, 68u8, 228u8, 238u8, 27u8, 132u8, 3u8, 189u8, 96u8, 42u8,
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
                    const CALL: &'static str = "set";
                    const PALLET: &'static str = "Timestamp";
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
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >;
                }
                pub mod current_slot {
                    use super::runtime_types;
                    pub type CurrentSlot = runtime_types::sp_consensus_slots::Slot;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
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
                        "Aura",
                        "Authorities",
                        (),
                        [
                            95u8, 52u8, 203u8, 53u8, 254u8, 107u8, 134u8, 122u8, 95u8, 253u8, 51u8,
                            137u8, 142u8, 106u8, 237u8, 248u8, 159u8, 80u8, 41u8, 233u8, 137u8,
                            133u8, 13u8, 217u8, 176u8, 88u8, 132u8, 199u8, 241u8, 47u8, 125u8,
                            27u8,
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
                        "Aura",
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
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn slot_duration(
                    &self,
                ) -> ::subxt_core::constants::address::StaticAddress<::core::primitive::u64>
                {
                    ::subxt_core::constants::address::StaticAddress::new_static(
                        "Aura",
                        "SlotDuration",
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
                    const CALL: &'static str = "report_equivocation";
                    const PALLET: &'static str = "Grandpa";
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
                    const CALL: &'static str = "report_equivocation_unsigned";
                    const PALLET: &'static str = "Grandpa";
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
                    const CALL: &'static str = "note_stalled";
                    const PALLET: &'static str = "Grandpa";
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
                const EVENT: &'static str = "NewAuthorities";
                const PALLET: &'static str = "Grandpa";
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
                const EVENT: &'static str = "Paused";
                const PALLET: &'static str = "Grandpa";
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
                const EVENT: &'static str = "Resumed";
                const PALLET: &'static str = "Grandpa";
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
                    const CALL: &'static str = "transfer_allow_death";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "force_transfer";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "transfer_keep_alive";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "transfer_all";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "force_unreserve";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "upgrade_accounts";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "force_set_balance";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "force_adjust_total_issuance";
                    const PALLET: &'static str = "Balances";
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
                    const CALL: &'static str = "burn";
                    const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Endowed";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "DustLost";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Transfer";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "BalanceSet";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Reserved";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Unreserved";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "ReserveRepatriated";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Deposit";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Withdraw";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Slashed";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Minted";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Burned";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Suspended";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Restored";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Upgraded";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Issued";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Rescinded";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Locked";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Unlocked";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Frozen";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "Thawed";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "TotalIssuanceForced";
                const PALLET: &'static str = "Balances";
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
                const EVENT: &'static str = "TransactionFeePaid";
                const PALLET: &'static str = "TransactionPayment";
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
                    const CALL: &'static str = "sudo";
                    const PALLET: &'static str = "Sudo";
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
                    const CALL: &'static str = "sudo_unchecked_weight";
                    const PALLET: &'static str = "Sudo";
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
                    const CALL: &'static str = "set_key";
                    const PALLET: &'static str = "Sudo";
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
                    const CALL: &'static str = "sudo_as";
                    const PALLET: &'static str = "Sudo";
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
                    const CALL: &'static str = "remove_key";
                    const PALLET: &'static str = "Sudo";
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
                            65u8, 68u8, 215u8, 118u8, 74u8, 139u8, 142u8, 184u8, 11u8, 96u8, 101u8,
                            180u8, 78u8, 217u8, 8u8, 101u8, 114u8, 37u8, 194u8, 25u8, 128u8, 95u8,
                            93u8, 7u8, 50u8, 194u8, 87u8, 48u8, 254u8, 124u8, 81u8, 69u8,
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
                            71u8, 101u8, 228u8, 213u8, 171u8, 146u8, 246u8, 65u8, 97u8, 12u8,
                            252u8, 39u8, 67u8, 169u8, 34u8, 17u8, 53u8, 26u8, 85u8, 97u8, 187u8,
                            246u8, 145u8, 234u8, 106u8, 172u8, 199u8, 203u8, 233u8, 37u8, 6u8,
                            75u8,
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
                            130u8, 212u8, 103u8, 90u8, 114u8, 130u8, 211u8, 29u8, 51u8, 89u8,
                            226u8, 129u8, 136u8, 119u8, 208u8, 209u8, 9u8, 96u8, 253u8, 77u8, 79u8,
                            206u8, 64u8, 101u8, 246u8, 4u8, 53u8, 22u8, 249u8, 242u8, 152u8, 158u8,
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
                const EVENT: &'static str = "Sudid";
                const PALLET: &'static str = "Sudo";
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
                const EVENT: &'static str = "KeyChanged";
                const PALLET: &'static str = "Sudo";
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
                const EVENT: &'static str = "KeyRemoved";
                const PALLET: &'static str = "Sudo";
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
                const EVENT: &'static str = "SudoAsDone";
                const PALLET: &'static str = "Sudo";
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
                    pub wasm_hash: create_nucleus::WasmHash,
                    pub energy: create_nucleus::Energy,
                    pub capacity: create_nucleus::Capacity,
                }
                pub mod create_nucleus {
                    use super::runtime_types;
                    pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                    pub type WasmHash = ::subxt_core::utils::H256;
                    pub type Energy = ::core::option::Option<::core::primitive::u128>;
                    pub type Capacity = ::core::primitive::u8;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for CreateNucleus {
                    const CALL: &'static str = "create_nucleus";
                    const PALLET: &'static str = "Nucleus";
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
                    const CALL: &'static str = "upload_nucleus_wasm";
                    const PALLET: &'static str = "Nucleus";
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
                pub struct MockRegister {
                    pub nucleus_id: mock_register::NucleusId,
                }
                pub mod mock_register {
                    use super::runtime_types;
                    pub type NucleusId = ::subxt_core::utils::AccountId32;
                }
                impl ::subxt_core::blocks::StaticExtrinsic for MockRegister {
                    const CALL: &'static str = "mock_register";
                    const PALLET: &'static str = "Nucleus";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                pub fn create_nucleus(
                    &self,
                    name: types::create_nucleus::Name,
                    wasm_hash: types::create_nucleus::WasmHash,
                    energy: types::create_nucleus::Energy,
                    capacity: types::create_nucleus::Capacity,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::CreateNucleus>
                {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Nucleus",
                        "create_nucleus",
                        types::CreateNucleus {
                            name,
                            wasm_hash,
                            energy,
                            capacity,
                        },
                        [
                            235u8, 228u8, 147u8, 80u8, 107u8, 191u8, 232u8, 154u8, 130u8, 137u8,
                            139u8, 85u8, 204u8, 206u8, 117u8, 21u8, 194u8, 194u8, 131u8, 76u8,
                            146u8, 147u8, 144u8, 17u8, 44u8, 163u8, 238u8, 98u8, 210u8, 75u8,
                            141u8, 18u8,
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

                pub fn mock_register(
                    &self,
                    nucleus_id: types::mock_register::NucleusId,
                ) -> ::subxt_core::tx::payload::StaticPayload<types::MockRegister> {
                    ::subxt_core::tx::payload::StaticPayload::new_static(
                        "Nucleus",
                        "mock_register",
                        types::MockRegister { nucleus_id },
                        [
                            142u8, 209u8, 252u8, 91u8, 84u8, 73u8, 44u8, 138u8, 78u8, 30u8, 106u8,
                            210u8, 254u8, 118u8, 83u8, 189u8, 232u8, 243u8, 164u8, 8u8, 29u8,
                            201u8, 221u8, 244u8, 39u8, 57u8, 22u8, 238u8, 56u8, 27u8, 109u8, 12u8,
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
                pub wasm_hash: nucleus_created::WasmHash,
                pub wasm_version: nucleus_created::WasmVersion,
                pub energy: nucleus_created::Energy,
                pub capacity: nucleus_created::Capacity,
            }
            pub mod nucleus_created {
                use super::runtime_types;
                pub type Id = ::subxt_core::utils::AccountId32;
                pub type Name = ::subxt_core::alloc::vec::Vec<::core::primitive::u8>;
                pub type Manager = ::subxt_core::utils::AccountId32;
                pub type WasmHash = ::subxt_core::utils::H256;
                pub type WasmVersion = ::core::primitive::u32;
                pub type Energy = ::core::primitive::u128;
                pub type Capacity = ::core::primitive::u8;
            }
            impl ::subxt_core::events::StaticEvent for NucleusCreated {
                const EVENT: &'static str = "NucleusCreated";
                const PALLET: &'static str = "Nucleus";
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
                const EVENT: &'static str = "NucleusUpgraded";
                const PALLET: &'static str = "Nucleus";
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
                const EVENT: &'static str = "InstanceRegistered";
                const PALLET: &'static str = "Nucleus";
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
                            180u8, 125u8, 106u8, 22u8, 220u8, 45u8, 35u8, 157u8, 75u8, 189u8, 32u8,
                            237u8, 75u8, 29u8, 248u8, 100u8, 190u8, 241u8, 213u8, 143u8, 29u8,
                            75u8, 144u8, 220u8, 205u8, 16u8, 227u8, 213u8, 48u8, 151u8, 151u8,
                            81u8,
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
                            180u8, 125u8, 106u8, 22u8, 220u8, 45u8, 35u8, 157u8, 75u8, 189u8, 32u8,
                            237u8, 75u8, 29u8, 248u8, 100u8, 190u8, 241u8, 213u8, 143u8, 29u8,
                            75u8, 144u8, 220u8, 205u8, 16u8, 227u8, 213u8, 48u8, 151u8, 151u8,
                            81u8,
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
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
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
                pub mod check_non_zero_sender {
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
                    pub struct CheckNonZeroSender;
                }
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
        pub mod pallet_nucleus {
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
                    create_nucleus {
                        name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                        wasm_hash: ::subxt_core::utils::H256,
                        energy: ::core::option::Option<::core::primitive::u128>,
                        capacity: ::core::primitive::u8,
                    },
                    #[codec(index = 1)]
                    upload_nucleus_wasm {
                        nucleus_id: ::subxt_core::utils::AccountId32,
                        to: runtime_types::sp_core::OpaquePeerId,
                        hash: ::subxt_core::utils::H256,
                    },
                    #[codec(index = 2)]
                    mock_register {
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
                        wasm_hash: ::subxt_core::utils::H256,
                        wasm_version: ::core::primitive::u32,
                        energy: ::core::primitive::u128,
                        capacity: ::core::primitive::u8,
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
                pub struct NucleusEquation<_0, _1, _2> {
                    pub name: ::subxt_core::alloc::vec::Vec<::core::primitive::u8>,
                    pub manager: _0,
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
            #[derive(
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
        pub mod sp_consensus_aura {
            use super::runtime_types;
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
                }
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
            #[derive(
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
        pub mod vrs_runtime {
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
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 7)]
                Nucleus(runtime_types::pallet_nucleus::pallet::Call),
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
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 7)]
                Nucleus(runtime_types::pallet_nucleus::pallet::Error),
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
                #[codec(index = 3)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 5)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 6)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 7)]
                Nucleus(runtime_types::pallet_nucleus::pallet::Event),
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
    }
}
