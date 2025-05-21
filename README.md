# pinocchio-counter

This is a demonstration of a Solana program and CLI for simple counter state on-chain.

## Setup

A hardcoded program id keypair exists at `credentials/counter.json`. This keypair is public
and should not be used for anything other than local testing of this program.

## Compiling

```
cd program 
cargo build-sbf
cd ..
```

```
cd cli
cargo build
cd ..
```

## Deploying Locally

```
cd /ledger
solana-test-validator --reset
```

```
cd -
solana program deploy target/sbf-solana-solana/release/pinocchio_counter_sbf.so --program-id credentials/counter.json
```

## Testing Against Local Validator

```
cd cli
cargo run init
cargo run increment
```

Check the account data with
```
solana address 5pa3rfwGTRyNLiWaM8bzk8jwYet6YaRLmRy2x4a8MPgq
```

## Notes

https://github.com/Nagaprasadvr/solana-pinocchio-starter

# Design Guide

Solarcade.xyz should showcase why Solana is great at what it does. Specifically, it's great at micro transactions - quick and cheap.

## User Story Chess
1. User goes to solarcade.xyz/chess
2. User sees chess board set up - with parameters to the right
    a. Game setup
    b. Ante amount
    c. Tip amount? Pre-selected with 0.001
    c. Start game
3. When user clicks start game it will pop up a signature
    a. The amount should only be the Ante Amount + (0.01) refundable account cost
4. Gives link to share
    a. Friend opens link
    b. Friend signs message 
    c. Plays
5a. Timeout - winner is set to the last person who played
5b. Forfeit - winner is set to the other player
5c. Winner - winner is set
5d. Draw - both parties have to agree

6 Either player can claim funds - this is permissionless 


### Internal Design
1. Server keeps game state
2. Server should 
