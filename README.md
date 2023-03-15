# Sign In With Ethereum Using Rust

Verify the ownership of a account by signing a standard message;


## Motivation
Provide authentication services based on Ethereum at the frontend and backend

## Assembly
#### Frontend: rust for wasm; 
Sign the message and deliver the account, message and address to the back end;
#### Banckend: rust for siwe
Receive the request, use the signature to recover the address, and verify the account;