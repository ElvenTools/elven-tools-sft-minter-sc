use elven_tools_sft_minter_sc::{setup::Setup, storage::Storage, operations::Operations, *};
use multiversx_sc::types::{Address, EsdtLocalRole};
use multiversx_sc_scenario::{managed_buffer, rust_biguint, testing_framework::*, DebugApi};

const WASM_PATH: &'static str = "output/elven_tools_sft_minter_sc.wasm";
const USER_BALANCE: u64 = 1_000_000_000_000_000_000;
const TOKEN_ID: &[u8] = "TSTN-000000".as_bytes();
const TEST_ENTRY: &[u8] = "test".as_bytes();

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> elven_tools_sft_minter_sc::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub user_address: Address,
    pub owner_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<elven_tools_sft_minter_sc::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> elven_tools_sft_minter_sc::ContractObj<DebugApi>,
{
    pub fn new(sc_builder: ContractObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_zero);
        let user_address = b_mock.create_user_account(&rust_biguint!(USER_BALANCE));
        let sc_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner_address), sc_builder, WASM_PATH);

        // simulate deploy
        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        b_mock.set_esdt_local_roles(
            sc_wrapper.address_ref(),
            TOKEN_ID,
            &[EsdtLocalRole::NftCreate,
            EsdtLocalRole::Burn,
            EsdtLocalRole::Mint,
            EsdtLocalRole::NftAddQuantity,
            EsdtLocalRole::NftBurn,][..],
        );

        ContractSetup {
            b_mock,
            user_address,
            owner_address,
            contract_wrapper: sc_wrapper,
        }
    }
}

#[test]
fn create_token_test() {
    let mut setup = ContractSetup::new(elven_tools_sft_minter_sc::contract_obj);
    let owner_address = setup.owner_address.clone();

    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.create_token();

                assert_eq!(
                    sc.create_token_testing().get(),
                    managed_buffer!(TEST_ENTRY)
                );
            },
        )
        .assert_ok();
}
