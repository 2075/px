// px poc

extern crate futures;
extern crate hyper;
extern crate ipfs_api;
extern crate web3;

// use web3::futures::Future;
// use hyper::rt::Future;
use futures::{Future, Stream};

use ipfs_api::{
	// response,
	IpfsClient
};
use std::io::Cursor;
use std::io::{self, Write};

// fn print_recursive(indent: usize, cmd: &response::CommandsResponse) {

// 	let cmd_indent = " ".repeat(indent * 4);
// 	let opt_indent = " ".repeat((indent + 1) * 4);

// 	println!("{}[{}]", cmd_indent, cmd.name);

// 	if cmd.options.len() > 0 {
// 		println!("{}* options:", cmd_indent);
// 		for options in cmd.options.iter() {
// 			println!("{}{}", opt_indent, &options.names[..].join(", "));
// 		}
// 	}

// 	if cmd.subcommands.len() > 0 {
// 		println!("{}- subcommands:", cmd_indent);
// 		for subcommand in cmd.subcommands.iter() {
// 			print_recursive(indent + 1, subcommand);
// 		}
// 	}

// }

fn read() {

	let client = IpfsClient::default();
	let req = client
	.get("/data/pkg.json")
	.concat2()
	.map(|res| {
		let out = io::stdout();
		let mut out = out.lock();
		out.write_all(&res).unwrap();
	})
	.map_err(|e| eprintln!("{}", e));

	hyper::rt::run(req);

}

fn write( data:String ) {

	println!( "{}", data );
	let data = Cursor::new( data );
	let client = IpfsClient::default();
	let req = client
		.add(data)
		.map(|res| {
				println!("{}", res.hash);
		})
		.map_err(|e| eprintln!("{}", e));

	hyper::rt::run(req);

}

fn show_web3_accounts() {
	let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();
	let web3 = web3::Web3::new(transport);
	let accounts = web3.eth().accounts().wait().unwrap();
	println!("Accounts: {:?}", accounts);	
}

fn main() {

	println!("Hello, world!");

	// display accounts in local web3 node
	println!("connecting to local ethereum node...");
	println!("read accounts:");
	show_web3_accounts();

	/** 
	 * 
	 */

	println!("connecting to local ipfs node...");
	// let client = IpfsClient::default();

	// // get commands from node
	// let req = client
	// .commands()
	// .map(|commands| print_recursive(0, &commands))
	// .map_err(|e| eprintln!("{}", e));
	// hyper::rt::run(req);

	// write data
	println!("write data:");
	write( "Hello, world!".to_string() );

	println!("read data:");
	read()



	// let bootstrap = client.bootstrap_list().map(|bootstrap| {
	//     println!("current bootstrap peers:");
	//     for peer in bootstrap.peers {
	//         println!("  {}", peer);
	//     }
	// });

	// let drop = client.bootstrap_rm_all().map(|drop| {
	//     println!("dropped:");
	//     for peer in drop.peers {
	//         println!("  {}", peer);
	//     }
	// });

	// let add = client.bootstrap_add_default().map(|add| {
	//     println!("added:");
	//     for peer in add.peers {
	//         println!("  {}", peer);
	//     }
	// });

	// hyper::rt::run(
	//     bootstrap
	//         .and_then(|_| {
	//             println!();
	//             println!("dropping all bootstrap peers...");

	//             drop
	//         })
	//         .and_then(|_| {
	//             println!();
	//             println!("adding default peers...");

	//             add
	//         })
	//         .map_err(|e| eprintln!("{}", e)),
	// );


}

