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
    };
//instantiate
    let info = mock_info("admin", &[]);
    let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
// 
    let msg = ExecuteMsg::Mint{
        recipient: Addr::unchecked("recipient".to_string()),
        amount: Uint128::from(10u128)
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Mint{:?}", res);

// 
    let msg = ExecuteMsg::BurnFrom{
        account: Addr::unchecked("account".to_string()),
        amount: Uint128::from(1u128)
    };

    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    println!("Burn{:?}", res);

}

