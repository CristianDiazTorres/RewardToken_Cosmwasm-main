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
/*
fn spend_allowance(deps: DepsMut, owner: Addr, spender: Addr, amount: Uint128) -> Result<Response, ContractError> {
    let current_allowance = get_allowance(deps.storage, owner, spender);
    if current_allowance < amount {
        return Err(ContractError::AllowanceError { });
    }
    approve(deps.storage, owner, spender, amount);
    Ok(Response::new())
}

fn get_allowance(storage: &dyn Storage, owner: Addr, spender: Addr) -> Uint128 {
    let mut spender_map: Map<Addr, Uint128> = ALLOWANCES.load(storage, owner.clone()).unwrap();
    return spender_map.get(spender.clone());
}

fn approve(storage: &dyn Storage, owner: Addr, spender: Addr, amount: Uint128) -> Result<Response, ContractError> {
    if owner == Addr::unchecked("".to_string()) || spender == Addr::unchecked("".to_string()) {
        return Err(ContractError::ZeroAddrError{ })
    }
    let mut spender_map: Map<Addr, Uint128> = ALLOWANCES.load(storage, owner.clone())?;
    spender_map.set(spender, amount);
    ALLOWANCES.save(storage, owner, &spender_map)?;
    Ok(Response::new())
} */
