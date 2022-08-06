use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::structs::CurrencyAccount;

// authorizations
pub const ADMIN: Item<Addr> = Item::new("admin");
pub const AUTHORIZED_HANDLERS: Map<Addr, bool> = Map::new("authorized_handlers");

pub const VALID_CURRENCIES: Item<Vec<String>> = Item::new("valid_currencies");

// balance using beneficiary identifier and identifier for currency
pub const BALANCES: Map<(String, String), CurrencyAccount> = Map::new("balances");
