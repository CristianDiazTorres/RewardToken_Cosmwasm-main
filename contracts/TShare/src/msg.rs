use cosmwasm_std::{Uint128, Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub starttime: Uint128,
    pub community_fund: String,
    pub dev_fund: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Burn {
        amount: Uint128
    },
    setTreasuryFund { community_fund: Addr },
    setDevFund { dev_fund: Addr },
    claimRewards {},
    distributeReward { farmingIncentiveFund: Addr },
    governanceRecoverUnsupported { token: Addr, amount: Uint128, to: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner{ },
}
