use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;


pub fn dispatch_default(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        _ => Err(ContractError::Never {}),
    }
}

fn _try_sample_execute(
    _deps: DepsMut,
    _info: MessageInfo,
    _spender: String,
    _expires: bool,
) -> Result<Response, ContractError> {
    return Ok(Response::new());
}
