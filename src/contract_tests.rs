#[cfg(test)]
mod tests {
    use cosmwasm_std::{coin, Addr, Empty};

    use cw_multi_test::{App, BankSudo, Contract, ContractWrapper, Executor};

    use crate::contract::{execute, instantiate, query};
    use crate::execute_messages::msg::ExecuteMsg;
    use crate::execute_messages::msg_admin::AdminExecuteMsg;
    use crate::instantiation::msg::InstantiateMsg;

    const TEST_DENOM: &str = "uusd";
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
                amount: vec![coin(50000000, "buddycoin")],
            }))
            .unwrap();

        let vault_contract_id = router.store_code(contract_vault());

        let _cw20_contract_id = router.store_code(contract_cw20());

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
            currency_id: "buddycoin".into(),
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
            currency_id: "buddycoin".to_string(),
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
                &[coin(256000, "buddycoin".to_string())],
            )
            .unwrap();

        //let msg = AdminExecuteMsg::
    }
}
