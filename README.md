## Elven Tools SFT Minter Smart Contract

The SFT minter will be a complementary tool of the new NFT minter but will also be used as an independent tool in many other use cases. Because there is no actual ETA for the new NFT minter, the SFT minter will have basic functionality at first and will be developed in time. But please add your requirements. Use the GitHub issues for that.

## Current functionality

- issue collection
- configure and manage a collection
- limit per one address
- mint, create, buy and send SFTs (payment with EGLD)
- claim funds from smart contract

## For later

- [Kanban board](https://github.com/orgs/ElvenTools/projects/8/views/1)

## How to use it

The simplest way would be with [Elven Tools CLI](https://www.npmjs.com/package/elven-tools). You can find all the guidance here: [SFT Workflow jump start](https://www.elven.tools/docs/jump-start.html#sft-minter-tl%3Bdr). Check out short walkthrough video: [youtu.be/rMF3ItijHUA](https://youtu.be/rMF3ItijHUA).

You could also use [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/mxpy-cli) tool. 

Also check the interaction snippets included in the repository. Remember to set the path to your PEM file. You can do this in `devnet.snippets.sh` file.

In the latest version of VSCode MultiversX IDE extension, there is no more an option to run snippets, but you  can do this by hand in the terminal (in the project root run): 

**deploy**:
```
. interactions/devnet.snippets.sh && deploy
```

**issueToken**:
```
. interactions/devnet.snippets.sh && issueToken
```

**setLocalRoles**:
```
. interactions/devnet.snippets.sh && setLocalRoles
```

**createToken**:
```
. interactions/devnet.snippets.sh && createToken
```

**buy**:
```
. interactions/devnet.snippets.sh && buy
```

The code is open source, and there is an ABI file.

## Testing
You will find tests in tests/elven_tools_sft_minter_sc_rust_test.rs. To run a test, you can click on the Run Test button from under the test name in VS Code or you can run it with 

```
cargo test --test elven_tools_sft_minter_sc_rust_test
```

(Tests need some more love)

## Contact

- [Julian.io](https://www.julian.io)

## Other tools

- [Elven Tools](https://www.elven.tools)
- [useElven](https://www.useelven.com)
- [elven.js](https://www.elvenjs.com)
- [Buildo](https://github.com/xdevguild/buildo-begins)
