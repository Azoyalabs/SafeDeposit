use cosmwasm_std::{Addr, Deps};
//use cw721::Approval;

use crate::{error::ContractError, structs::CurrencyAccount};

use crate::state::state_entries::ADMIN;

use super::state_entries::{AUTHORIZED_HANDLERS, BALANCES, VALID_CURRENCIES};

pub fn is_admin(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    return Ok(admin == caller);
}

pub fn is_authorized_handler(deps: Deps, target: Addr) -> Result<bool, ContractError> {
    let authorization = AUTHORIZED_HANDLERS.load(deps.storage, target);

    match authorization {
        Err(_) => return Ok(false),
        Ok(auth) => return Ok(auth),
    }
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

pub fn get_currency_account(
    deps: Deps,
    owner: String,
    currency_identifier: String,
) -> Result<CurrencyAccount, ContractError> {
    let account = BALANCES.load(deps.storage, (owner, currency_identifier));

    match account {
        Ok(acc) => return Ok(acc),
        Err(_) => return Ok(CurrencyAccount::new()),
    }
}

pub fn get_all_currency_accounts(
    deps: Deps,
    owner: String,
) -> Result<Vec<CurrencyAccount>, ContractError> {
    let all_currencies = VALID_CURRENCIES.load(deps.storage)?;

    let accounts = all_currencies
        .into_iter()
        .map(
            |currency_id| match BALANCES.load(deps.storage, (owner.clone(), currency_id)) {
                Ok(val) => val,
                Err(_) => CurrencyAccount::new(),
            },
        )
        .collect();

    return Ok(accounts);
}
