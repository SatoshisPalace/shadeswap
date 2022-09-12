use crate::{
    execute::{
        add_address_to_whitelist, add_liquidity, get_estimated_lp_token, get_shade_dao_info,
        load_trade_history_query, query_calculate_price, query_liquidity, register_lp_token,
        remove_address_from_whitelist, remove_liquidity, set_staking_contract, swap,
        swap_simulation,
    },
    state::{config_r, config_w, trade_count_r, whitelist_r, Config},
};
use cosmwasm_std::{
    entry_point, from_binary, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdError, StdResult, SubMsg, SubMsgResult, Uint128, WasmMsg,
};
use shadeswap_shared::{
    contract_interfaces::snip20::{InitConfig, InstantiateMsg},
    core::{
        admin_r, admin_w, apply_admin_guard, create_viewing_key, set_admin_guard, ContractLink,
        TokenAmount, TokenType,
    },
    msg::amm_pair::{ExecuteMsg, InitMsg, InvokeMsg, QueryMsg, QueryMsgResponse},
    msg::staking::InitMsg as StakingInitMsg,
    Contract,
};

use crate::execute::register_pair_token;

const AMM_PAIR_CONTRACT_VERSION: u32 = 1;
pub const INSTANTIATE_LP_TOKEN_REPLY_ID: u64 = 1u64;
pub const INSTANTIATE_STAKING_CONTRACT_REPLY_ID: u64 = 2u64;
pub const BLOCK_SIZE: usize = 256;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    if msg.pair.0 == msg.pair.1 {
        return Err(StdError::generic_err(
            "Creating Pair Contract with the same two tokens.",
        ));
    }

    let mut messages = vec![];
    let viewing_key = create_viewing_key(&env, &_info, msg.prng_seed.clone(), msg.entropy.clone());
    register_pair_token(&env, &mut messages, &msg.pair.0, &viewing_key)?;
    register_pair_token(&env, &mut messages, &msg.pair.1, &viewing_key)?;

    let init_snip20_msg = InstantiateMsg {
        name: format!(
            "SHADESWAP Liquidity Provider (LP) token for {}-{}",
            &msg.pair.0, &msg.pair.1
        ),
        admin: Some(env.contract.address.to_string()),
        symbol: "SWAP-LP".to_string(),
        decimals: 18,
        initial_balances: None,
        prng_seed: msg.prng_seed.clone(),
        config: Some(InitConfig {
            public_total_supply: Some(true),
            enable_deposit: Some(false),
            enable_redeem: Some(false),
            enable_mint: Some(true),
            enable_burn: Some(true),
            enable_transfer: Some(true),
        }),
        query_auth: None,
    };

    let mut response = Response::new();

    response = response.add_submessage(SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Instantiate {
            code_id: msg.lp_token_contract.id,
            msg: to_binary(&init_snip20_msg)?,
            label: format!(
                "{}-{}-ShadeSwap-Pair-Token-{}",
                &msg.pair.0, &msg.pair.1, &env.contract.address
            ),
            code_hash: msg.lp_token_contract.code_hash.clone(),
            funds: vec![],
        }),
        INSTANTIATE_LP_TOKEN_REPLY_ID,
    ));

    match msg.staking_contract {
        Some(c) => {
            response = response.add_submessage(SubMsg::reply_on_success(
                CosmosMsg::Wasm(WasmMsg::Instantiate {
                    code_id: c.contract_info.id,
                    label: format!("ShadeSwap-Pair-Staking-Contract-{}", &env.contract.address),
                    msg: to_binary(&StakingInitMsg {
                        staking_amount: c.amount,
                        reward_token: c.reward_token.clone(),
                        pair_contract: ContractLink {
                            address: env.contract.address.clone(),
                            code_hash: env.contract.code_hash.clone(),
                        },
                        prng_seed: msg.prng_seed.clone(),
                    })?,
                    code_hash: c.contract_info.code_hash.clone(),
                    funds: vec![],
                }),
                INSTANTIATE_STAKING_CONTRACT_REPLY_ID,
            ));
        }
        _ => {
            ();
        }
    }

    let config = Config {
        factory_contract: msg.factory_info.clone(),
        lp_token: ContractLink {
            code_hash: msg.lp_token_contract.code_hash,
            address: Addr::unchecked(""),
        },
        pair: msg.pair,
        viewing_key: viewing_key,
        custom_fee: msg.custom_fee.clone(),
        staking_contract: None,
    };

    config_w(deps.storage).save(&config)?;

    match msg.admin {
        Some(admin) => admin_w(deps.storage).save(&admin)?,
        None => println!("No admin given"),
    }
    Ok(response.add_attribute("created_exchange_address", env.contract.address))
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive {
            from, amount, msg, ..
        } => receiver_callback(deps, env, info, Addr::unchecked(from), amount, msg),
        ExecuteMsg::AddLiquidityToAMMContract {
            deposit,
            slippage,
            staking,
        } => add_liquidity(deps, env, &info, deposit, slippage, staking),
        ExecuteMsg::SetCustomPairFee { custom_fee } => {
            apply_admin_guard(&info.sender, deps.storage)?;
            let config = config_r(deps.storage).load()?;
            config_w(deps.storage).save(&Config { factory_contract: config.factory_contract, lp_token: config.lp_token, staking_contract: config.staking_contract, pair: config.pair, viewing_key: config.viewing_key, custom_fee: custom_fee });
            Ok(Response::default())
        }
        ExecuteMsg::SetAMMPairAdmin { admin } => {
            set_admin_guard(deps.storage, info, Addr::unchecked(admin))
        }
        ExecuteMsg::AddWhiteListAddress { address } => {
            apply_admin_guard(&info.sender, deps.storage)?;
            add_address_to_whitelist(deps.storage, Addr::unchecked(address), env)
        }
        ExecuteMsg::RemoveWhitelistAddresses { addresses } => {
            apply_admin_guard(&info.sender, deps.storage)?;
            remove_address_from_whitelist(deps.storage, addresses, env)
        }
        ExecuteMsg::SwapTokens {
            offer,
            expected_return,
            to,
            router_link,
            callback_signature,
        } => {
            // this is assert if token is SCRT if not then swapp will be called via SNIP20 Interface
            if !offer.token.is_native_token() {
                return Err(StdError::generic_err("Use the receive interface"));
            }

            offer.assert_sent_native_token_balance(&info)?;
            let config_settings = config_r(deps.storage).load()?;
            let sender = info.sender.clone();
            swap(
                deps,
                env,
                config_settings,
                sender,
                to,
                offer,
                expected_return,
                router_link,
                callback_signature,
            )
        }
    }
}

