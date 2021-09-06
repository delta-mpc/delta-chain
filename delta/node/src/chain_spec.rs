// A struct wraps Vec<u8>, represents as our `PeerId`.
use std::{collections::BTreeMap, str::FromStr};

use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{H160, Pair, Public, sr25519, U256};
use sp_core::OpaquePeerId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::app_crypto::Ss58Codec;
use sp_runtime::traits::{IdentifyAccount, Verify};

use delta_runtime::{
    AccountId, AuraConfig, BalancesConfig, EthereumConfig, EVMConfig, GenesisConfig, GrandpaConfig,
    NodeAuthorizationConfig, opaque::SessionKeys, SessionConfig, Signature, SudoConfig,
    SystemConfig, ValidatorSetConfig, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId, AccountId) {
    (
        get_from_seed::<AuraId>(s),
        get_from_seed::<GrandpaId>(s),
        get_account_id_from_seed::<sr25519::Public>(s)
    )
}

fn session_keys(
    aura: AuraId,
    grandpa: GrandpaId,
) -> SessionKeys {
    SessionKeys { aura, grandpa }
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
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
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
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
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn delta_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "DeltaChain",
        // ID
        "delta_chain",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![(
                    AuraId::from_ss58check("5Gjy51XjttHf5mA7M1zqoKCQiiGjxmw3nGztaJyCRid4ePDE").unwrap(),
                    GrandpaId::from_ss58check("5EEEv4XpWKkhtC57dJbsqCaNkx8R7DfMEi2Ue8krrLfv58SG").unwrap(),
                    AccountId::from_ss58check("5Gjy51XjttHf5mA7M1zqoKCQiiGjxmw3nGztaJyCRid4ePDE").unwrap(),
                )],
                // Sudo account
                AccountId::from_ss58check("5Gjy51XjttHf5mA7M1zqoKCQiiGjxmw3nGztaJyCRid4ePDE").unwrap(),
                // Pre-funded accounts
                vec![
                    AccountId::from_ss58check("5Gjy51XjttHf5mA7M1zqoKCQiiGjxmw3nGztaJyCRid4ePDE").unwrap(),
                    AccountId::from_ss58check("5HbNR8NG9GcoaWtTDi9dKv6zuYKHBVXpwaZba3n1qWJSMt9W").unwrap(),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId, AccountId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        validator_set: ValidatorSetConfig {
            validators: initial_authorities.iter().map(|x| x.2.clone()).collect::<Vec<_>>(),
        },
        session: SessionConfig {
            keys: initial_authorities.iter().map(|x| {
                (x.2.clone(), x.2.clone(), session_keys(x.0.clone(), x.1.clone()))
            }).collect::<Vec<_>>(),
        },
        aura: AuraConfig {
            authorities: vec![],
        },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
        evm: EVMConfig {
            accounts: {
                let mut map = BTreeMap::new();
                map.insert(
                    // H160 address of Alice dev account
                    // Derived from SS58 (42 prefix) address
                    // SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                    // hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
                    // Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
                    H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    H160::from_str("cee2b721fc2fcbb3c136effec5d555c9f9c97db1")
                        .expect("internal H160 is valid; qed"),
                    pallet_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map
            },
        },
        ethereum: EthereumConfig {},
        dynamic_fee: Default::default(),
        node_authorization: NodeAuthorizationConfig {
            nodes: vec![
                (
                    OpaquePeerId(bs58::decode("12D3KooWKpxeRfkRPmCQUb9PCBVPgdo139n3g8SNAKE3Ap4V6bWy").into_vec().unwrap()),
                    endowed_accounts[0].clone()
                ),
                (
                    OpaquePeerId(bs58::decode("12D3KooWAQvUgm4nJ1uSgG9dZCYe1J5tsnoYaHa21VJr1HCo56s9").into_vec().unwrap()),
                    endowed_accounts[0].clone()
                ),
            ],
        },
    }
}
