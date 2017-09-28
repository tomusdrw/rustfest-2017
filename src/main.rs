extern crate web3;

use std::collections::HashMap;

use web3::futures::Future;
use web3::types::Address;

fn main() {
    let (_loop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let mut balances = HashMap::new();
    let address: Address = "0x8d12a197cb00d4747a1fe03395095ce2a5cc6819".parse().unwrap();

    loop {
        let balance = web3.eth().balance(address.clone(), None).wait().unwrap();
        if let Some(previous) = balances.insert(address.clone(), balance.clone()) {
            if previous != balance {
                println!("New balance for {:?}: {:?}", address, balance);
            }
        }
    }
}