fn receiver_callback(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    from: Addr,
    amount: Uint128,
    msg: Option<Binary>,
) -> StdResult<Response> {
    let msg = msg.ok_or_else(|| {
        StdError::generic_err("Receiver callback \"msg\" parameter cannot be empty.")
    })?;

    let config = config_r(deps.storage).load()?;
    let from_caller = from.clone();
    match from_binary(&msg)? {
        InvokeMsg::SwapTokens {
            to,
            expected_return,
            router_link,
            callback_signature,
        } => {
            for token in config.pair.into_iter() {
                match token {
                    TokenType::CustomToken { contract_addr, .. } => {
                        if *contract_addr == info.sender {
                            let offer = TokenAmount {
                                token: token.clone(),
                                amount,
                            };

                            return swap(
                                deps,
                                env,
                                config,
                                from,
                                Some(Addr::unchecked(
                                    to.ok_or_else(|| StdError::generic_err("".to_string()))?,
                                )),
                                offer,
                                expected_return,
                                router_link,
                                callback_signature,
                            );
                        }
                    }
                    _ => continue,
                }
            }

            Err(StdError::generic_err("".to_string()))
        }
        InvokeMsg::RemoveLiquidity { from } => {
            if config.lp_token.address != info.sender {
                return Err(StdError::generic_err("".to_string()));
            }
            match from {
                Some(address) => remove_liquidity(deps, env, amount, Addr::unchecked(address)),
                None => remove_liquidity(deps, env, amount, from_caller),
            }
        }
    }
}

pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPairInfo {} => {
            let config = config_r(deps.storage).load()?;
            let balances = config.pair.query_balances(
                deps,
                env.contract.address.to_string(),
                config.viewing_key.0,
            )?;
            let total_liquidity = query_liquidity(deps, &config.lp_token)?;
            to_binary(&QueryMsgResponse::GetPairInfo {
                liquidity_token: config.lp_token,
                factory: config.factory_contract,
                pair: config.pair,
                amount_0: balances[0],
                amount_1: balances[1],
                total_liquidity,
                contract_version: AMM_PAIR_CONTRACT_VERSION,
            })
        }
        QueryMsg::GetTradeHistory { pagination } => {
            let data = load_trade_history_query(deps, pagination)?;
            to_binary(&QueryMsgResponse::GetTradeHistory { data })
        }
        QueryMsg::GetAdmin {} => {
            let admin_address = admin_r(deps.storage).load()?;
            to_binary(&QueryMsgResponse::GetAdminAddress {
                address: admin_address,
            })
        }
        QueryMsg::GetWhiteListAddress {} => {
            let stored_addr = whitelist_r(deps.storage).load()?;
            to_binary(&QueryMsgResponse::GetWhiteListAddress {
                addresses: stored_addr,
            })
        }
        QueryMsg::GetTradeCount {} => {
            let count = trade_count_r(deps.storage).load()?;
            to_binary(&QueryMsgResponse::GetTradeCount { count })
        }
        QueryMsg::GetStakingContract {} => {
            let staking_contract = config_r(deps.storage).load()?.staking_contract;
            to_binary(&QueryMsgResponse::StakingContractInfo {
                staking_contract: staking_contract,
            })
        }
        QueryMsg::GetEstimatedPrice { offer, exclude_fee } => {
            let swap_result = query_calculate_price(deps, env, offer, exclude_fee)?;
            to_binary(&QueryMsgResponse::EstimatedPrice {
                estimated_price: swap_result.price,
            })
        }
        QueryMsg::SwapSimulation { offer } => swap_simulation(deps, env, offer),
        QueryMsg::GetShadeDaoInfo {} => get_shade_dao_info(deps),
        QueryMsg::GetEstimatedLiquidity { deposit, slippage } => {
            get_estimated_lp_token(deps, env, deposit, slippage)
        }
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match (msg.id, msg.result) {
        (INSTANTIATE_LP_TOKEN_REPLY_ID, SubMsgResult::Ok(s)) => match s.data {
            Some(x) => {
                let contract_address = String::from_utf8(x.to_vec())?;
                register_lp_token(
                    deps,
                    _env,
                    Contract {
                        address: Addr::unchecked(contract_address),
                        code_hash: "".to_string(),
                    },
                );
                Ok(Response::default())
            }
            None => todo!(),
        },
        (INSTANTIATE_STAKING_CONTRACT_REPLY_ID, SubMsgResult::Ok(s)) => match s.data {
            Some(x) => {
                let contract_address = String::from_utf8(x.to_vec())?;
                set_staking_contract(
                    deps,
                    _env,
                    Some(ContractLink {
                        address: Addr::unchecked(contract_address),
                        code_hash: "".to_string(),
                    }),
                );
                Ok(Response::default())
            }
            None => todo!(),
        },
        _ => Err(StdError::generic_err(format!("Unknown reply id"))),
    }
}
