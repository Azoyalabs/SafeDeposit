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

pub fn increase_locked_value(
    storage: &mut dyn Storage,
    account_holder: String,
    currency_identifier: String,
    amount_to_lock: u128,
) -> Result<(), ContractError> {
    BALANCES.update(
        storage,
        (account_holder.clone(), currency_identifier.clone()),
        |account| -> Result<_, ContractError> {
            let mut acc = match account {
                None => {
                    return Err(ContractError::AccountNotFound {
                        owner: account_holder,
                        currency_identifier: currency_identifier,
                    })
                }
                Some(val) => val,
            };

            if amount_to_lock > acc.available {
                return Err(ContractError::InsufficientFundsAvailableForLock {
                    currency_identifier: currency_identifier,
                    available: acc.available.to_string(),
                    required: amount_to_lock.to_string(),
                });
            }

            acc.available -= amount_to_lock;
            acc.locked += amount_to_lock;

            return Ok(acc);
        },
    )?;

    return Ok(());
}

pub fn decrease_locked_value(
    storage: &mut dyn Storage,
    account_holder: String,
    currency_identifier: String,
    amount_to_unlock: u128,
) -> Result<(), ContractError> {
    BALANCES.update(
        storage,
        (account_holder.clone(), currency_identifier.clone()),
        |account| -> Result<_, ContractError> {
            let mut acc = match account {
                None => {
                    return Err(ContractError::AccountNotFound {
                        owner: account_holder,
                        currency_identifier: currency_identifier,
                    })
                }
                Some(val) => val,
            };

            if amount_to_unlock > acc.locked {
                return Err(ContractError::InsufficientFundsLockedForUnlock {
                    currency_identifier: currency_identifier,
                    available: acc.locked.to_string(),
                    required: amount_to_unlock.to_string(),
                });
            }

            acc.locked -= amount_to_unlock;
            acc.available += amount_to_unlock;

            return Ok(acc);
        },
    )?;

    return Ok(());
}

pub fn transfer_locked_value(
    storage: &mut dyn Storage,
    account_holder: String,
    beneficiary: String,
    currency_identifier: String,
    amount_to_transfer: u128,
) -> Result<(), ContractError> {
    BALANCES.update(
        storage,
        (account_holder.clone(), currency_identifier.clone()),
        |account| -> Result<_, ContractError> {
            let mut acc = match account {
                None => {
                    return Err(ContractError::AccountNotFound {
                        owner: account_holder,
                        currency_identifier: currency_identifier.clone(),
                    })
                }
                Some(val) => val,
            };

            if amount_to_transfer > acc.locked {
                return Err(ContractError::InsufficientFundsLockedForTransfer {
                    currency_identifier: currency_identifier.clone(),
                    available: acc.locked.to_string(),
                    required: amount_to_transfer.to_string(),
                });
            }

            acc.locked -= amount_to_transfer;

            return Ok(acc);
        },
    )?;

    BALANCES.update(
        storage,
        (beneficiary.clone(), currency_identifier.clone()),
        |account| -> Result<_, ContractError> {
            let mut acc = match account {
                None => CurrencyAccount::new(),
                Some(val) => val,
            };

            acc.available += amount_to_transfer;

            return Ok(acc);
        },
    )?;

    return Ok(());
}
