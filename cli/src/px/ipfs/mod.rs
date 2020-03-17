//
//	ipfs interface
//
extern crate futures;
extern crate ipfs_api;

use std::io::{ Cursor, self, Write};
use ipfs_api::{response, IpfsClient};
use futures::TryStreamExt;

#[tokio::main]
async fn read() {

	let client = IpfsClient::default();

	match client
		.get( "/data/pkg.json" )
		.map_ok(|chunk| chunk.to_vec())
		.try_concat()
		.await
	{
		Ok(res) => {
			let out = io::stdout();
			let mut out = out.lock();
			out.write_all(&res).unwrap();
		}
		Err(e) => eprintln!( "error getting file: {}", e  )
	} 

}

#[tokio::main]
async fn write( data: String ) {

	let client = IpfsClient::default();
	let data = Cursor::new( data );

	match client.add(data).await {
		Ok( res ) => println!( "{}", res.hash ),
		Err( e ) => eprintln!( "error adding file: {}", e )
	}

}

#[cfg_attr(feature = "hyper", tokio::main)]
pub async fn ls() {

	println!("connecting to local ipfs node...");
	let client = IpfsClient::default();
    match client.commands().await {
        Ok(commands) => print_recursive(0, &commands),
        Err(e) => eprintln!("error getting commands: {}", e),
    }

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

fn print_recursive(indent: usize, cmd: &response::CommandsResponse) {

	let cmd_indent = " ".repeat(indent * 4);
	let opt_indent = " ".repeat((indent + 1) * 4);

	println!("{}[{}]", cmd_indent, cmd.name);

	if cmd.options.len() > 0 {
		println!("{}* options:", cmd_indent);
		for options in cmd.options.iter() {
			println!("{}{}", opt_indent, &options.names[..].join(", "));
		}
	}

	if cmd.subcommands.len() > 0 {
		println!("{}- subcommands:", cmd_indent);
		for subcommand in cmd.subcommands.iter() {
			print_recursive(indent + 1, subcommand);
		}
	}

}


