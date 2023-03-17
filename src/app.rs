use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::*;

#[rpc]
pub trait Rpc {
    /// Adds two numbers and returns a result
    #[rpc(name = "sign_in")]
    fn sign_in(&self, msg: String) -> Result<String>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
    fn sign_in(&self, msg: String) -> Result<String> {
        println!("{}", "Test1");
        Ok("Test1".to_string())
    }
}

pub fn server_start() {
    let mut io = jsonrpc_core::IoHandler::new();
    io.extend_with(RpcImpl.to_delegate());
    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
        ]))
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}
