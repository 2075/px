//
//	import a package from npm and store in the cache dir
//

	extern crate curl;
	extern crate serde_json;

	use curl::easy::Easy;
	use serde_json::Value;
	use std::io::{stdout, Write};

	pub fn get_package( arg: &str) {

		// let pkg_name: &str = "lodash"
		// let exec: &str = "$( npm v " + pkg_name + " dist.tarball | tail -n 1 | cut -d \' -f 2 )"
		// let cache_dir: &str = "$( mktemp -d -p . ${pkg_name}__XXXXXXXXX )"
		
		// ## Unpacks to directory named after package@version

		// "curl $url | tar -xzf - --strip 1 --directory $tmp_dir"
		// "rm -rf $pkg_name"
		// "mv $tmp_dir $pkg_name"

	}

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
