use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map, U128Key};

pub const OPERATOR: Item<Addr> = Item::new("operator");
pub const BALANCES: Map<Addr, Uint128> = Map::new("balances");

pub const NAME: Item<String> = Item::new("name");
pub const SYMBOL: Item<String> = Item::new("symbol");
pub const TOTALSUPPLY: Item<Uint128> = Item::new("totalsupply");

pub const STARTTIME: Item<Uint128> = Item::new("starttime");
pub const ENDTIME: Item<Uint128> = Item::new("endtime");
pub const DEV_FUND_LAST_CLAIMED: Item<Uint128> = Item::new("dev_fund_last_claimed");
pub const COMMUNITY_FUND_REWARDRATE: Item<Uint128> = Item::new("community_fund_rewardrate");
pub const DEV_FUND_REWARDRATE: Item<Uint128> = Item::new("dev_fund_rewardrate");
pub const COMMUNITY_FUND: Item<Addr> = Item::new("community_fund");
pub const DEV_FUND: Item<Addr> = Item::new("dev_fund");
pub const COMMUNITIY_FUND_LAST_CLAIMED: Item<Uint128> = Item::new("community_fund_last_claimed");
pub const REWARD_POOL_DISTRIBUTED: Item<bool> = Item::new("reward_pool_distributed");
