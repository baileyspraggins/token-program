# token-program

Simple version of an SPL Token Program

## Description

- Solana program created during the Chainlink Solana Bootcamp to show how to create an SPL Token Program.

## Features

- A Simple CLI client that shows the creation of a token, 100 coins minting to a token account, then that token account transfering 5 coins to another created token account.

## Instructions

    - CreateToken
        - Creates the master token.
    - CreateTokenAccount
        - Creates the token accounts that are under the master token.
    - Mint { amount: u64 }
        - Creates coins and sends them to the authority token account.
    - Transfer { amount: u64 }
        - Transfers 5 coins from the authority token account to the other token account

## Concepts Used

- Solana Programs and Accounts
- Serialization and Deserialization
- Solana Web3.JS RPC
- SPL Token Accounts

## How to interact with this code.

- By default this code is run on a local cluster. If you wish you change to the Solana devnet make sure your cli cluster is set as well as line 28 in utils.js is set to "https://api.devnet.solana.com"
- To set the name of the token you wish to create, please enter a string for the name parameter in line 60 in the token_program.ts file.

## Running the Program

- In the root of the project director run "cargo build-bpf" to build the project.
- Follow the prompt to deploy to your local cluster
- After your program is deployed enter the command npm run start and the program will execute

## Future Additions to project

- Implement a burn feature to the program that removes coins from the authority account in order to decrease the circulating supply.
