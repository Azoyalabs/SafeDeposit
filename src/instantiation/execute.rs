use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    state::state_entries::{ADMIN, VALID_CURRENCIES},
    ContractError,
};

use super::msg::InstantiateMsg;

pub fn execute_instantiation(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    ADMIN.save(deps.storage, &info.sender)?;

    VALID_CURRENCIES.save(deps.storage, &vec![])?;

    return Ok(Response::default());
}
