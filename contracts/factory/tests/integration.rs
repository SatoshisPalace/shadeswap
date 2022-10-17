use cosmwasm_std::{
    to_binary, Addr, Empty,
};
use factory::contract::{execute, instantiate, query};
use secret_multi_test::{App, Contract, ContractWrapper, Executor};
use shadeswap_shared::{
    core::{ContractInstantiationInfo, ContractLink},
    factory::{InitMsg, QueryResponse, QueryMsg},
    utils::testing::TestingExt
};

pub fn contract_counter() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(execute, instantiate, query);
    Box::new(contract)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn factory_integration_tests() {
    let mut router = App::default();
    let owner = Addr::unchecked("owner");

    let init_msg = InitMsg {
        pair_contract: ContractInstantiationInfo {
            code_hash: "".to_string(),
            id: 0u64,
        },
        amm_settings: shadeswap_shared::amm_pair::AMMSettings {
            lp_fee: shadeswap_shared::core::Fee { nom: 2, denom: 100 },
            shade_dao_fee: shadeswap_shared::core::Fee { nom: 2, denom: 100 },
            shade_dao_address: ContractLink {
                address: Addr::unchecked("".to_string()),
                code_hash: "".to_string(),
            },
        },
        lp_token_contract: ContractInstantiationInfo {
            code_hash: "".to_string(),
            id: 0u64,
        },
        prng_seed: to_binary(&"".to_string()).unwrap(),
        api_key: "api_key".to_string(),
        authenticator: None,
    };
    let counter_contract_code_id = router.store_code(contract_counter());

    let mocked_contract_addr = router
        .instantiate_contract(
            counter_contract_code_id,
            owner.clone(),
            &init_msg,
            &[],
            "counter",
            None,
        )
        .unwrap();

    println!("{}", mocked_contract_addr.address.to_string());

    let query: QueryResponse = router.query_test(mocked_contract_addr,to_binary(&QueryMsg::GetConfig { }).unwrap()).unwrap();
    match query {
        QueryResponse::GetConfig { pair_contract: _, amm_settings, lp_token_contract: _, authenticator: _ } => {
            assert_eq!(amm_settings.lp_fee, shadeswap_shared::core::Fee { nom: 2, denom: 100 });
            assert_eq!(amm_settings.shade_dao_fee, shadeswap_shared::core::Fee { nom: 2, denom: 100 });
        },
        _ => panic!("Query Responsedoes not match")
    }
}

