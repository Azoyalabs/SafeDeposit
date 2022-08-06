use cosmwasm_std::{Deps, Env, StdResult};



pub fn sample_query(_deps: Deps, _env: Env) -> StdResult<bool> {
    return Ok(true);
}