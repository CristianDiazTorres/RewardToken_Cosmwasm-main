use super::*;
use cosmwasm_std::{from_binary, Uint128, Addr};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};

use crate::contract::{execute, instantiate};
use crate::query::{query};
use crate::msg::{QueryMsg, ExecuteMsg, InstantiateMsg};

use crate::mock_querier::mock_dependencies;

#[test]
fn workflow(){
    let mut deps = mock_dependencies(&[]);
    
    let msg = InstantiateMsg{
        starttime: Uint128::from(mock_env().block.time.seconds() + 1000),
        community_fund: "community_fund".to_string(),
        dev_fund: "dev_fund".to_string(),
    };
//instantiate
    let info = mock_info("admin", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

// 
    let msg = ExecuteMsg::Burn{
        amount: Uint128::zero()
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Burn{:?}", res);

// 
    let msg = ExecuteMsg::setTreasuryFund{
        community_fund: Addr::unchecked("community_fund".to_string())
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("SetCommunityFund{:?}", res);
// 
    let msg = ExecuteMsg::setDevFund{
        dev_fund: Addr::unchecked("dev_fund".to_string())
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("SetDevFund{:?}", res);
// 
    let msg = ExecuteMsg::claimRewards{ };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("ClaimRewards{:?}", res);
// 
    let msg = ExecuteMsg::distributeReward{ 
        farmingIncentiveFund: Addr::unchecked("distributeReward")
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("distributeReward{:?}", res);
// 
    let msg = ExecuteMsg::governanceRecoverUnsupported{ 
        token: Addr::unchecked("token"),
        amount: Uint128::from(1u128),
        to: Addr::unchecked("to"),
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("governanceRecoverUnsupported{:?}", res);
}

