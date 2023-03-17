# Sign In With Ethereum Using Rust
Verify the ownership of a account by signing a standard message.


## Motivation
* Enable users to use their Ethereum accounts to access web services instead of accounts owned by Web2. 

* Ethereum WASM support is very high potential.

## Assembly
#### Frontend: rust for wasm; 
Sign the message and deliver the account, message and address to the backend.
#### Banckend: rust for siwe
Receive the request, use the signature to recover the address, and verify the account.