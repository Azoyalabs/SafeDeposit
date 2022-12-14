use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    SetAuthorizationStatus { target: String, new_status: bool },
    AddValidCurrency { currency_id: String },
}
