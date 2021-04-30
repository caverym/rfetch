use std::{env::var, io::Read};
use std::io::Result;
use std::fs::File;

use crate::errorhere;

macro_rules! handle {
	($f:expr) => {
		match $f {
			Ok(a) => Some(a),
			_ => None,
		}
	};
}

#[derive(Debug)]
pub struct Ecos {
	pub name: Option<String>,
	pub home: Option<String>,
	pub shell: Option<String>,
	pub desktop: Option<String>,
	pub session: Option<String>,
	pub distro: Option<String>,
}

impl Ecos {
	pub fn get() -> Result<Self> {

		let ecos = Self {
		    name: Self::getuser(),
		    home: Self::gethome(),
		    shell: Self::getshell(),
		    desktop: Self::getdesktop(),
		    session: Self::getsession(),
		    distro: Self::getdistro(),	
		};

		Ok(ecos)
	}

	fn getuser() -> Option<String> {
		handle!(var("USER"))
	}

	fn gethome() -> Option<String> {
		handle!(var("HOME"))
	}

	fn getshell() -> Option<String> {
		let path = handle!(var("SHELL"))?;
		let mut shell: String = path.split('/').last()?.into();
		shell.to_title();
		Some(shell)
	}

	fn getdesktop() -> Option<String> {
		let mut desktop: String = handle!(var("DESKTOP_SESSION"))?;
		desktop.to_title();
		Some(desktop)
	}

	fn getsession() -> Option<String> {
		let mut session: String = handle!(var("XDG_SESSION_TYPE"))?;
		session.to_title();
		Some(session)
	}

	fn getdistro() -> Option<String> {
		handle!(read_distro())
	}
}

fn read_distro() -> Result<String> {
	let mut file: File = File::open("/etc/lsb-release")?;

	let mut buf: Vec<u8> = Vec::new();
	file.read_to_end(&mut buf)?;

	let lsb: String = match String::from_utf8(buf) {
		Ok(s) => s,
		Err(e) => errorhere(&e.to_string())?,
	};


	let v: Vec<&str> = lsb.split('\n').collect();
	for l in v {
		if l.contains("DISTRIB_ID") {
			let n: Vec<&str> = l.split('=').collect();
			return Ok(n[1].into());
		}
	}


	errorhere("")
}

trait Title {
    fn to_title(&mut self);
}

impl Title for String {
    fn to_title(&mut self) {
        let mut c = self.chars();

        *self = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}