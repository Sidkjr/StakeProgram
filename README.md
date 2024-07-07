# Stake Program

In this program the user can Airdrop themselves tokens, Stake/Unstake them.

## Description

The Token Mint has a mint authority PDA which is used to Airdrop the tokens to the User's token account and the Staking pool has a Pool-Authority PDA which is used as authority in-case the user want's to unstake their tokens. 
## Getting Started

### Executing program

- To run this program, You will need to create an anchor project using `anchor init stake_program`. The code is w.r.t anchor version `0.29.0` and solana version `1.18.17` So make sure to configure your avm and solana-cli.

- In the cargo.toml file of the program(Not the root project) - add `anchor-spl = "0.29.0"` just below the anchor-lang dependency.

- Since i'm using a local-solana-validator(Localnet) Configure your local solana blockchain by `solana-test-validator --reset` then make sure to create a new keypair in another terminal using the command: `solana-keygen new -o keypair.json`.

- A new .json file will be created, you will need to then type in the terminal `solana config set --keypair <PATH_TO_KEYPAIR.JSON>` also set this exact path in the Anchor.toml file of the root project folder.

- Copy and Paste the Rust code and the test code from the tests folder. But while copying the code make sure to not copy the `declareid!()` line. Just keep it as it is. It represents your program, If you replaced it with the program id in my code, the program won't run.

- then run `anchor build`. If nothing goes wrong you can move ahead.

- the also run `npm install` to install all the dependencies listed in the package.json. We will be needing them.

- then run `anchor deploy`.

- Finally, run `anchor test`. If it says port already used try running this instead `anchor test --skip-local-validator` .Once a mint tx is created for the program, It is not possible to rerun the program again.


## Authors

Siddhant Khare  
[@Siddhant Khare](https://www.linkedin.com/in/siddhant-khare-13938920a/)

## Happy Coding
