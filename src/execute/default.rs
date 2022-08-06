use std::str::FromStr;

use cosmwasm_std::{
    from_binary, to_binary, Binary, DepsMut, Env, MessageInfo, Response, Uint128, WasmMsg,
};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;
use crate::state::{state_reads, state_writes};

pub fn dispatch_default(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive {
            sender,
            amount,
            msg,
        } => try_receive(deps, info, sender, amount, msg),
        ExecuteMsg::DepositNative { beneficiary } => try_deposit_native(deps, info, beneficiary),
        ExecuteMsg::DepositCw20 {
            sender,
            beneficiary,
            token_address,
            amount,
        } => try_deposit_cw20(deps, env, sender, beneficiary, token_address, amount),
        ExecuteMsg::Lock {
            account,
            currency_identifier,
            amount,
        } => try_lock(deps, info, account, currency_identifier, amount),
        ExecuteMsg::Unlock {
            account,
            currency_identifier,
            amount,
        } => try_unlock(deps, info, account, currency_identifier, amount),
        ExecuteMsg::TransferLocked {
            account,
            currency_identifier,
            amount,
            beneficiary,
        } => try_transfer_lock(
            deps,
            info,
            account,
            currency_identifier,
            amount,
            beneficiary,
        ),
        _ => Err(ContractError::Never {}),
    }
}

fn try_transfer_lock(
    deps: DepsMut,
    info: MessageInfo,
    account: String,
    currency_identifier: String,
    amount: String,
    beneficiary: String,
) -> Result<Response, ContractError> {
    if !state_reads::is_authorized_handler(deps.as_ref(), info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    state_writes::transfer_locked_value(
        deps.storage,
        account,
        beneficiary,
        currency_identifier,
        Uint128::from_str(&amount)?.u128(),
    )?;

    return Ok(Response::new());
}

fn try_lock(
    deps: DepsMut,
    info: MessageInfo,
    account: String,
    currency_identifier: String,
    amount: String,
) -> Result<Response, ContractError> {
    if !state_reads::is_authorized_handler(deps.as_ref(), info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    state_writes::increase_locked_value(
        deps.storage,
        account,
        currency_identifier,
        Uint128::from_str(&amount)?.u128(),
    )
    .unwrap();

    return Ok(Response::new());
}

fn try_unlock(
    deps: DepsMut,
    info: MessageInfo,
    account: String,
    currency_identifier: String,
    amount: String,
) -> Result<Response, ContractError> {
    if !state_reads::is_authorized_handler(deps.as_ref(), info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    state_writes::decrease_locked_value(
        deps.storage,
        account,
        currency_identifier,
        Uint128::from_str(&amount)?.u128(),
    )
    .unwrap();

    return Ok(Response::new());
}

fn try_deposit_cw20(
    deps: DepsMut,
    env: Env,
    sender: String,
    beneficiary: String,
    token_address: String,
    amount: String,
) -> Result<Response, ContractError> {
    if state_reads::is_valid_currency(deps.as_ref(), token_address.clone())? {
        state_writes::update_deposit(
            deps.storage,
            beneficiary.clone(),
            token_address.clone(),
            Uint128::from_str(&amount).unwrap().u128(),
        )?;
    } else {
        return Err(ContractError::Cw20NotAccepted {
            token_address: token_address.clone(),
        });
    }

    let cw_msg = cw20::Cw20ExecuteMsg::TransferFrom {
        owner: sender,
        recipient: env.contract.address.into_string(),
        amount: Uint128::from_str(&amount)?,
    };
    let msg = WasmMsg::Execute {
        contract_addr: token_address,
        msg: to_binary(&cw_msg)?,
        funds: vec![],
    };

    return Ok(Response::new().add_message(msg));
}

fn try_receive(
    deps: DepsMut,
    info: MessageInfo,
    _sender: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    let beneficiary: String = from_binary(&msg)?;

    if state_reads::is_valid_currency(deps.as_ref(), info.sender.to_string())? {
        state_writes::update_deposit(
            deps.storage,
            beneficiary.clone(),
            info.sender.to_string(),
            amount.u128(),
        )?;
    } else {
        return Err(ContractError::Cw20NotAccepted {
            token_address: info.sender.to_string(),
        });
    }

    return Ok(Response::new());
}

fn try_deposit_native(
    deps: DepsMut,
    info: MessageInfo,
    beneficiary: String,
) -> Result<Response, ContractError> {
    // validate that beneficiary is a valid address
    match deps.api.addr_validate(beneficiary.as_str()) {
        Ok(_) => (),
        Err(_) => {
            return Err(ContractError::InvalidDepositBeneficiary {
                beneficiary: beneficiary,
            })
        }
    }

    for deposit in info.funds {
        if state_reads::is_valid_currency(deps.as_ref(), deposit.denom.clone())? {
            state_writes::update_deposit(
                deps.storage,
                beneficiary.clone(),
                deposit.denom,
                deposit.amount.u128(),
            )?;
        } else {
            return Err(ContractError::NativeCurrencyNotAccepted {
                denom: deposit.denom,
            });
        }
    }

    return Ok(Response::new());
}

fn _try_sample_execute(
    _deps: DepsMut,
    _info: MessageInfo,
    _spender: String,
    _expires: bool,
) -> Result<Response, ContractError> {
    return Ok(Response::new());
}
