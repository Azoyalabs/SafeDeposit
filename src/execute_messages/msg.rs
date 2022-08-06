use cosmwasm_std::{Binary, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::execute_messages::msg_admin::AdminExecuteMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Admin(AdminExecuteMsg),

    // Receive cw20 hook
    Receive {
        sender: String,
        amount: Uint128,
        msg: Binary,
    },

    // User deposits. Caller can deposit for someone else
    DepositNative {
        beneficiary: String,
    },
    DepositCw20 {
        sender: String,
        beneficiary: String,
        token_address: String,
        amount: String,
    },

    // user withdraws. Can only withdraw from own account, but can transfer to anybody
    WithdrawNative {
        beneficiary: String,
        denom: String,
        amount: String,
    },
    WithdrawCw20 {
        beneficiary: String,
        token_address: String,
        amount: String,
    },

    // Services can lock / unlock deposits
    Lock {
        account: String,
        currency_identifier: String,
        amount: String,
    },
    Unlock {
        account: String,
        currency_identifier: String,
        amount: String,
    },
    // and transfer locked deposits
    TransferLocked {
        account: String,
        currency_identifier: String,
        amount: String,
        beneficiary: String,
    },
}
