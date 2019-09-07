//
//	web3 interface
//

use super::config;
use web3;
// use hyper::rt::Future;
use futures::{Future, Stream};

fn show_web3_accounts() {
	
	let ( _eloop, transport ) = web3::transports::Http::new( config::WEB3_LOCAL ).unwrap();
	let web3 = web3::Web3::new( transport );
	let accounts = web3.eth().accounts().wait().unwrap();
	println!( "Accounts: {:?}", accounts );

}


pub fn get_accounts() {

	println!("connecting to local ethereum node...");
	println!("read accounts:");
	show_web3_accounts();

}