use ethers;
use ethers::core::types::Address;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha3::{Digest, Keccak256};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
struct SignRequest {
    message: String,
    sig: Signature,
    address: String,
}

#[derive(PartialEq, Debug)]
pub enum SysError {
    FetchError(String),
    CommitError(String),
}

pub fn create_siwe_str(address: String) -> String {
    let mut msg = String::from("localhost:4361");
    msg += " wants you to sign in with your Ethereum account:\n";
    msg += address.as_str();
    msg += "\n";
    msg += "I accept the ServiceOrg Terms of Service: https://service.org/tos\n";
    msg += "\nURI: http://localhost:4361\n";
    msg += "Version: 1\n";
    msg += "Chain ID: 1\n";
    msg += "Nonce: 32891756\n";
    msg += "Issued At: 2021-09-30T16:25:24Z\n";
    msg += "Resources:\n";
    msg += "- https://example.com/my-web2-claim.json";

    return msg;
}

pub async fn sign_in(message: String, sig: Signature, address: String) -> String {
    let url = "http://127.0.0.1:3030/sign_in";
    let client = reqwest::Client::new();

    let request = SignRequest {
        message,
        sig,
        address,
    };
    let data = serde_json::to_string(&request).unwrap();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(data)
        .send()
        .await
        .unwrap();
    let json: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    return json.to_string();
}

/// Takes an eth address and returns it as a checksum formatted string.
pub fn eip55(addr_str: String) -> String {
    let hash = Keccak256::digest(addr_str.as_bytes());
    "0x".chars()
        .chain(addr_str.chars().enumerate().map(|(i, c)| {
            match (c, hash[i >> 1] & if i % 2 == 0 { 128 } else { 8 } != 0) {
                ('a'..='f' | 'A'..='F', true) => c.to_ascii_uppercase(),
                _ => c.to_ascii_lowercase(),
            }
        }))
        .collect()
}

/// fetch_balance
pub async fn fetch_balance(account: String) -> Result<String, SysError> {
    let provider =
        Provider::<Http>::try_from("https://eth-goerli.g.alchemy.com/v2/hbTrHkM").unwrap();

    let balance_from = provider
        .get_balance(Address::from_str(account.as_str()).unwrap(), None)
        .await;
    match balance_from {
        Ok(value) => Result::Ok(value.to_string()),
        Err(err) => Result::Err(SysError::FetchError(err.to_string())),
    }
}

// #[tokio::test]
pub async fn sign_in_test() {
    let msg = create_siwe_str("0x63F9725f107358c9115BC9d86c72dD5823E9B1E6".to_string());
    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        .parse::<LocalWallet>()
        .unwrap();
    let signature: Signature = wallet.sign_message(msg.as_str()).await.unwrap();

    let verify_result = signature.verify(
        msg.as_str(),
        Address::from_str("0x63F9725f107358c9115BC9d86c72dD5823E9B1E6").unwrap(),
    );
    assert!(verify_result.is_ok() == true);
    println!("{}", "verify");
}
