//
//	config and settings
//

	use super::util;

	use directories::{BaseDirs};
	use std::path::Path;

	// use std::io;
	// use std::io::Cursor;
	// // use std::io::{self, Write};
	// use std::io::prelude::*;
	use std::fs;
	use std::fs::{File, OpenOptions};
	// use std::os::unix;
	// use std::error::Error;

	//

	pub const WEB3_LOCAL: &str = "http://localhost:8545";

	pub const INFURA_ID: &str = "ca17b747ef5d4ba08b951db1a4da4ca8";
	pub const INFURA_NODE: &str = "infura.io/v3/";
	pub const INFURA_NET: &str = "ropsten";

	pub const IPFS_LOCAL: &str = "http://localhost";
	pub const IPFS_GLOBAL: &str = "http://ipfs.one.io";

	pub const CLI_NAME: &str = "px-cli";
	pub const CLI_VERSION: &str = "0.0.1";
	pub const CLI_BUILD: i32 = 1;

	//

	pub fn version() {

		println!("{} {}", CLI_NAME, CLI_VERSION );

	}

	pub fn check_first_run () {

		if let Some( base_dirs ) = BaseDirs::new() {

			let home = Path::new( base_dirs.home_dir() );
			let dir = base_dirs.home_dir().join( ".config/px" );
			let file = Path::new("config.json");

			if dir.exists() == false {

				println!( "first run..." );

				fs::create_dir_all(&dir).unwrap_or_else(|why| {
					println!("! {:?}", why.kind());
				});

				crate::px::util::touch( &Path::new(&dir.join(&file))).unwrap_or_else(|why| {
					println!("! {:?}", why.kind());
				});

			}

		}

	}

	pub fn create_config_file () {

		if let Some( base_dirs ) = BaseDirs::new() {

			let home = Path::new( base_dirs.home_dir() );
			let file = base_dirs.home_dir().join( ".config/px/config.json" );

			match util::cat( &file ) {
				Err(why) => println!("! {:?}", why.kind()),
				Ok(s) => println!("> {}", s),
			}

		}

	}
