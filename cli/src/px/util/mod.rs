	use std::io;
	use std::io::Cursor;
	// use std::io::{self, Write};
	use std::io::prelude::*;
	use std::fs;
	use std::fs::{File, OpenOptions};
	use std::os::unix;
	use std::path::Path;
	use std::error::Error;
	use directories::{BaseDirs, UserDirs, ProjectDirs};

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

