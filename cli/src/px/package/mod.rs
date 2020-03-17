//
//	import a package from npm and store in the cache dir
//

extern crate curl;
extern crate serde_json;

use curl::easy::Easy;
use std::io::{stdout, Write};
use std::process::Command;

pub fn ls() {
	println!("px pkg ls {{ own | local | global }}")
}

pub fn add(args: &Vec<String>) {

	println!("px pkg add <name|hash|uri> ")

	// let pkg_name: &str = "lodash"
	// let exec: &str = "$( npm v " + pkg_name + " dist.tarball | tail -n 1 | cut -d \' -f 2 )"
	// let cache_dir: &str = "$( mktemp -d -p . ${pkg_name}__XXXXXXXXX )"
	
	// ## Unpacks to directory named after package@version

	// "curl $url | tar -xzf - --strip 1 --directory $tmp_dir"
	// "rm -rf $pkg_name"
	// "mv $tmp_dir $pkg_name"

}

pub fn rm(args: &Vec<String>) {
	println!("px pkg ls {{ own | local | global }}")
}

pub fn create (args: &Vec<String>) {

	println!("create project {}", args[3] );


	let mut echo_hello = Command::new("sh");
	echo_hello.arg("-c").arg("echo hello");

	let mut npm = Command::new("npm")
		.arg("v")
		.arg("lodash")
		.arg("dist.tarball")
		// .arg("|")
		// .arg("tail")
		// .arg("-n")
		// .arg("1")
		// .arg("|")
		// .arg("cut")
		// .arg("-d")
		// .arg("\' -f 2 )")
	    .status();
	    //.expect("process failed to execute");

}

//
//
//

pub fn curl_from_url( url: &str) {

	let mut easy = Easy::new();

	easy.url( url ).unwrap();
	easy.write_function(|data| {
		stdout().write_all(data).unwrap();
		Ok(data.len())
	}).unwrap();
	easy.perform().unwrap();

	println!("{}", easy.response_code().unwrap());

}
