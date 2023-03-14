use reqwest::blocking::Client;
use serde_json::{json, Value};
use ethers;
use ethers::core::types::H160;
use ethers::prelude::k256;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
use ethers::core::types::Address;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://47.242.179.164:9933";
    let address = "0x17155EE3e09033955D272E902B52E0c10cB47A91";
    let data = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBalance\",\"params\":[\"{}\",\"latest\"],\"id\":1}}", address);
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(data)
        .send()?
        .text()?;
    let json: Value = serde_json::from_str(&response)?;
    let balance_hex = json["result"].as_str().unwrap_or_default();
    let balance_dec = u128::from_str_radix(balance_hex.trim_start_matches("0x"), 16)?;
    println!("ETH balance: {} wei", balance_dec);
    Ok(())
}

pub fn createSiweStr(address: String) -> String {
    let mut msg = String::from("Domain");
    msg += " wants you to sign in with your Ethereum account:\n";
    msg += address.as_str();
    msg += "\n";
    msg += "I accept the ServiceOrg Terms of Service: https://service.org/tos\n";

    msg += "URI: https://service.org/login\n";
    msg += "Version: 1\n";
    msg += "Chain ID: 1\n";
    msg += "Nonce: 32891756";
    msg += "Issued At: 2021-09-30T16:25:24Z\n";
    msg += "Resources:\n";
    msg += "- ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/\n";
    msg += "- https://example.com/my-web2-claim.json";

    return msg;
}

#[tokio::test]
pub async fn signIn(){
    let msg = createSiweStr("0x63F9725f107358c9115BC9d86c72dD5823E9B1E6".to_string());
    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        .parse::<LocalWallet>().unwrap();
    let signature: Signature = wallet.sign_message(msg.as_str()).await.unwrap();

    let verify_result = signature.verify(msg.as_str(), Address::from_str("0x63F9725f107358c9115BC9d86c72dD5823E9B1E6").unwrap());
    assert!(verify_result.is_ok() == true);

    println!("{}", "verify");

}
