use elven_tools_sft_minter::{setup::Setup, storage::Storage, operations::Operations, *};
use multiversx_sc::{types::{Address, EsdtLocalRole, ManagedVec, MultiValueEncoded}};
use multiversx_sc_scenario::{managed_buffer, managed_biguint, rust_biguint, managed_token_id, testing_framework::*, DebugApi};

const WASM_PATH: &'static str = "output/elven_tools_sft_minter.wasm";
const USER_BALANCE: u64 = 1_000_000_000_000_000_000;
const TOKEN_ID: &[u8] = "TSTN-000000".as_bytes();
const TOKEN_DISPLAY_NAME: &[u8] = "TestToken".as_bytes();
const URI: &[u8] = "https://ipfs.io/ipfs/12321eqewqeqw/1.png".as_bytes();
const URI2: &[u8] = "https://ipfs.io/ipfs/12321eqewqeqw/2.png".as_bytes();
const TAGS: &[u8] = "tag1,tag2,tag3".as_bytes();
const METADATA_CID: &[u8] = "32423432ewqewqeqwe32432423".as_bytes();
const METADATA_FILE: &[u8] = "metadata.json".as_bytes();

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> elven_tools_sft_minter::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub user_address: Address,
    pub owner_address: Address,
    pub contract_wrapper:
        ContractObjWrapper<elven_tools_sft_minter::ContractObj<DebugApi>, ContractObjBuilder>,
}

impl<ContractObjBuilder> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> elven_tools_sft_minter::ContractObj<DebugApi>,
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
                sc.collection_token_id().set(managed_token_id!(TOKEN_ID));
            })
            .assert_ok();

        b_mock.set_esdt_local_roles(
            sc_wrapper.address_ref(),
            TOKEN_ID,
            &[EsdtLocalRole::NftCreate,
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
fn create_and_buy_token_test() {
    let mut setup = ContractSetup::new(elven_tools_sft_minter::contract_obj);
    let owner_address = setup.owner_address.clone();
    let user_address = setup.user_address.clone();

    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut uris_vec = ManagedVec::new();
                uris_vec.push(managed_buffer!(URI));
                uris_vec.push(managed_buffer!(URI2));

                let uris_multi = MultiValueEncoded::from(uris_vec);

                sc.create_token(
                  managed_buffer!(TOKEN_DISPLAY_NAME),
                  managed_biguint!(100_000_000_000_000_000),
                  managed_buffer!(METADATA_CID),
                  managed_buffer!(METADATA_FILE),
                  managed_biguint!(100000),
                  managed_biguint!(10),
                  managed_biguint!(100),
                  managed_buffer!(TAGS),
                  uris_multi
                );

                assert_eq!(
                    sc.token_price_tag(01u64).get().price,
                    managed_biguint!(100_000_000_000_000_000)
                );
                assert_eq!(
                  sc.token_price_tag(01u64).get().display_name,
                  managed_buffer!(TOKEN_DISPLAY_NAME)
                );
            },
        )
        .assert_ok();

    setup
        .b_mock
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(200_000_000_000_000_000),
            |sc| {
              sc.buy(2u32, 01u64);
            },
        )
        .assert_ok();

    setup
        .b_mock
        .check_egld_balance(&owner_address, &rust_biguint!(200_000_000_000_000_000))
}
