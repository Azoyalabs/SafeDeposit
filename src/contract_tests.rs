#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use cosmwasm_std::{coin, Addr, Empty, Uint128};

    use cw20::Cw20Coin;
    use cw_multi_test::{App, BankSudo, Contract, ContractWrapper, Executor};

    use crate::contract::{execute, instantiate, query};
    use crate::execute_messages::msg::ExecuteMsg;
    use crate::execute_messages::msg_admin::AdminExecuteMsg;
    use crate::instantiation::msg::InstantiateMsg;

    const TEST_DENOM_NATIVE: &str = "test_native";
    const TEST_DENOM_CW20: &str = "test_cw20";
    const TEST_CREATOR: &str = "creator";
    const _TEST_USER: &str = "user";
    const _TEST_USER2: &str = "user2";

    const _TEST_PRICE: u64 = 10000000;

    const _TEST_INVALID_DENOM: &str = "notuusd";

    pub fn contract_vault() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new_with_empty(execute, instantiate, query); //.with_reply(reply);
        Box::new(contract)
    }

    pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new_with_empty(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        ); //.with_reply(reply);
        Box::new(contract)
    }

    pub fn setup_env() -> (App, Addr) {
        let owner = Addr::unchecked(TEST_CREATOR);

        let mut router = App::default(); // new(app_builder);
        router
            .sudo(cw_multi_test::SudoMsg::Bank(BankSudo::Mint {
                to_address: owner.clone().to_string(),
                amount: vec![coin(50000000, TEST_DENOM_NATIVE)],
            }))
            .unwrap();

        let vault_contract_id = router.store_code(contract_vault());

        //let _cw20_contract_id = router.store_code(contract_cw20());

        let msg = InstantiateMsg {};

        let mocked_contract_addr = router
            .instantiate_contract(
                vault_contract_id,
                owner.clone(),
                &msg,
                &[],
                "vault",
                Some(owner.into()),
            )
            .unwrap();

        return (router, mocked_contract_addr);
    }

    pub fn create_cw20(
        router: &mut App,
        name: &str,
        symbol: &str,
        beneficiary: &str,
        amount: u128,
    ) -> Addr {
        let owner = Addr::unchecked(TEST_CREATOR);

        let cw20_contract_id = router.store_code(contract_cw20());

        let msg = cw20_base::msg::InstantiateMsg {
            name: name.to_string(),
            symbol: symbol.to_string(),
            decimals: 6,
            initial_balances: vec![Cw20Coin {
                address: beneficiary.to_string(),
                amount: Uint128::from(amount),
            }],
            mint: None,
            marketing: None,
        };

        let mocked_contract_addr = router
            .instantiate_contract(
                cw20_contract_id,
                owner.clone(),
                &msg,
                &[],
                "token",
                Some(owner.into()),
            )
            .unwrap();

        return mocked_contract_addr;
    }

    #[test]
    fn instantiate_success() {
        let (_, _) = setup_env();
    }

    #[test]
    fn change_status() {
        let (mut app, contract_address) = setup_env();

        let owner = Addr::unchecked(TEST_CREATOR);

        let admin_msg = AdminExecuteMsg::SetAuthorizationStatus {
            target: TEST_CREATOR.into(),
            new_status: true,
        };
        let msg = ExecuteMsg::Admin(admin_msg);

        let _res = app
            .execute_contract(owner, contract_address, &msg, &[])
            .unwrap();
    }

    #[test]
    fn add_valid_currency() {
        let (mut app, contract_address) = setup_env();

        let owner = Addr::unchecked(TEST_CREATOR);

        let admin_msg = AdminExecuteMsg::AddValidCurrency {
            currency_id: TEST_DENOM_NATIVE.into(),
        };
        let msg = ExecuteMsg::Admin(admin_msg);

        let _res = app
            .execute_contract(owner, contract_address, &msg, &[])
            .unwrap();

        //let msg = AdminExecuteMsg::
    }

    #[test]
    fn deposit_native_currency() {
        let (mut app, contract_address) = setup_env();

        let owner = Addr::unchecked(TEST_CREATOR);

        let admin_msg = AdminExecuteMsg::AddValidCurrency {
            currency_id: TEST_DENOM_NATIVE.to_string(),
        };
        let msg = ExecuteMsg::Admin(admin_msg);

        let _res = app
            .execute_contract(owner.clone(), contract_address.clone(), &msg, &[])
            .unwrap();

        let msg = ExecuteMsg::DepositNative {
            beneficiary: owner.clone().into(),
        };
        let _res = app
            .execute_contract(
                owner.clone(),
                contract_address.clone(),
                &msg,
                &[coin(256000, TEST_DENOM_NATIVE.to_string())],
            )
            .unwrap();

        //let msg = AdminExecuteMsg::
    }

    #[test]
    fn withdraw_native_currency() {
        let (mut app, contract_address) = setup_env();

        let owner = Addr::unchecked(TEST_CREATOR);

        let admin_msg = AdminExecuteMsg::AddValidCurrency {
            currency_id: TEST_DENOM_NATIVE.to_string(),
        };
        let msg = ExecuteMsg::Admin(admin_msg);

        let _res = app
            .execute_contract(owner.clone(), contract_address.clone(), &msg, &[])
            .unwrap();

        let msg = ExecuteMsg::DepositNative {
            beneficiary: owner.clone().into(),
        };
        let _res = app
            .execute_contract(
                owner.clone(),
                contract_address.clone(),
                &msg,
                &[coin(256000, TEST_DENOM_NATIVE.to_string())],
            )
            .unwrap();

        let msg = ExecuteMsg::WithdrawNative {
            beneficiary: owner.clone().into_string(),
            denom: TEST_DENOM_NATIVE.to_string(),
            amount: "50000".into(),
        };
        let _res = app
            .execute_contract(owner.clone(), contract_address.clone(), &msg, &[])
            .unwrap();
        //let msg = AdminExecuteMsg::
    }

    #[test]
    fn deposit_cw20_currency_allowances() {
        let (mut app, contract_address) = setup_env();

        let cw_address = create_cw20(
            &mut app,
            TEST_DENOM_CW20,
            "cwtest",
            TEST_CREATOR.clone(),
            5000000,
        );

        let owner = Addr::unchecked(TEST_CREATOR);

        let admin_msg = AdminExecuteMsg::AddValidCurrency {
            currency_id: cw_address.to_string(),
        };
        let msg = ExecuteMsg::Admin(admin_msg);

        let _res = app
            .execute_contract(owner.clone(), contract_address.clone(), &msg, &[])
            .unwrap();

        // need allowances
        let msg = cw20_base::msg::ExecuteMsg::IncreaseAllowance {
            spender: contract_address.clone().into_string(),
            amount: Uint128::from_str("500000").unwrap(),
            expires: None,
        };
        let _res = app
            .execute_contract(owner.clone(), cw_address.clone(), &msg, &[])
            .unwrap();

        let msg = ExecuteMsg::DepositCw20 {
            sender: owner.clone().into(),
            beneficiary: owner.clone().into(),
            token_address: cw_address.into_string(),
            amount: "50000".to_string(),
        };
        let _res = app
            .execute_contract(owner.clone(), contract_address.clone(), &msg, &[])
            .unwrap();

        //let msg = AdminExecuteMsg::
    }
}
