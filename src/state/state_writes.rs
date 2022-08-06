use crate::{error::ContractError, structs::CurrencyAccount};
use cosmwasm_std::{Addr, Storage};

use crate::state::state_entries::ADMIN;

use super::state_entries::BALANCES;

pub fn update_admin(storage: &mut dyn Storage, new_admin: Addr) -> Result<(), ContractError> {
    ADMIN.save(storage, &new_admin).unwrap();

    return Ok(());
}

pub fn update_deposit(
    storage: &mut dyn Storage,
    beneficiary: String,
    currency_identifier: String,
    amount: u128,
) -> Result<(), ContractError> {
    BALANCES.update(
        storage,
        (beneficiary, currency_identifier),
        |balance| -> Result<_, ContractError> {
            let mut balance = match balance {
                Some(b) => b,
                None => CurrencyAccount::new(),
            };

            balance.available += amount;

            return Ok(balance);
        },
    )?;

    return Ok(());
}
