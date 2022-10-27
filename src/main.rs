use std::str::FromStr;
use std::string;

use async_std::io::ReadExt;
use ethers;
use ethers::abi::AbiDecode;
use ethers::core::types::H160;
use ethers::signers::{LocalWallet, Signer, Wallet};
use ethers::types::Signature;
use hex::{FromHex, ToHex};
use siwe::Message;

#[async_std::main]
async fn main() {
    println!("SIWE app");

    sign_verify().await;

    // siwe_msg();
    // siwe_test();
}



async fn sign_verify() -> Result<(), Box<dyn std::error::Error>> {
    // let message: Message = "aaadfsg".parse::<Message>()?;

    // instantiate the wallet
    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        .parse::<LocalWallet>()?;

    let address = wallet.address();
    let address_hex = hex::encode(H160::as_bytes(&address).to_vec());
    println!("{}", address_hex);

    let string_message = r#"localhost:4361 wants you to sign in with your Ethereum account:
    0x4f8828d8CE3906cFe31381eB2a8aC4ADe601C36F
    
    SIWE Notepad Example
    
    URI: http://localhost:4361
    Version: 1
    Chain ID: 1
    Nonce: kEWepMt9knR6lWJ6A
    Issued At: 2021-12-07T18:28:18.807Z"#;
    
    //63f9725f107358c9115bc9d86c72dd5823e9b1e6
                                           //0x63F9725f107358c9115BC9d86c72dD5823E9B1E6

    //sign a message
    let signature: Signature = wallet.sign_message(string_message).await?;

    let result = signature.recover(string_message);
    // println!("{}", result.unwrap());
    let address_hex1 = hex::encode(H160::as_bytes(&result.unwrap()).to_vec());
    println!("{}", address_hex1);

    let verify_result = signature.verify(string_message, wallet.address()).unwrap();

    println!("{}", "verify");

    let msg = Message::from_str(
        r#"localhost:4361 wants you to sign in with your Ethereum account:
0x4f8828d8CE3906cFe31381eB2a8aC4ADe601C36F

SIWE Notepad Example

URI: http://localhost:4361
Version: 1
Chain ID: 1
Nonce: kEWepMt9knR6lWJ6A
Issued At: 2021-12-07T18:28:18.807Z"#,
    )
    .unwrap();

    let sig = <[u8; 65]>::from(signature);
    println!("{}", "verify_eip191 start");


    // if let Err(e) = message.verify(&signature).await {
    //     // message cannot be correctly authenticated at this time
    // }
    
    let signer: Vec<u8> = msg.verify_eip191(&sig)?;
    println!("{}", "verify_eip191 end");

    println!("{}", hex::encode(signer));

    Ok(())
}

fn siwe_test() {
    let message = Message::from_str(
        r#"localhost:4361 wants you to sign in with your Ethereum account:
0x4f8828d8CE3906cFe31381eB2a8aC4ADe601C36F

SIWE Notepad Example

URI: http://localhost:4361
Version: 1
Chain ID: 1
Nonce: kEWepMt9knR6lWJ6A
Issued At: 2021-12-07T18:28:18.807Z"#,
    )
    .unwrap();

    println!("{}", "verify_eip191:");
    println!("{}", "verify_eip191:");
}

