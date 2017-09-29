extern crate hyper;
extern crate web3;

use std::sync::Arc;
use hyper::header::{Authorization, ContentLength};
use hyper::server::{Http, Request, Response, Service};
use web3::contract::{Contract, Options};
use web3::futures::{self, Future};
use web3::types::{Address, U256};

#[derive(Clone)]
struct HelloBlockchain {
    web3: Arc<web3::Web3<web3::transports::Http>>,
}

impl HelloBlockchain {
    fn forbidden() -> Response {
        Response::new()
            .with_status(hyper::StatusCode::Forbidden)
            .with_body("Forbidden")
    }
}

impl Service for HelloBlockchain {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let contract_address: Address = "0x62d69f6867A0A084C6d313943dC22023Bc263691".parse().unwrap();
        let contract = Contract::from_json(self.web3.eth(), contract_address, include_bytes!("./abi.json")).unwrap();
        let header = req.headers().get::<Authorization<String>>();
        let address = match header.map(|h| h.parse::<Address>()) {
            Some(Ok(address)) => address,
            _ => return Box::new(futures::future::ok(Self::forbidden())),
        };

        Box::new(contract.query("locked", (address, ), None, Options::default(), None)
            .then(|locked: Result<U256, _>| {
                match locked {
                    Ok(v) if v != 0.into() => {
                        let body = format!("Balance: {:?}", locked);
                        Ok(Response::new()
                           .with_header(ContentLength(body.len() as u64))
                           .with_body(body))
                    }
                    _ => Ok(Self::forbidden())
                }
            }))
    }
}

fn main() {
    let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = Arc::new(web3::Web3::new(http));

    let addr = "127.0.0.1:3000".parse().unwrap();
    let handler = HelloBlockchain { web3 };
    let server = Http::new().bind(&addr, move || Ok(handler.clone())).unwrap();
    server.run().unwrap();
}
