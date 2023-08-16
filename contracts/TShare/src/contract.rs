#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    Addr, to_binary, DepsMut, Env, MessageInfo, Response, QuerierWrapper,
    Uint128, CosmosMsg, WasmMsg, Storage
};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{OPERATOR, BALANCES, TOTALSUPPLY, STARTTIME, NAME, SYMBOL,
    ENDTIME, DEV_FUND_LAST_CLAIMED, COMMUNITY_FUND_REWARDRATE, DEV_FUND_REWARDRATE, 
    COMMUNITY_FUND, DEV_FUND, COMMUNITIY_FUND_LAST_CLAIMED, REWARD_POOL_DISTRIBUTED};

use crate::util::{balance_of, check_onlyoperator, mint, burn};
// version info for migration info
const CONTRACT_NAME: &str = "TShare";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const FARMING_POOL_REWARD_ALLOCATION: u128 = 60_000_000_000_000_000_000_000;
const COMMUNITY_FUND_POOL_ALLOCATION: u128 = 0;
const DEV_FUND_POOL_ALLOCATION: u128 = 5_000_000_000_000_000_000_000;

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
    let ether: Uint128 = Uint128::from((10 as u128).pow(18 as u32));
    let day: Uint128 = Uint128::from((60 * 60 * 24) as u128);
    let mut _deps = deps;
    mint(_deps.branch(), info.sender, Uint128::from(ether));
    STARTTIME.save(_deps.storage, &msg.starttime)?;
    let vesting_duration = Uint128::from(365 as u32) * day;
    let end_time = msg.starttime + vesting_duration;
    ENDTIME.save(_deps.storage, &end_time)?;
    COMMUNITIY_FUND_LAST_CLAIMED.save(_deps.storage, &msg.starttime)?;
    DEV_FUND_LAST_CLAIMED.save(_deps.storage, &msg.starttime)?;
    let community_fund_reward_rate: Uint128 = Uint128::from(COMMUNITY_FUND_POOL_ALLOCATION / vesting_duration.u128());
    COMMUNITY_FUND_REWARDRATE.save(_deps.storage, &community_fund_reward_rate);
    let dev_fund_reward_rate: Uint128 = Uint128::from(DEV_FUND_POOL_ALLOCATION / vesting_duration.u128());
    DEV_FUND_REWARDRATE.save(_deps.storage, &dev_fund_reward_rate)?;
    let dev_fund = _deps.api.addr_validate(msg.dev_fund.as_str())?;
    if dev_fund == Addr::unchecked("".to_string()) {
        return Err(ContractError::ZeroAddrError{ });
    }
    DEV_FUND.save(_deps.storage, &dev_fund)?;
    let community_fund = _deps.api.addr_validate(msg.community_fund.as_str())?;
    COMMUNITY_FUND.save(_deps.storage, &community_fund)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

fn try_set_treasury_fund(deps: DepsMut, env: Env, info: MessageInfo, community_fund: Addr) -> Result<Response, ContractError> {
    let dev_fund: Addr = DEV_FUND.load(deps.storage)?;
    if info.sender != dev_fund {
        return Err(ContractError::DevAddrError{ });
    }
    COMMUNITY_FUND.save(deps.storage, &community_fund)?;
    Ok(Response::new()
        .add_attribute("action", "set_treasury_fund"))
}

fn try_set_dev_fund(deps: DepsMut, env: Env, info: MessageInfo, dev_fund: Addr) -> Result<Response, ContractError> {
    let dev_fund: Addr = DEV_FUND.load(deps.storage)?;
    if info.sender != dev_fund {
        return Err(ContractError::DevAddrError{ });
    }
    if dev_fund == Addr::unchecked("".to_string()) {
        return Err(ContractError::ZeroAddrError{ });
    }
    DEV_FUND.save(deps.storage, &dev_fund)?;
    Ok(Response::new()
        .add_attribute("action", "set_dev_fund"))
}

fn unclaimed_treasury_fund(storage: &dyn Storage, env: Env)-> Uint128 {
    let mut now = Uint128::from(env.block.time.seconds());
    let end_time = ENDTIME.load(storage).unwrap();
    if now > end_time {
        now = end_time;
    }
    let cLastClaimed = COMMUNITIY_FUND_LAST_CLAIMED.load(storage).unwrap();
    let mut out = Uint128::zero();
    if cLastClaimed < now {
        let cRewardRate = COMMUNITY_FUND_REWARDRATE.load(storage).unwrap();
        let pending = (now - cLastClaimed) * cRewardRate;
        out = pending;
    }
    return out;
}

