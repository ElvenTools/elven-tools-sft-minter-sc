# TODO: remove, just for development

USER_PEM="../../../walletKey.pem"
PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

SC_ADDRESS=$(mxpy data load --key=address-devnet)
ISSUE_COLLECTION_TOKEN_COST=50000000000000000

deploy() {
    mxpy --verbose contract deploy --chain=${CHAIN_ID} --project=${PROJECT} --pem=${USER_PEM} --gas-limit=120000000 --proxy=${PROXY} --recall-nonce --outfile="deploy-devnet.interaction.json" --send

    SC_ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")
    mxpy data store --key=address-devnet --value=${SC_ADDRESS}

    echo ""
    echo "Smart contract address: ${SC_ADDRESS}"
}

issueToken() {
    read -p "COLLECTION_TOKEN_NAME: Enter collection token name (Alphanumeric characters only): " COLLECTION_TOKEN_NAME
    read -p "COLLECTION_TOKEN_TICKER: Enter collection token ticker name (Alphanumeric UPPERCASE only): " COLLECTION_TOKEN_TICKER
    mxpy --verbose contract call ${SC_ADDRESS} --function="issueToken" --chain=${CHAIN_ID} --pem=${USER_PEM} --gas-limit=60000000 --proxy=${PROXY} --recall-nonce --value=${ISSUE_COLLECTION_TOKEN_COST} --arguments str:${COLLECTION_TOKEN_NAME} str:${COLLECTION_TOKEN_TICKER} --send
}

setLocalRoles() {
    mxpy --verbose contract call ${SC_ADDRESS} --function="setLocalRoles" --chain=${CHAIN_ID} --pem=${USER_PEM} --gas-limit=60000000 --proxy=${PROXY} --recall-nonce --send
}
