# Sign In With Ethereum Using Rust
Using siwe-rs to implement EIP-4361 in a appliaction.


## Motivation
* Users can use their Ethereum accounts to access web services instead of accounts owned by Web2. 

* Ethereum WASM support is very high potential.

## Assembly
#### Frontend: rust for wasm; 
Sign the message(EIP-4361 string representations) and submit account, message, and signature to the backend.
#### Banckend: rust for siwe
Receive the request, use the signature to recover the address, and verify the account.

## Running
*You may need to pre install the Rust / Wasm build tool:* *https://trunkrs.dev/#install*

`
run backend:
cargo run --release
`
`
run frontend:
trunk serve --release
`