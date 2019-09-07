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

// extern crate state;
// extern crate curl;
// extern crate serde_json;
extern crate directories;
extern crate futures;
extern crate hyper;
extern crate ipfs_api;
extern crate web3;

use std::env;

mod px;

fn init () {

	px::config::check_first_run();

}

fn set_config () {

}

fn get_config () {

}

fn get_pkg( url: &str ) {

}

//
//
//

fn packages() {

}

fn accounts() {
	
}

fn version() {

	px::config::version();

	println!("

	");

}

fn config() {

	println!("list config");
	px::config::create_config_file();

}

fn help() {

	px::config::version();

	println!("
	usage:
	px <command> <params>
	{{ add | remove }} <package name> — add or remove a pkg to/from your registry
	{{ ls }} config | packages | accounts — list configs, packages or accounts
	");

}

fn error() {

	eprintln!("invalid command");

}

//
//
//

fn main() {

	px::config::version();

	init();

	let args: Vec<String> = env::args().collect();

	match args.len() {

		1 => {

		},
		2 => {

			let cmd = &args[1];

			match &cmd[..] {
				"help" => help(),
				"version" => version(),
				"ls" => println!("missing argument"),
				_ => error()

			}

		},
		3 => {

			let cmd = &args[1];
			let arg = &args[2];

			match &cmd[..] {
				"help" => help(),
				"ls" => {

					match &arg[..] {

						"config" => config(),
						"packages" => packages(),
						"accounts" => accounts(),
						_ => println!("missing argument")

					}

				},
				_ => error()

			}

		},
		_ => error()


	}

}
