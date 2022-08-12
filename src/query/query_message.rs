use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance {
        account_owner: String,
        currency_id: String,
    },
    GetAllBalances {
        account_owner: String,
    },
}
