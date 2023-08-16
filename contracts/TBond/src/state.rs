use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map, U128Key};

pub const OPERATOR: Item<Addr> = Item::new("operator");
pub const BALANCES: Map<Addr, Uint128> = Map::new("balances");

pub const NAME: Item<String> = Item::new("name");
pub const SYMBOL: Item<String> = Item::new("symbol");
pub const TOTALSUPPLY: Item<Uint128> = Item::new("totalsupply");
