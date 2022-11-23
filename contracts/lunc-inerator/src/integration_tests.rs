#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const OWNER: &str = "OWNER";
    const DEVEL: &str = "DEVEL";

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "uluna";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {
            stable_denom: "uluna".to_string(),
            community_owner: OWNER.to_string(),
            community_dev: DEVEL.to_string(),
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod deposit {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn valid_deposit() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Deposit {};
            let funds = vec![Coin {
                denom: NATIVE_DENOM.to_string(),
                amount: Uint128::new(1),
            }];

            let cosmos_msg = cw_template_contract.call(msg, funds).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
