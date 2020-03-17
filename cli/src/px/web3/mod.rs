//
//	web3 interface
//

extern crate futures;
extern crate web3;

use super::config;
use web3::futures::Future;

pub fn eth_accounts() {

	println!( "Connecting Ethereum Node: {:?}", config::WEB3_LOCAL );	
	let ( _eloop, transport ) = web3::transports::Http::new( config::WEB3_LOCAL ).unwrap();
	let web3 = web3::Web3::new( transport );
	let accounts = web3.eth().accounts().wait().unwrap();
	println!( "Accounts: {:?}", accounts );

}


pub fn accounts() {

	println!("connecting to local ethereum node...");
	println!("read accounts:");
	eth_accounts();

}