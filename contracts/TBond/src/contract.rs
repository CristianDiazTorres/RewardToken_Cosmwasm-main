#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response, QuerierWrapper,
    Uint128, CosmosMsg, WasmMsg, Storage
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{OPERATOR, BALANCES, NAME, SYMBOL, TOTALSUPPLY};

use crate::util::{balance_of, check_onlyoperator};
// version info for migration info
const CONTRACT_NAME: &str = "TBond";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    OPERATOR.save(deps.storage, &info.sender);
    NAME.save(deps.storage, &String::from("3BOND"))?;
    SYMBOL.save(deps.storage, &String::from("3BOND"))?;
    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BurnFrom { account, amount } => try_burn_from(deps, env, info, account, amount),
        ExecuteMsg::Mint { recipient, amount } => try_mint(deps, info, recipient, amount),
    }
}

fn try_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    check_onlyoperator(deps.storage, info.sender);
    let balance_before: Uint128 = balance_of(deps.storage, recipient.clone());
    perform_mint(deps.storage, recipient.clone(), amount).unwrap();
    let balance_after: Uint128 = balance_of(deps.storage, recipient);
    let ret: bool = balance_after > balance_before;
    Ok(Response::new()
        .add_attribute("action", ret.to_string()))
}

fn perform_mint(
    store: &mut dyn Storage,
    recipient: Addr,
    amount: Uint128
) -> Result<Response,ContractError> {
    if recipient != Addr::unchecked("".to_string()) {
        return Err(ContractError::ZeroAddrError{ })
    }
    let mut total_supply = TOTALSUPPLY.load(store)?;
    total_supply += amount;
    TOTALSUPPLY.save(store, &total_supply)?;
    let mut balance = BALANCES.load(store, recipient.clone())?;
    balance += amount;
    BALANCES.save(store, recipient.clone(), &balance)?;
    let mut msgs: Vec<CosmosMsg> = vec![];
    Ok(Response::new()
        .add_messages(msgs))
}

fn try_burn(deps: DepsMut, _env: Env, info:MessageInfo, recipient: Addr, amount: Uint128) -> Result<Response,ContractError> {
    let mut sender = info.sender;
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

fn try_burn_from(deps: DepsMut, _env: Env, info: MessageInfo, recipient: Addr, amount: Uint128) -> Result<Response,ContractError> {
    let mut sender = info.sender;
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
        .add_attribute("action","burn_from")
        .add_messages(msgs))
}
