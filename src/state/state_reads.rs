use cosmwasm_std::{Addr, Deps};
//use cw721::Approval;

use crate::error::ContractError;

use crate::state::state_entries::ADMIN;

use super::state_entries::VALID_CURRENCIES;

pub fn is_admin(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    return Ok(admin == caller);
}

pub fn is_valid_currency(deps: Deps, currency_identifier: String) -> Result<bool, ContractError> {
    let valid_currencies = VALID_CURRENCIES.load(deps.storage);

    match valid_currencies {
        Err(_) => {
            return Err(ContractError::StorageItemNotExist {
                identifier: currency_identifier,
            })
        }
        Ok(identifiers) => return Ok(identifiers.contains(&currency_identifier)),
    }
}
