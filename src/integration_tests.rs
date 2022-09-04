/*#[cfg(test)]
mod tests {
    use crate::helpers::TemplateContract;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Cw20HookMsg, Cw20DepositResponse, Cw721HookMsg, Cw721DepositResponse};
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, to_binary};
    use cw20::{Cw20Contract, Cw20Coin, BalanceResponse};
    use cw20_base::msg::ExecuteMsg as Cw20ExecuteMsg;
    use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
    use cw20_base::msg::QueryMsg as Cw20QueryMsg;
    use cw721::OwnerOfResponse;
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    //use cw20_example::{self};
    //use nft::helpers::NftContract;
    //use nft::{self};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    /*pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw20_example::contract::execute,
            cw20_example::contract::instantiate,
            cw20_example::contract::query,
        );
        Box::new(contract)
    }*/

    /*pub fn contract_nft() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            nft::contract::entry::execute,
            nft::contract::entry::instantiate,
            nft::contract::entry::query,
        );
        Box::new(contract)
    }*/

    const USER: &str = "wallet_test";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

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

    fn store_code() -> (App, u64, u64, u64) {
        let mut app = mock_app();
        let template_id = app.store_code(contract_template());
        let cw20_id = app.store_code(contract_cw20());
        let cw721_id = app.store_code(contract_nft());
        (app, template_id, cw20_id, cw721_id)
    }

    fn template_instantiate(app: &mut App, template_id: u64) -> TemplateContract {
        let msg = InstantiateMsg {};
        let template_contract_address = app
            .instantiate_contract(
                template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "ptemplate",
                None,
            )
            .unwrap();
        TemplateContract(template_contract_address)
    }

    /*
    fn cw_20_instantiate(app: &mut App, cw20_id:u64) -> Cw20Contract {
        let coin = Cw20Coin {address:USER.to_string(), amount:Uint128::from(10000u64)};
        let msg:Cw20InstantiateMsg = Cw20InstantiateMsg {decimals:10, name:"Token".to_string(), symbol:"TKN".to_string(), initial_balances:vec![coin], marketing:None, mint:None };
        let cw20_contract_address = app
        .instantiate_contract(
            cw20_id,
            Addr::unchecked(ADMIN),
            &msg,
            &[],
            "cw20-example",
            None,
        )
        .unwrap();
    Cw20Contract(cw20_contract_address)
    }

    pub fn cw721_instantiate(app:&mut App, nft_id:u64, name:String, symbol:String, minter:String) -> NftContract {
        let contract = app
            .instantiate_contract(
                nft_id,
                Addr::unchecked(ADMIN),
                &nft::contract::InstantiateMsg { name, symbol, minter },
                &[],
                "nft",
                None,
            )
            .unwrap();
        NftContract(contract)
    }

    fn get_cw20_deposits(app: &App, template_contract: &TemplateContract) -> Cw20DepositResponse {
        app.wrap()
            .query_wasm_smart(template_contract.addr(), &QueryMsg::Cw20Deposits { address: USER.to_string() })
            .unwrap()
    }

    fn get_balance(app: &App, cw20_contract: &Cw20Contract, user:String) -> BalanceResponse {
        app.wrap()
            .query_wasm_smart(cw20_contract.addr(), &Cw20QueryMsg::Balance { address: user })
            .unwrap()
    }

    fn get_cw721_deposits(app: &App, template_contract: &TemplateContract, nft_contract:&NftContract) -> Cw721DepositResponse {
        app.wrap()
            .query_wasm_smart(template_contract.addr(), &QueryMsg::Cw721Deposits { address: USER.to_string(), contract: nft_contract.addr().to_string() })
            .unwrap()
    }

    fn get_owner_of(app: &App, nft_contract:&NftContract, token_id:String) -> OwnerOfResponse {
        app.wrap()
            .query_wasm_smart(nft_contract.addr(), &nft::contract::QueryMsg::OwnerOf { token_id, include_expired: None })
            .unwrap()
    }

    fn deposit_nft(app: &mut App, template_contract:&TemplateContract, cw721_contract:&NftContract, token_id:String) {
        let hook_msg = Cw721HookMsg::Deposit { owner: USER.to_string(), token_id: "0".to_string() };
        let msg = nft::contract::ExecuteMsg::SendNft { contract: template_contract.addr().to_string(), token_id: "0".to_string(), msg: to_binary(&hook_msg).unwrap() };
        let cosmos_msg = cw721_contract.call(msg).unwrap();
        app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
    }

    #[test]
    fn deposit_cw20() {
        let (mut app, deposit_id, cw20_id, _cw721_id) = store_code();
        let template_contract = template_instantiate(&mut app, deposit_id);
        let cw20_contract = cw_20_instantiate(&mut app, cw20_id);

        let balance = get_balance(&app, &cw20_contract, USER.to_string());
        println!("Intial Balance {:?}", balance);

        let hook_msg = Cw20HookMsg::Deposit { owner: USER.to_string(), amount: 500 };

        let msg = Cw20ExecuteMsg::Send { contract: template_contract.addr().to_string(), amount: Uint128::from(500u64), msg: to_binary(&hook_msg).unwrap() };
        let cosmos_msg = cw20_contract.call(msg).unwrap();
        app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

        let deposits = get_cw20_deposits(&app, &template_contract);
        println!("{:?}", deposits.deposits[0]);

        let balance = get_balance(&app, &cw20_contract, template_contract.addr().into_string());
        println!("Deposit Contract {:?}", balance);

        let balance = get_balance(&app, &cw20_contract, USER.to_string());
        println!("Post {:?}", balance);
    }
    */
}*/