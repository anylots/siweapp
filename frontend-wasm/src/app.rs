use reqwest::blocking::Client;
use serde_json::{json, Value};

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

pub fn createSiweMsg(address: String) -> String {
    let mut msg = String::from("Domain");
    msg = msg + " wants you to sign in with your Ethereum account:";
    msg = msg + address;
}
