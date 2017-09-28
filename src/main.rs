extern crate web3;
extern crate bigint;

use std::{io, fs, env};
use std::io::BufRead;
use std::collections::HashMap;

use bigint::uint::U256;
use web3::futures::Future;
use web3::types::Address;

fn addresses() -> Vec<Address> {
    let file = env::args().nth(1).expect("Please provide addresses file.");
    let file = io::BufReader::new(fs::File::open(file).unwrap());
    file.lines()
        .filter_map(|line| {
            let line = line.unwrap();
            match line.parse() {
                Ok(address) => Some(address),
                Err(err) => {
                    println!("Ignoring invalid address: {} ({:?})", line, err);
                    None
                }
            }
        }).collect()
}

fn main() {
    let (_loop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(transport);
    let mut balances = HashMap::new();

    let addresses = addresses();

    loop {
        for address in &addresses {
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
}

fn as_eth(balance: &U256) -> String {
    let mili_eth = *balance * 1000.into() / U256::from_dec_str("1000000000000000000").unwrap();
    format!("{:.3} Ether", mili_eth.low_u64() as f64 / 1000f64)
}
