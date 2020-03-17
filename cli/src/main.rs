#[allow(dead_code)]

// px cli
// proof of concept

// 
//
//
//	$ PX add <package_reference>
//
//	- search registry contract for package_reference
//
//	- pkg found
//		- add package reference url to package.json
//		- install package from reference url
//
//	- pkg not found
//		- pull npm package from npm repo into cache
//		- create package reference
//		- push npm package to ipfs node
//		- create package reference contract
//		- deploy reference contract to eth node
//
//	- stdout result
//
//
//

#[macro_use]
mod px;

use std::env;
use std::io;
use std::process;

fn init () {
	// px::config::version();
	px::config::first_run();
	// px::config::out_env_key("HOME");
}

fn error() {
	eprintln!("invalid command");
}

fn main() {
	use std::io::Write;
	let mut stdout = io::stdout();
	let mut stderr = io::stderr();	
	//let args: Vec<String> = env::args().collect();
	let args = env::args().collect::<Vec<_>>();

	init();

	match args.len() {

		1 => {},

		2 => {
			let cmd = &args[1];
			match &cmd[..] {
				"help" => px::help::help(),
				"version" => px::config::version(),
				_ => error()
			}
		},

		3 => {
			let cmd = &args[1];
			let arg = &args[2];
			match &cmd[..] {
				"pkg" => {
					match &arg[..] {
						"ls" => px::package::ls(),
						"add" => px::package::add(&args),
						"rm" => px::package::rm(&args),
						"create" => px::package::create(&args),
						_ => println!("missing argument")
					}
				},
				"web3" => {
					match &arg[..] {
						"accounts" => px::web3::accounts(),
						_ => println!("missing argument")
					}
				},				
				"ipfs" => {
					match &arg[..] {
						// "ls" => px::ipfs::ls(),
						_ => println!("missing argument")
					}
				},				
				"cfg" => {
					match &arg[..] {
						"ls" => px::config::ls(),
						"get" => px::config::get(&args),
						"set" => px::config::set(&args),
						_ => println!("missing argument")
					}
				},
				_ => error()
			}
		},

		_ => error()

	}

	let _ = stdout.flush();
	// process::exit();

}
