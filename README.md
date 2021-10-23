# Seeded Contract

A smart contract built on the Solana blockchain

# Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool)
- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

# Development

## Step 1: Deploying the contract

1. `cd scripts && chmod +x *`
2. `./create-wallet.sh`
3. `./contract-build.sh`
4. `./contract-deploy.sh`

## Step 2: Running the test client

1. `cd client`
2. `yarn install`
3. `yarn build`
4. `yarn deploy`
5. `yarn start`
