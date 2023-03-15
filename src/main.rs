use std::str::FromStr;

use actix_cors::Cors;
use actix_web::{get, http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use ethers;
use ethers::core::types::H160;
use ethers::prelude::k256;
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
use hex::FromHex;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use serde::Deserialize;
use sha3::{Digest, Keccak256};
use siwe::Message;

// #[async_std::main]
// async fn main() {

// }

#[derive(Deserialize)]
struct Info {
    message: String,
    sig: String,
}

// #[actix_web::main]
 fn main()  {
    println!("start SIWE app");

    getSignature();

    // HttpServer::new(|| App::new().wrap(cors).service(hello))
    //     .bind("127.0.0.1:8080")?
    //     .run()
    //     .await

    // HttpServer::new(|| {
    //     let cors = Cors::default()
    //         .allowed_origin("http://127.0.0.1:3000")
    //         .allowed_origin("https://127.0.0.1")
    //         .allowed_origin("127.0.0.1:3000")
    //         .allowed_origin("http://localhost:3000")
    //         .allowed_origin("http://localhost")
    //         .allowed_methods(vec!["GET", "POST", "OPTIONS"])
    //         .allowed_headers(vec![
    //             "Access-Control-Allow-Headers",
    //             "Authorization",
    //             "authorization",
    //             "X-Requested-With",
    //             "Content-Type",
    //             "content-type",
    //             "Origin",
    //             "Client-id",
    //             "user-agent",
    //             "User-Agent",
    //             "Accept",
    //             "Referer",
    //             "referer",
    //             "Nonce",
    //             "signature",
    //             "Timestamp",
    //             "AppKey",
    //             "x-super-properties",
    //             "X-Super-Properties",
    //         ])
    //         .allowed_header(http::header::CONTENT_TYPE)
    //         .max_age(3600);

    //     App::new().wrap(cors).service(hello)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
    
}

#[get("/index.html")]
async fn index(req: HttpRequest) -> &'static str {
    "<p>Hello World!</p>"
}

#[post("/")]
async fn hello(request: web::Json<Info>) -> impl Responder {
    println!("hello");

    println!("{}", request.sig);
    // let message = &request.message;
    // let signature = &request.sig;

    let result = verify_siwe(&request.message, &request.sig).await;
    assert!(result.is_ok());

    HttpResponse::Ok().body("Hello world!")
}
async fn verify_siwe(message: &String, signature: &String) -> Result<(), Box<dyn std::error::Error>> {
    let siwe_msg = Message::from_str(message.as_str()).unwrap();
    let sig = <[u8; 65]>::from_hex(signature.strip_prefix("0x").unwrap()).unwrap();

    println!("{}", "verify_eip191 start");

    let signer: Vec<u8> = siwe_msg.verify_eip191(&sig)?;
    
    let public_key = &k256::ecdsa::VerifyingKey::from_sec1_bytes(&signer).unwrap();
    let public_key = public_key.to_encoded_point(false);
    let process_sha3 = Keccak256::default()
        .chain_update(&public_key.as_bytes()[1..])
        .finalize();
    let result_slice = process_sha3.as_slice();
    let address_recovery = hex::encode(&result_slice[12..]);

    println!("address_recovery:{}", address_recovery);
    println!("{}", "verify_eip191 end");

    Ok(())
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
    let public_key = &k256::ecdsa::VerifyingKey::from_sec1_bytes(&signer).unwrap();
    let public_key = public_key.to_encoded_point(false);
    let process_sha3 = Keccak256::default()
        .chain_update(&public_key.as_bytes()[1..])
        .finalize();
    let result_slice = process_sha3.as_slice();
    let address_recovery = hex::encode(&result_slice[12..]);

    println!("address_recovery:{}", address_recovery);
    println!("{}", "verify_eip191 end");

    Ok(())
}

fn getSignature(){
    let data = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8,10";
    let msg = ethers::utils::hash_message(data);
    println!("{}", msg);

}
