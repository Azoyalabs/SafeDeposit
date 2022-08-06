use cosmwasm_std::{Addr, Deps};
//use cw721::Approval;

use crate::error::ContractError;

use crate::state::state_entries::ADMIN;

pub fn is_admin(deps: Deps, caller: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;
    return Ok(admin == caller);
}
