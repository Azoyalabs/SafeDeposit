
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

// authorizations
pub const ADMIN: Item<Addr> = Item::new("admin");


pub const SAMPLE_REGISTRATION_STATUS: Map<Addr, bool> = Map::new("sample_registration_status");
