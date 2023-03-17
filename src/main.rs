mod app;
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

fn main(){
    app::server_start();
}

#[actix_web::main]
 async fn test() -> std::io::Result<()> {
    println!("start SIWE app");

    // HttpServer::new(|| App::new().wrap(cors).service(hello))
    //     .bind("127.0.0.1:8080")?
    //     .run()
    //     .await?;

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:3000")
            .allowed_origin("https://127.0.0.1")
            .allowed_origin("127.0.0.1:3000")
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                "Access-Control-Allow-Headers",
                "Authorization",
                "authorization",
                "X-Requested-With",
                "Content-Type",
                "content-type",
                "Origin",
                "Client-id",
                "user-agent",
                "User-Agent",
                "Accept",
                "Referer",
                "referer",
                "Nonce",
                "signature",
                "Timestamp",
                "AppKey",
                "x-super-properties",
                "X-Super-Properties",
            ])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors).service(sign_verify)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}

#[get("/index.html")]
async fn index(req: HttpRequest) -> &'static str {
    "<p>Hello World!</p>"
}

#[post("/")]
async fn sign_verify(request: web::Json<Info>) -> impl Responder {
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


