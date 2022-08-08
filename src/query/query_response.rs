use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::CurrencyAccount;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SampleResponse {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CurrencyAccountResponse {
    pub account: CurrencyAccount,
}
