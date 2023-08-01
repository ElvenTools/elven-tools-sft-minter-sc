## Elven Tools SFT Minter Smart Contract

The SFT minter will be a complementary tool of the new NFT minter but will also be used as an independent tool in many other use cases. Because there is no actual ETA for the new NFT minter, the SFT minter will have basic functionality at first and will be developed in time. But please add your requirements. Use the GitHub issues for that.

## Current functionality

- issue collection
- configure and manage a collection
- limit per one address
- mint, create, buy and send SFTs (payment with EGLD)
- claim funds from smart contract
- change the price

## For later

- [Kanban board](https://github.com/orgs/ElvenTools/projects/8/views/1)

## How to use it

The simplest way would be with [Elven Tools CLI](https://www.elven.tools/docs/cli-sft-workflow.html). 

Setup steps with the CLI:
1. `npm install elven-tools -g`
2. `elven-tools derive-pem`
3. `elven-tools deploy sft-minter`
4. `elven-tools sft-minter issue-collection-token`
5. `elven-tools sft-minter set-roles`
7. `elven-tools sft-minter create`  
8. `elven-tools sft-minter start-selling`
---then---  
- Check all available interaction commands: [SFT minter commands](https://www.elven.tools/docs/cli-commands.html#sft-minter-commands)

(more to come, check the kanban todo [board](https://github.com/orgs/ElvenTools/projects/8))

You can also use it with `npx` without global installation.

You can find all the guidance here: [SFT Workflow jump start](https://www.elven.tools/docs/jump-start.html#sft-minter-tl%3Bdr). Check out short walkthrough video: [youtu.be/rMF3ItijHUA](https://youtu.be/rMF3ItijHUA).

You could also use [mxpy](https://docs.multiversx.com/sdk-and-tools/sdk-py/mxpy-cli) tool. 

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

- [Elven Tools](https://www.elven.tools) - SFTs and NFTs tools on the MultiversX blockchain - smart contracts, cli and dapp template
- [useElven](https://www.useelven.com) - React hooks to be used with Next.js or standalone React
- [elven.js](https://www.elvenjs.com) - The browser only lite MultiversX SDK, no build steps required. Works with static websites.
- [Buildo Begins](https://github.com/xdevguild/buildo-begins) - All things MultiversX CLI tool.
