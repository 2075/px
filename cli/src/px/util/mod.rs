	use std::io;
	use std::io::{Read};
	use std::fs::{File, OpenOptions};
	use std::path::Path;

	pub fn cat(path: &Path) -> io::Result<String> {

		let mut f = File::open(path)?;
		let mut s = String::new();
		match f.read_to_string(&mut s) {
			Ok(_) => Ok(s),
			Err(e) => Err(e),
		}

	}

	pub fn touch(path: &Path) -> io::Result<()> {

		match OpenOptions::new().create(true).write(true).open(path) {
			Ok(_) => Ok(()),
			Err(e) => Err(e),
		}
		
	}

