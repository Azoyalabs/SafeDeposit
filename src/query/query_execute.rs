use cosmwasm_std::{Deps, Env, StdResult};

use crate::{state::state_reads, structs::CurrencyAccount};

use super::query_response::CurrencyAccountResponse;

pub fn sample_query(_deps: Deps, _env: Env) -> StdResult<bool> {
    return Ok(true);
}

pub fn get_currency_account(
    deps: Deps,
    account_owner: String,
    currency_id: String,
) -> CurrencyAccountResponse {
    let account_data = state_reads::get_currency_account(deps, account_owner, currency_id).unwrap();

    return CurrencyAccountResponse {
        account: account_data,
    };
}
