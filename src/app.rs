use ethers::core::types::Address;
use ethers::types::Signature;
use serde::{Deserialize, Serialize};
use siwe::Message;
use siwe::VerificationOpts;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
struct SignRequest {
    message: String,
    sig: Signature,
    address: String,
}

pub async fn process_sign_in(param: String) -> String {
    println!("process_sign_in");
    println!("{}", param);
    let sign_request: SignRequest = serde_json::from_str(param.as_str()).unwrap();
    let result = verify_siwe(sign_request.message, sign_request.sig, sign_request.address).await;
    return result;
}
async fn verify_siwe(message: String, signature: Signature, address: String) -> String {
    //step1. verify ecdsa
    let verify_result = signature.verify(
        message.clone(),
        Address::from_str(address.as_str()).unwrap(),
    );
    if verify_result.is_err() {
        // return "fail".to_string();
    }
    // return "success".to_string();

    //step2. verify message opts
    let siwe_msg: Message = Message::from_str(message.as_str()).unwrap();
    let sig: [u8; 65] = <[u8; 65]>::from(signature);
    //opts for verify domain, date, nonce
    let opts = VerificationOpts {
        ..Default::default()
    };
    if let Err(e) = siwe_msg.verify(&sig, &opts).await {
        println!("{}", e);
        // message cannot be correctly authenticated at this time
    }
    return "success".to_string();
}
