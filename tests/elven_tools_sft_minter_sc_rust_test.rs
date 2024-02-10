use elven_tools_sft_minter::{operations::Operations, setup::Setup, storage::Storage, *};
use multiversx_sc::codec::Empty;
use multiversx_sc::types::{Address, EsdtLocalRole, ManagedVec, MultiValueEncoded};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint,
    testing_framework::*, DebugApi,
};

const WASM_PATH: &'static str = "output/elven_tools_sft_minter.wasm";
const USER_BALANCE: u64 = 6_000_000_000_000_000_000;
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
            &[
                EsdtLocalRole::NftCreate,
                EsdtLocalRole::NftAddQuantity,
                EsdtLocalRole::NftBurn,
            ][..],
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
fn sft_minter_test() {
    let mut setup = ContractSetup::new(elven_tools_sft_minter::contract_obj);
    let owner_address = setup.owner_address.clone();
    let user_address = setup.user_address.clone();

    let giveaway_user_address_1 = setup.b_mock.create_user_account(&rust_biguint!(0));
    let giveaway_user_address_2 = setup.b_mock.create_user_account(&rust_biguint!(0));

    // Create token
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
                    uris_multi,
                );

                // Allow to buy
                sc.start_selling(01u64);

                assert_eq!(sc.paused(01u64).is_empty(), true);
                assert_eq!(
                    sc.token_tag(01u64).get().price,
                    managed_biguint!(100_000_000_000_000_000)
                );
                assert_eq!(
                    sc.token_tag(01u64).get().display_name,
                    managed_buffer!(TOKEN_DISPLAY_NAME)
                );
            },
        )
        .assert_ok();

    // Buy token
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
        .check_egld_balance(&owner_address, &rust_biguint!(200_000_000_000_000_000));

    // After buying the user should have 2 supply per address
    setup
        .b_mock
        .execute_query(&setup.contract_wrapper, |sc| {
            let query_result = sc
                .amount_per_address_total(01u64, &managed_address!(&user_address))
                .get();

            assert_eq!(query_result, managed_biguint!(2));
        })
        .assert_ok();

    // Claim SC funds
    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(200_000_000_000_000_000),
            |sc| {
                sc.claim_sc_funds();
            },
        )
        .assert_ok();

    setup
        .b_mock
        .check_egld_balance(&setup.contract_wrapper.address_ref(), &rust_biguint!(0));

    // Get price query
    setup
        .b_mock
        .execute_query(&setup.contract_wrapper, |sc| {
            let query_result = sc.get_price(01u64);

            assert_eq!(query_result, managed_biguint!(100_000_000_000_000_000));
        })
        .assert_ok();

    // Get token display name query
    setup
        .b_mock
        .execute_query(&setup.contract_wrapper, |sc| {
            let query_result = sc.get_token_display_name(01u64);

            assert_eq!(query_result, managed_buffer!(TOKEN_DISPLAY_NAME));
        })
        .assert_ok();

    // Get max amount to buy per address query
    setup
        .b_mock
        .execute_query(&setup.contract_wrapper, |sc| {
            let query_result = sc.get_max_amount_per_address(01u64);

            assert_eq!(query_result, managed_biguint!(10));
        })
        .assert_ok();

    // Change the price
    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_new_price(01u64, managed_biguint!(200_000_000_000_000_000));
            },
        )
        .assert_ok();

    setup
        .b_mock
        .execute_query(&setup.contract_wrapper, |sc| {
            let query_result = sc.get_price(01u64);

            assert_eq!(query_result, managed_biguint!(200_000_000_000_000_000));
        })
        .assert_ok();

    // Change the max supply per address limit
    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.set_new_amount_limit_per_address(01u64, managed_biguint!(20));
            },
        )
        .assert_ok();

    setup
        .b_mock
        .execute_query(&setup.contract_wrapper, |sc| {
            let query_result = sc.get_max_amount_per_address(01u64);

            assert_eq!(query_result, managed_biguint!(20));
        })
        .assert_ok();

    // Now I should be able to buy more than 10 supply with a new price of 2egld per token
    // I can buy max 18 supply at once even if the limit is 20, because I already have 2
    setup
        .b_mock
        .execute_tx(
            &user_address,
            &setup.contract_wrapper,
            &rust_biguint!(3_600_000_000_000_000_000),
            |sc| {
                sc.buy(18u32, 01u64);
            },
        )
        .assert_ok();

    // I should now have 99980 supply left
    setup.b_mock.check_nft_balance(
        &setup.contract_wrapper.address_ref(),
        TOKEN_ID,
        01u64,
        &rust_biguint!(99980),
        Option::<&Empty>::None,
    );

    // I want to decrease the initial supply now
    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.burn(01u64, &managed_biguint!(980));
            },
        )
        .assert_ok();

    // I should now have 99000 supply left
    setup.b_mock.check_nft_balance(
        &setup.contract_wrapper.address_ref(),
        TOKEN_ID,
        01u64,
        &rust_biguint!(99000),
        Option::<&Empty>::None,
    );

    // I want to increase the initial supply now
    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.mint(01u64, &managed_biguint!(1000));
            },
        )
        .assert_ok();

    // I should now have 100000 supply left
    setup.b_mock.check_nft_balance(
        &setup.contract_wrapper.address_ref(),
        TOKEN_ID,
        01u64,
        &rust_biguint!(100000),
        Option::<&Empty>::None,
    );

    // Giveaway 10000 to two addresses per 5000
    setup
        .b_mock
        .execute_tx(
            &owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let mut receivers = MultiValueEncoded::new();
                receivers.push((managed_address!(&giveaway_user_address_1), 01u64, managed_biguint!(5000)));
                receivers.push((managed_address!(&giveaway_user_address_2), 01u64, managed_biguint!(5000)));

                sc.giveaway(receivers);
            },
        )
        .assert_ok();

    // I should now have 90000 supply left because I gave away 10000 in two 5000 batches
    setup.b_mock.check_nft_balance(
        &setup.contract_wrapper.address_ref(),
        TOKEN_ID,
        01u64,
        &rust_biguint!(90000),
        Option::<&Empty>::None,
    );
}