fn unclaimed_dev_fund(storage: &dyn Storage, env: Env) ->Uint128 {
    let mut now = Uint128::from(env.block.time.seconds());
    let end_time = ENDTIME.load(storage).unwrap();
    if now > end_time {
        now = end_time;
    }
    let dLastClaimed = DEV_FUND_LAST_CLAIMED.load(storage).unwrap();
    let mut out = Uint128::zero();
    if dLastClaimed < now {
        let dRewardRate = DEV_FUND_REWARDRATE.load(storage).unwrap();
        let pending = (now - dLastClaimed) * dRewardRate;
        out = pending;
    }
    return out;
}

fn try_claimrewards(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let mut pending: Uint128 = unclaimed_treasury_fund(deps.storage, env.clone());
    let community_fund = COMMUNITY_FUND.load(deps.storage)?;
    let mut _deps = deps;
    if pending > Uint128::zero() && community_fund != Addr::unchecked("".to_string()) {
        mint(_deps.branch(), community_fund, pending)?;
        let now = Uint128::from(env.block.time.seconds());
        COMMUNITIY_FUND_LAST_CLAIMED.save(_deps.storage, &now);
    }
    pending = unclaimed_dev_fund(_deps.storage, env.clone());
    let dev_fund = DEV_FUND.load(_deps.storage)?;
    if pending > Uint128::zero() && dev_fund != Addr::unchecked("".to_string()) {
        mint(_deps.branch(), dev_fund, pending)?;
        let now = Uint128::from(env.block.time.seconds());
        DEV_FUND_LAST_CLAIMED.save(_deps.storage, &now);
    }
    Ok(Response::new()
        .add_attribute("action", "claimrewards"))
}

fn try_distributereward(deps: DepsMut, env: Env, info: MessageInfo, farmingIncentiveFund: Addr) -> Result<Response, ContractError> {
    let dev_fund = DEV_FUND.load(deps.storage)?;
    check_onlyoperator(deps.storage, dev_fund)?;
    let mut reward_pool_distributed = REWARD_POOL_DISTRIBUTED.load(deps.storage)?;
    if reward_pool_distributed == true {
        return Err(ContractError::DistributeError{});
    }
    if farmingIncentiveFund == Addr::unchecked("".to_string()) {
        return Err(ContractError::FarmingIncentiveFundError{});
    }
    reward_pool_distributed = true;
    REWARD_POOL_DISTRIBUTED.save(deps.storage, &reward_pool_distributed);
    mint(deps, farmingIncentiveFund, Uint128::from(FARMING_POOL_REWARD_ALLOCATION))?;
    Ok(Response::new()
        .add_attribute("action", "distributereward"))
}

fn try_burn(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    burn(deps, info.sender, amount)?;
    Ok(Response::new()
        .add_attribute("action", "burn"))
}

fn try_governance_recover_unsupported(deps: DepsMut, env: Env, info: MessageInfo, token: Addr, amount: Uint128, to: Addr) -> Result<Response, ContractError> {
    let bank_cw20 = WasmMsg::Execute {
        contract_addr: String::from(token),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: to.to_string(),
            amount: amount,
        }).unwrap(),
        funds: Vec::new()
    };
    Ok(Response::new()
        .add_attribute("action", "governancerecoverunsupported"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Burn { amount } => try_burn(deps, env, info, amount),
        ExecuteMsg::setTreasuryFund { community_fund } => try_set_treasury_fund(deps, env, info, community_fund),
        ExecuteMsg::setDevFund { dev_fund } => try_set_dev_fund(deps, env, info, dev_fund),
        ExecuteMsg::claimRewards { } => try_claimrewards(deps, env, info),
        ExecuteMsg::distributeReward { farmingIncentiveFund } => try_distributereward(deps, env, info, farmingIncentiveFund),
        ExecuteMsg::governanceRecoverUnsupported { token, amount, to } => try_governance_recover_unsupported(deps, env, info, token, amount, to),
    }
}
