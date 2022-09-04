#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, from_binary, Binary, Deps, DepsMut, 
                    Env, MessageInfo, Response, StdResult, Order};

use cw2::set_contract_version;
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ReceiveMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Cw20DepositResponse, Cw721DepositResponse, Cw20HookMsg, Cw721HookMsg};
use crate::state::{Cw20Deposits, Cw721Deposits, CW20_DEPOSITS, CW721_DEPOSITS};

// version info for migration info
//const CONTRACT_NAME: &str = "crates.io:standart-cw";
const CONTRACT_NAME: &str = "template-sc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive (cw20_msg) => receive_cw20(deps, _env, info, cw20_msg),
        ExecuteMsg::ReceiveNft (cw721_msg) => receive_cw721(deps, _env, info, cw721_msg),
        ExecuteMsg::WithdrawNft {contract, token_id} => execute_cw721_withdraw(deps, info, contract, token_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Cw20Deposits { address } => to_binary(&query_cw20_deposits(deps, address)?),
        QueryMsg::Cw721Deposits { address, contract } => to_binary(&query_cw721_deposits(deps, address, contract)?),
    }
}

pub fn receive_cw20(
    deps:DepsMut,
    _env: Env,
    info:MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Deposit {owner, amount }) => execute_cw20_deposit(deps, info, owner, amount),
        _ => Err(ContractError::CustomError { val: "Invalid Cw20HookMsg".to_string() }),
    }
}

pub fn receive_cw721(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    cw721_msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw721_msg.msg) {
        Ok(Cw721HookMsg::Deposit {owner, token_id }) => execute_cw721_deposit(deps, info, owner, token_id),
        _ => Err(ContractError::CustomError { val: "Invalid Cw721HookMsg".to_string() }),
    }
}

pub fn execute_cw20_deposit(deps: DepsMut, info: MessageInfo, owner:String, amount:u128) -> Result<Response, ContractError> {
    let sender = info.sender.clone().into_string();
    //check to see if u
    match CW20_DEPOSITS.load(deps.storage, (&owner, &sender)) {
        Ok(mut deposit) => {
            //add coins to their account
            deposit.amount = deposit.amount.checked_add(amount).unwrap();
            deposit.count = deposit.count.checked_add(1).unwrap();
            CW20_DEPOSITS
                .save(deps.storage, (&owner, &sender), &deposit)
                .unwrap();
        }
        Err(_) => {
            //user does not exist, add them.
            let deposit = Cw20Deposits {
                count: 1,
                owner: owner.clone(),
                contract:info.sender.into_string(),
                amount
            };
            CW20_DEPOSITS
                .save(deps.storage, (&owner, &sender), &deposit)
                .unwrap();
        }
    }
    Ok(Response::new()
        .add_attribute("execute", "cw20_deposit")
        .add_attribute("owner", owner)
        .add_attribute("contract", sender.to_string())
        .add_attribute("amount", amount.to_string()))
}

pub fn execute_cw721_deposit(deps: DepsMut, info: MessageInfo, owner:String, token_id:String) -> Result<Response, ContractError> {
    let cw721_contract = info.sender.clone().into_string();
    //check to see if u

    if CW721_DEPOSITS.has(deps.storage, (&cw721_contract, &owner, &token_id)) == true {
        return Err(ContractError::CustomError { val: "Already deposited".to_string() });
    }

    let deposit = Cw721Deposits {
        owner: owner.clone(),
        contract:info.sender.into_string(),
        token_id:token_id.clone()
    };
    CW721_DEPOSITS
        .save(deps.storage, (&cw721_contract, &owner, &token_id), &deposit)
        .unwrap();
    
    Ok(Response::new()
        .add_attribute("execute", "cw721_deposit")
        .add_attribute("owner", owner)
        .add_attribute("contract", cw721_contract.to_string())
        .add_attribute("token_id", token_id.to_string()))
}

pub fn execute_cw721_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    contract:String,
    token_id: String,
) -> Result<Response, ContractError> {
    let owner = info.sender.clone().into_string();
    if CW721_DEPOSITS.has(deps.storage, (&contract, &owner, &token_id)) == false {
        return Err(ContractError::NoCw721ToWithdraw {  });
    }

    CW721_DEPOSITS.remove(deps.storage, (&contract, &owner, &token_id));

    Ok(Response::new()
    .add_attribute("execute", "withdraw"))
}

pub fn query_cw20_deposits(deps: Deps, address:String) -> StdResult<Cw20DepositResponse> {
    let res: StdResult<Vec<_>> = CW20_DEPOSITS
        .prefix(&address)
        .range(deps.storage, None, None, Order::Descending)
        .collect();
    let deposits = res?;
    Ok(Cw20DepositResponse { 
        deposits
     })
}

fn query_cw721_deposits(deps: Deps, address: String, contract:String) -> StdResult<Cw721DepositResponse> {
    let res: StdResult<Vec<_>> = CW721_DEPOSITS
        .prefix((&contract, &address))
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let deposits = res?;
    Ok(Cw721DepositResponse { deposits })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const SENDER: &str = "sender_address";

    fn setup_contract(deps: DepsMut) {
        let msg = InstantiateMsg{};
        let info = mock_info(SENDER, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        println!("{:?}", res);
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn _instantiate() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

}
