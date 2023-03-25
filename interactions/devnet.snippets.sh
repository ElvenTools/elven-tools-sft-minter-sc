# Change the path to your PEM file
USER_PEM="../../../walletKey.pem"
PROXY="https://devnet-gateway.multiversx.com"
CHAIN_ID="D"

ASSETS_URIS_CONCAT=""
SC_ADDRESS=$(mxpy data load --key=address-devnet)
TOKEN_SAVED_SELLING_PRICE=$(mxpy data load --key=token-selling-price)
# Built in price for token issuance
ISSUE_COLLECTION_TOKEN_COST=50000000000000000

# Deploy SFT Minter Smart Contract
deploy() {
  mxpy --verbose contract deploy --chain=${CHAIN_ID} --project=${PROJECT} --pem=${USER_PEM} --gas-limit=20000000 --proxy=${PROXY} --recall-nonce --outfile="deploy-devnet.interaction.json" --send

  SC_ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")
  mxpy data store --key=address-devnet --value=${SC_ADDRESS}

  echo ""
  echo "Smart contract address: ${SC_ADDRESS}"
}

# Issue SFT collection token
issueToken() {
  read -p "COLLECTION_TOKEN_NAME: Enter collection token name (Alphanumeric characters only): " COLLECTION_TOKEN_NAME
  read -p "COLLECTION_TOKEN_TICKER: Enter collection token ticker name (Alphanumeric UPPERCASE only): " COLLECTION_TOKEN_TICKER
  mxpy --verbose contract call ${SC_ADDRESS} --function="issueToken" --chain=${CHAIN_ID} --pem=${USER_PEM} --gas-limit=60000000 --proxy=${PROXY} --recall-nonce --value=${ISSUE_COLLECTION_TOKEN_COST} --arguments str:${COLLECTION_TOKEN_NAME} str:${COLLECTION_TOKEN_TICKER} --send
}

# Set special roles for the token. Hardcoded in smart contract for now
setLocalRoles() {
  mxpy --verbose contract call ${SC_ADDRESS} --function="setLocalRoles" --chain=${CHAIN_ID} --pem=${USER_PEM} --gas-limit=60000000 --proxy=${PROXY} --recall-nonce --send
}

# Create SFT token
createToken() {
  read -p "TOKEN_DISPLAY_NAME: Enter token display name: " TOKEN_DISPLAY_NAME
  read -p "TOKEN_SELLING_PRICE: Enter token selling price (1 EGLD = 1000000000000000000): " TOKEN_SELLING_PRICE
  read -p "METADATA_IPFS_CID: Enter the the metadata file CID from IPFS: " METADATA_IPFS_CID
  read -p "METADATA_IPFS_FILE_NAME: Enter the the metadata file name uploaded using IPFS (ex: metadata.json): " METADATA_IPFS_FILE_NAME
  read -p "INITIAL_AMOUNT_OF_TOKENS: Enter the initial amount of tokens: " INITIAL_AMOUNT_OF_TOKENS
  read -p "MAX_PER_ADDRESS: Enter the maximum of tokens per address: " MAX_PER_ADDRESS
  read -p "ROYALTIES: Enter royalties percent (55,66% = 5566): " ROYALTIES
  read -p "TAGS: Enter descriptive tags (ex: tag1,tag2,tag3,tag4): " TAGS

  echo "ASSETS_URIS: Enter assets URIS. Whole URIs from IPFS. To your images, music, video files (separate with spaces):"
  read -a ASSETS_URIS

  for URI in ${ASSETS_URIS[@]}
  do
    ASSETS_URIS_CONCAT=${ASSETS_URIS_CONCAT:+$ASSETS_URIS_CONCAT }str:$URI
  done

  mxpy data store --key=token-selling-price --value=${TOKEN_SELLING_PRICE}

  mxpy --verbose contract call ${SC_ADDRESS} --function="createToken" --chain=${CHAIN_ID} --pem=${USER_PEM} --gas-limit=20000000 --proxy=${PROXY} --recall-nonce --arguments str:"${TOKEN_DISPLAY_NAME}" ${TOKEN_SELLING_PRICE} str:${METADATA_IPFS_CID} str:${METADATA_IPFS_FILE_NAME} ${INITIAL_AMOUNT_OF_TOKENS} ${MAX_PER_ADDRESS} ${ROYALTIES} str:"${TAGS}" ${ASSETS_URIS_CONCAT} --send
}

# Buy SFT tokens
buy() {
  read -p "AMOUNT_OF_TOKENS: Enter the amount of tokens to buy: " AMOUNT_OF_TOKENS
  read -p "SFT_TOKEN_NONCE: Enter the SFT token to buy nonce: " SFT_TOKEN_NONCE

  VALUE_TO_SEND=$((TOKEN_SAVED_SELLING_PRICE * AMOUNT_OF_TOKENS))

  mxpy --verbose contract call ${SC_ADDRESS} --function="buy" --chain=${CHAIN_ID} --pem=${USER_PEM} --gas-limit=20000000 --proxy=${PROXY} --value=${VALUE_TO_SEND} --recall-nonce --arguments ${AMOUNT_OF_TOKENS} ${SFT_TOKEN_NONCE} --send
}
