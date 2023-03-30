use ethers::core::types::Address;
use ethers::types::Signature;
use serde::{Deserialize, Serialize};
use siwe::Message;
use siwe::VerificationOpts;
use siweapp::limiter;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
struct SignRequest {
    message: String,
    sig: Signature,
    address: String,
}


// pub fn init(){
//     let mut bucket = token_bucket.lock().unwrap();
//     let result = bucket.acquire(1);
// }

/**
 * sign in service
 */
pub async fn process_sign_in(param: String) -> String {
    let limiter = limiter::new_limiter();
    let mut bucket = limiter.lock().unwrap();
    let result = bucket.acquire(1);
    if !result {
        return "Please try again later".to_string();
    }

    println!("process_sign_in");
    println!("{}", param);
    let sign_request: SignRequest = serde_json::from_str(param.as_str()).unwrap();
    if let Err(e) = verify_siwe(
        &sign_request.message,
        sign_request.sig,
        &sign_request.address,
    )
    .await
    {
        return e.to_string();
    }

    return "success".to_string();
}

/**
 * verify siwe message
 */
async fn verify_siwe(message: &str, signature: Signature, address: &str) -> Result<(), String> {
    //step1. verify ecdsa
    if let Err(e) = signature.verify(message.clone(), Address::from_str(address).unwrap()) {
        return Err(e.to_string());
    }

    //step2. verify message opts
    let siwe_msg: Message = Message::from_str(message).unwrap();
    //opts for verify domain, date, nonce
    let opts = VerificationOpts::default();
    if let Err(e) = siwe_msg.verify(&<[u8; 65]>::from(signature), &opts).await {
        return Err(e.to_string());
    }
    Ok(())
}
