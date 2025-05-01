use sc_service::ChainType;
use sp_authority_discovery::AuthorityId;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use vrs_primitives::keys::{restaking::AuthorityId as RestakingId, vrf::AuthorityId as VrfId};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use vrs_runtime::opaque::SessionKeys;
use vrs_runtime::{AccountId, Signature, WASM_BINARY};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.

pub fn authority_keys_from_seed(
    s: &str,
) -> (
    AccountId,
    AuraId,
    GrandpaId,
    AuthorityId,
    RestakingId,
    VrfId,
    ImOnlineId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&s),
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<AuthorityId>(s),
        get_from_seed::<RestakingId>(s),
        get_from_seed::<VrfId>(s),
        get_from_seed::<ImOnlineId>(s),
    )
}

pub fn development_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Development")
    .with_id("dev")
    .with_protocol_id("vrs")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(testnet_genesis(
        // Initial PoA authorities
        vec![authority_keys_from_seed("Alice")],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        ],
        vec![(
            1,
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            false,
            1,
        )],
        vec![(
            1,
            "USDT".as_bytes().to_vec(),
            "USDT".as_bytes().to_vec(),
            18,
        )],
        vec![],
        true,
    ))
    .build())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    Ok(ChainSpec::builder(
        WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
        None,
    )
    .with_name("Local Testnet")
    .with_id("local_testnet")
    .with_chain_type(ChainType::Local)
    .with_genesis_config_patch(testnet_genesis(
        // Initial PoA authorities
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
            authority_keys_from_seed("Charlie"),
            // authority_keys_from_seed("Dave"),
        ],
        // Sudo account
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        // Pre-funded accounts
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        vec![(
            1,
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            false,
            1,
        )],
        vec![(
            1,
            "USDT".as_bytes().to_vec(),
            "USDT".as_bytes().to_vec(),
            18,
        )],
        vec![],
        true,
    ))
    .build())
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AuraId,
        GrandpaId,
        AuthorityId,
        RestakingId,
        VrfId,
        ImOnlineId,
    )>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    assets: Vec<(u32, AccountId, bool, u128)>,
    metadata: Vec<(u32, Vec<u8>, Vec<u8>, u8)>,
    asset_accounts: Vec<(u32, AccountId, u128)>,
    _enable_println: bool,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 1u64 << 60)).collect::<Vec<_>>(),
        },
        "sudo": {
            "key": Some(root_key),
        },
        "nucleus": {
            "preset": initial_authorities.iter().take(1).cloned().map(|x| x.0).collect::<Vec<_>>(),
        },
        "restaking": {
            "validators": initial_authorities.iter().cloned().map(|k| (k.0, 10000, "0x0000000000000000000000000000000000000000", "Original")).collect::<Vec<_>>(),
        },
        "session":  {
            "keys": initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.1.clone(),
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "assets": {
            "assets": assets,
            "metadata": metadata,
            "accounts": asset_accounts,
            "next_asset_id": None::<u32>,
        }
    })
}

fn session_keys(
    aura: AuraId,
    grandpa: GrandpaId,
    authority: AuthorityId,
    restaking: RestakingId,
    vrf: VrfId,
    im_online: ImOnlineId,
) -> SessionKeys {
    SessionKeys {
        aura,
        grandpa,
        authority,
        restaking,
        vrf,
        im_online,
    }
}
