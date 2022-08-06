use cosmwasm_std::{Env, DepsMut, MessageInfo, Response};

use crate::{ContractError, state::state_entries::ADMIN};

use super::msg::InstantiateMsg;



pub fn execute_instantiation(deps: DepsMut, _env: Env, info: MessageInfo, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    ADMIN.save(deps.storage, &info.sender)?;


    return Ok(Response::default());
}