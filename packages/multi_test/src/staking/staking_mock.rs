pub mod staking_mock {
    use cosmwasm_std::{Response, StdResult, MessageInfo, DepsMut, Env, entry_point, to_binary, Deps, Binary, CosmosMsg, BankMsg, Coin, Addr, Attribute};
    use schemars::JsonSchema;    
    use serde::{Deserialize, Serialize};
    use shadeswap_shared::{utils::{pad_query_result, pad_response_result}, staking::{ExecuteMsg, QueryMsg}};

    pub const BLOCK_SIZE: usize = 256;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    pub struct InitMsg{

    }    

    #[entry_point]
    pub fn instantiate(
        _deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        _msg: InitMsg,
    ) -> StdResult<Response> { 
        println!("test init staking");
        let mut response = Response::new();
        response.data = Some(env.contract.address.as_bytes().into());
        Ok(response) 
    }

    #[entry_point]
    pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        pad_query_result(
            match msg {
                QueryMsg::GetContractOwner {  } =>to_binary(""),
                QueryMsg::GetConfig {  } => to_binary(""),
                QueryMsg::WithPermit { permit: _, query: _ } => to_binary(""),
                QueryMsg::GetAdmin {  } => to_binary(""),
            },
            BLOCK_SIZE,
        )
    }

    
    #[entry_point]
    pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
        pad_response_result(
            match msg {
                ExecuteMsg::ClaimRewards {  } =>  Ok(Response::new()),
                ExecuteMsg::ProxyUnstake { for_addr, amount } =>  Ok(Response::new()),
                ExecuteMsg::Unstake { amount, remove_liqudity } =>  Ok(Response::new()),
                ExecuteMsg::Receive { from, msg, amount } =>  Ok(Response::new()),
                ExecuteMsg::SetRewardToken { reward_token, daily_reward_amount, valid_to } =>  Ok(Response::new()),
                ExecuteMsg::SetAuthenticator { authenticator } =>  Ok(Response::new()),
                ExecuteMsg::SetAdmin { admin } =>  Ok(Response::new()),
                ExecuteMsg::RecoverFunds { token, amount, to, msg } =>  Ok(Response::new()),
            },
            BLOCK_SIZE,
        )
    }
}