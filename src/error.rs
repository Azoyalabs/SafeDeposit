use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Never")]
    Never {},

    #[error("Invalid Deposit Beneficiary {beneficiary}")]
    InvalidDepositBeneficiary { beneficiary: String },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Storage Item does not exist: {identifier}")]
    StorageItemNotExist { identifier: String },

    #[error("Native Currency not Accepted {denom}")]
    NativeCurrencyNotAccepted { denom: String },

    #[error("Cw20 not Accepted {token_address}")]
    Cw20NotAccepted { token_address: String },

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    // payable
    #[error("Payable Contract or Method")]
    RequiresFunds {},

    #[error("Not Payable Contract or Method")]
    NotRequiresFunds {},

    #[error("Single Currency Accepted")]
    SingleCurrencyPayable {},

    #[error("Funds amount invalid")]
    InvalidFundsAmount {},
}
