use jsonrpc_minihttp_server::jsonrpc_core::*;
use jsonrpc_minihttp_server::cors::AccessControlAllowOrigin;
use std::string::ToString;
use std::thread;
use failure::Error;
use jsonrpc_minihttp_server::ServerBuilder;
use jsonrpc_minihttp_server::DomainsValidation;
use web3::types::Address;

pub struct StateKeyRequest {
    pub sc_address: Address,
    pub worker_address: Address,
    pub signature: [u8; 65],
    pub encryption_pub_key:[u8; 65],
}
pub struct StakeKeyResponse {
    pub sc_address: Address,
    pub encrypted_state_key: Vec<u8>,
    pub signature: [u8; 65],
}
pub struct PrincipalHttpServer {
    pub port: String,
}

impl PrincipalHttpServer {
    pub fn new(port: &str) -> PrincipalHttpServer {
        PrincipalHttpServer { port: port.to_string() }
    }
    pub fn start(&self) {
        let mut io = IoHandler::default();
        io.add_method("say_hello", |_| {
            Ok(Value::String("hello".into()))
        });

        let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
            .start_http(&"127.0.0.1:3040".parse().unwrap())
            .expect("Unable to start RPC server");

        server.wait().unwrap();
        println!("JSON-RPC HTTP server listening on port: {}", self.port);
    }
}

//////////////////////// TESTS  /////////////////////////////////////////

#[cfg(test)]
mod test {
    #[test]
    pub fn request_state_key() {
        println!("requesting the state key from the Principal");
    }
}
