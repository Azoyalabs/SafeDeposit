use crate::error::ContractError;
use cosmwasm_std::{Addr, Storage};

use crate::state::state_entries::ADMIN;

pub fn update_admin(storage: &mut dyn Storage, new_admin: Addr) -> Result<(), ContractError> {
    ADMIN.save(storage, &new_admin).unwrap();

    return Ok(());
}
