extern crate glob;
extern crate config;
extern crate directories;

use config::*;
use std::path::Path;
use std::collections::HashMap;
use directories::BaseDirs;
use super::util;
use std::env;
use std::fs;

pub const CLI_NAME: &str = "px cli";
pub const CLI_SEMVER: &str = "0.1";
pub const WEB3_LOCAL: &str = "http://localhost:7545";

//
//
//

pub fn ls () {

	if let Some( base_dirs ) = BaseDirs::new() {

		// let home = Path::new(base_dirs.home_dir());
		let file = base_dirs.home_dir().join(".config/px/config.json");

		// println!("home {}", home.display() );
		println!("config file {}", file.display() );

		let mut settings = Config::default();

		settings.merge(File::from(file)).unwrap();

		// Print out our settings (as a HashMap)
		println!("\n{:?} \n\n-----------",
		settings.try_into::<HashMap<String, String>>().unwrap());

	}
}

pub fn get (_args: &Vec<String>) {
}

pub fn set (_args: &Vec<String>) {
}

//
//
//

pub fn get_env_key( key: &str) -> Result<String, env::VarError> {
	match env::var(key) {
		Ok(val) => return Ok(val),
		Err(e) => return Err(e)
	}
}

pub fn out_env_key( key: &str) {
	match env::var(key) {
		Ok(val) => println!("{}: {:?}", key, val),
		Err(e) => panic!("couldn't interpret {}: {}", key, e ),
	}
}

pub fn raw () {

	if let Some( base_dirs ) = BaseDirs::new() {

		let file = base_dirs.home_dir().join( ".config/px/config.json" );

		match util::cat( &file ) {
			Err(why) => println!("! {:?}", why.kind()),
			Ok(s) => println!("> {}", s),
		}

	}

}

//
//
//

pub fn version() {
	println!("{} {}", CLI_NAME, CLI_SEMVER );
}

pub fn first_run () {

	if let Some( base_dirs ) = BaseDirs::new() {

		let dir = base_dirs.home_dir().join( ".config/px" );
		let file = Path::new("default/config.json");

		if dir.exists() == false {

			println!( "first run..." );

			fs::create_dir_all(&dir).unwrap_or_else(|why| {
				println!("! {:?}", why.kind());
			});

			crate::px::util::touch( &Path::new(&dir.join(&file))).unwrap_or_else(|why| {
				println!("! {:?}", why.kind());
			});

			println!( "px is ready to fly" );

		}

	}

}