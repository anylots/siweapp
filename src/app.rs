use std::future;

use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_http_server::ServerBuilder;
use common::Rpc;

struct RpcImpl;

// Server implementation
impl Rpc for RpcImpl {
    fn add(&self, a: u64, b: u64) -> Result<u64> {
        Ok(a + b)
    }

    fn call(&self, _: u64) -> BoxFuture<Result<String>> {
        Box::pin(future::ready(Ok("OK".to_owned())))
    }
}

fn main() {
	let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());
	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}