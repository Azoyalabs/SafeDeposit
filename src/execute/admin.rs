use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg_admin::AdminExecuteMsg;
use crate::state::state_reads;

pub fn dispatch_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    if !state_reads::is_admin(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    match admin_msg {
        //_ => return Ok(Response::new()),
        _ => Err(ContractError::Never {}),
    }
}

fn _try_sample_execute(
    _deps: DepsMut,
    _address: String,
    _payload: bool,
) -> Result<Response, ContractError> {
    return Ok(Response::new());
}
