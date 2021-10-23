#!/bin/bash
# initializes a new local solana wallet and fund it with 5 sol

mkdir ../solana-wallet
solana-keygen new --outfile ../solana-wallet/keypair.json
solana airdrop 5 $(solana-keygen pubkey ../solana-wallet/keypair.json)

