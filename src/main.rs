extern crate web3;

use web3::futures::Future;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

fn main() {
    let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
    let web3 = web3::Web3::new(http);

    // The contract address.
    let address: Address = "0x00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap();
    // Deploying a contract
    let contract = Contract::from_json(web3.eth(), address, include_bytes!("./abi.json")).unwrap();

    // Query the contract instance
    let result = contract.query("locked", (address, ), None, Options::default(), None);
    let balance_of: U256 = result.wait().unwrap();
    assert_eq!(balance_of, 0.into());
}
