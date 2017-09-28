extern crate bigint;
extern crate lettre;
extern crate web3;

use std::collections::HashMap;
use std::io::BufRead;
use std::{io, fs, env};

use bigint::uint::U256;
use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::transport::EmailTransport;
use lettre::email::EmailBuilder;
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
                    send_email(&address, &balance);
                }
            }
        }
    }
}


fn send_email(address: &Address, balance: &U256) {
    let email = EmailBuilder::new()
        .from("root@localhost")
        .to(("tomasz@parity.io", "Tomasz Drwięga"))
        .subject("ETH Balance Change")
        .text(&format!("Balance changed for {:?}: {}", address, balance))
        .build()
        .unwrap();
    let mut transport = SmtpTransportBuilder::localhost().unwrap().build();
    transport.send(email).expect("Should send successfuly.");
}

fn as_eth(balance: &U256) -> String {
    let mili_eth = *balance * 1000.into() / U256::from_dec_str("1000000000000000000").unwrap();
    format!("{:.3} Ether", mili_eth.low_u64() as f64 / 1000f64)
}
