use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg_admin::AdminExecuteMsg;
use crate::state::{state_reads, state_writes};

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
        AdminExecuteMsg::SetAuthorizationStatus { target, new_status } => {
            try_set_authorization_status(deps, target, new_status)
        }
        AdminExecuteMsg::AddValidCurrency { currency_id } => {
            try_add_valid_currency(deps, currency_id)
        }
        //_ => return Ok(Response::new()),
        _ => Err(ContractError::Never {}),
    }
}

fn try_add_valid_currency(deps: DepsMut, currency_id: String) -> Result<Response, ContractError> {
    state_writes::add_valid_currency(deps.storage, currency_id)?;

    return Ok(Response::new());
}

fn try_set_authorization_status(
    deps: DepsMut,
    target: String,
    new_status: bool,
) -> Result<Response, ContractError> {
    state_writes::admin::set_authorization_status(
        deps.storage,
        deps.api.addr_validate(&target)?,
        new_status,
    )?;

    return Ok(Response::new());
}

fn _try_sample_execute(
    _deps: DepsMut,
    _address: String,
    _payload: bool,
) -> Result<Response, ContractError> {
    return Ok(Response::new());
}
