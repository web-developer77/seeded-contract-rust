#!/bin/bash
# deploy the smart contract

solana program deploy ../target/deploy/seeded_contract.so --keypair ../solana-wallet/keypair.json

