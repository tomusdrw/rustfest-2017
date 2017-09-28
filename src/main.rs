extern crate web3;
extern crate bigint;

use std::collections::HashMap;

use bigint::uint::U256;
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
            let previous: U256 = (*previous).into();
            let balance: U256 = (*balance).into();

            if previous != balance {
                println!("New balance for {:?}: {:?}", address, as_eth(&balance));
            }
        }
    }
}

fn as_eth(balance: &U256) -> String {
    let mili_eth = *balance * 1000.into() / U256::from_dec_str("1000000000000000000").unwrap();
    format!("{:.3} Ether", mili_eth.low_u64() as f64 / 1000f64)
}
