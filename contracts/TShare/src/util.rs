use cosmwasm_std::{Storage, Response, Addr, Uint128, DepsMut, StdResult, WasmMsg, StdError,
        CosmosMsg, to_binary, QuerierWrapper};
use cw20::{Cw20ExecuteMsg};

use crate::error::ContractError;
use crate::state::{OPERATOR, BALANCES, TOTALSUPPLY};

pub fn balance_of(storage: &dyn Storage, sender: Addr) -> Uint128 {
    BALANCES.load(storage, sender.clone()).unwrap()
}

pub fn check_onlyoperator(storage: &dyn Storage, sender: Addr) -> Result<Response, ContractError> {
    let operator = OPERATOR.load(storage)?;
    if operator != sender {
        return Err(ContractError::Unauthorized{});
    }
    Ok(Response::new())
}

pub fn mint(
    deps: DepsMut,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let balance_before: Uint128 = balance_of(deps.storage, recipient.clone());
    let mut _deps = deps;
    perform_mint(_deps.branch(), recipient.clone(), amount).unwrap();
    let balance_after: Uint128 = balance_of(_deps.storage, recipient);
    let ret: bool = balance_after > balance_before;
    Ok(Response::new()
        .add_attribute("action", ret.to_string()))
}

fn perform_mint(
    deps: DepsMut,
    recipient: Addr,
    amount: Uint128
) -> Result<Response,ContractError> {
    if recipient != Addr::unchecked("".to_string()) {
        return Err(ContractError::ZeroAddrError{ })
    }
    let mut total_supply = TOTALSUPPLY.load(deps.storage)?;
    total_supply += amount;
    TOTALSUPPLY.save(deps.storage, &total_supply)?;
    let mut balance = BALANCES.load(deps.storage, recipient.clone())?;
    balance += amount;
    BALANCES.save(deps.storage, recipient.clone(), &balance)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    Ok(Response::new()
        .add_messages(msgs))
}

pub fn burn(deps: DepsMut, recipient: Addr, amount: Uint128) -> Result<Response,ContractError> {
    if recipient != Addr::unchecked("".to_string()) {
        return Err(ContractError::ZeroAddrError{ })
    }
    let mut balance = BALANCES.load(deps.storage, recipient.clone())?;
    if balance < amount {
        return Err(ContractError::BurnError{ })
    }
    balance -= amount;
    let mut total_supply = TOTALSUPPLY.load(deps.storage)?;
    total_supply += amount;
    TOTALSUPPLY.save(deps.storage, &total_supply)?;
    BALANCES.save(deps.storage, recipient.clone(), &balance)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    Ok(Response::new()
        .add_attribute("action","burn")
        .add_messages(msgs))
}
