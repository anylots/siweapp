use std::str::FromStr;

use ethers;
use ethers::core::types::H160;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
use sha3::{Digest, Keccak256};
use siwe::Message;

#[async_std::main]
async fn main() {
    println!("start SIWE app");

    let result = sign_verify().await;
    assert!(result.is_ok());
}

async fn sign_verify() -> Result<(), Box<dyn std::error::Error>> {
    // let message: Message = "aaadfsg".parse::<Message>()?;

    // instantiate the wallet
    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        .parse::<LocalWallet>()?;
    let address = wallet.address();
    let address_hex = hex::encode(H160::as_bytes(&address).to_vec());
    println!("{}", address_hex);

    //0x63F9725f107358c9115BC9d86c72dD5823E9B1E6
    let string_message = r#"service.org wants you to sign in with your Ethereum account:
0x63F9725f107358c9115BC9d86c72dD5823E9B1E6

I accept the ServiceOrg Terms of Service: https://service.org/tos

URI: https://service.org/login
Version: 1
Chain ID: 1
Nonce: 32891756
Issued At: 2021-09-30T16:25:24Z
Resources:
- ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/
- https://example.com/my-web2-claim.json"#;

    //sign a message
    let signature: Signature = wallet.sign_message(string_message).await?;
    let result = signature.recover(string_message);
    let address_hex_recover = hex::encode(H160::as_bytes(&result.unwrap()).to_vec());
    println!("{}", address_hex_recover);

    let verify_result = signature.verify(string_message, wallet.address()).unwrap();
    
    println!("{}", "verify");

    let siwe_msg = Message::from_str(string_message).unwrap();
    let sig = <[u8; 65]>::from(signature);

    println!("{}", "verify_eip191 start");

    let signer: Vec<u8> = siwe_msg.verify_eip191(&sig)?;
    let public_key = &signer[1..];
    let process_sha3 = Keccak256::default().chain_update(public_key).finalize();
    let result_slice = process_sha3.as_slice();
    let address_recovery = hex::encode(&result_slice[12..]);

    println!("{}", address_recovery);
    println!("{}", "verify_eip191 end");

    Ok(())
}
