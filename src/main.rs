#![warn(clippy::all)]

mod ecosys;
mod uname;
mod osrelease;

use ecosys::Ecos;
use std::env::{args, Args};
use std::io::{Error, ErrorKind, Result};
use uname::Uname;
use osrelease::OsRelease;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const PACKAGE: &str = env!("CARGO_PKG_NAME");

macro_rules! printo {
    ($fmt:expr, $o:expr) => {
        if let Some(v) = &$o {
            print!($fmt, v);
        }
    };
}

#[macro_export]
macro_rules! error {
    ($e:expr) => {
        crate::_errorhere(ErrorKind::Other, $e)
    };
}

fn _errorhere<R, T: ToString>(kind: ErrorKind, s: T) -> Result<R> {
    Err(Error::new(kind, s.to_string()))
}

fn main() -> Result<()> {
    let rfetch = Rfetch::create(Ecos::new(), Uname::new()?, OsRelease::open()?);

    if let Err(e) = rfetch.run(args()) {
        eprintln!("{}", e);
    }

    Ok(())
}

/// Main struct to contain the ecosys and uname structs to safely access both.
#[derive(Debug)]
struct Rfetch {
    user: Ecos,
    uname: Uname,
    osrel: OsRelease,
}

impl Rfetch {
    pub fn create(user: Ecos, uname: Uname, osrel: OsRelease) -> Rfetch {
        Self { user, uname, osrel } // logo: Vec::new() }
    }

    /// This is effectively the main function.
    /// This will parse the arguments and executes what the user requets.
    pub fn run(self, args: Args) -> Result<()> {
        // If there are no arguments, print default and exit.
        if args.len() == 1 {
            self.print_default();
        } else {
            // Iter through each argument, and the characters of every argument.
            args.into_iter()
                .skip(1)
                .try_for_each(|arg| self.parse_args(&arg))?;
        }

        Ok(())
    }

    // Iter through each argument happens here
    fn parse_args(&self, arg: &str) -> Result<()> {
        if !arg.contains('-') {
            error!("missing arguments")?;
        }

        match arg {
            "--help" | "-h" => Self::help(),
            "--version" | "-v" => Self::version(),
            #[cfg(debug_assertions)]
            "--debug" => {dbg!(self);}
            _ => arg.chars().try_for_each(|x| self.parse_chars(x))?,
        }

        Ok(())
    }

    // Iter through each char of every argument happens here
    fn parse_chars(&self, c: char) -> Result<()> {
        match c {
            'a' => self.print_arch(), // Each method name explains.
            'b' => self.print_board(),
            'c' => self.print_cpu(),
            'd' => self.print_desktop(),
            'D' => self.print_distro(),
            'H' => self.print_home(),
            'k' => self.print_kernel(),
            'm' => self.print_mem(),
            'n' => self.print_host(),
            'o' => self.print_os(),
            's' => self.print_shell(),
            'S' => self.print_session(),
            't' => self.print_time(),
            'u' => self.print_name(),

            '-' => (),
            _ => error!(format!("'{}' not a valid argument", c))?,
        }

        Ok(())
    }

    ///
    /// Isn't this super efficient.
    ///
    fn print_default(&self) {
        self.handle();
        self.print_distro();
        self.print_home();
        self.print_kernel();
        self.print_arch();
        self.print_shell();
        self.print_board();
        self.print_cpu();
        self.print_time();
        self.print_desktop();
        self.print_session();
        self.print_os();
        self.print_mem();
    }

    fn help() {
        println!("Usage: {} [FLAG]", PACKAGE);
        println!("{}", HELP);
        Self::version();
    }

    fn version() {
        println!("{} {} by {}", PACKAGE, VERSION, AUTHOR)
    }

    fn handle(&self) {
        let mut count = 1;

        let name = if let Some(n) = &self.user.name {
            n
        } else {
            self.print_host();
            return;
        };

        count += name.len();
        count += self.uname.nodename.len();
        printo!("\t\t{}@", self.user.name);
        print!("{}\n\t\t", self.uname.nodename);

        let vsep = vec!['-'; count];
        for s in vsep {
            print!("{}", s);
        }

        println!();
    }

    fn print_arch(&self) {
        println!("Arch:\t\t{}", self.uname.machine)
    }

    fn print_board(&self) {
        printo!("Host:\t\t{}\n", self.user.board)
    }

    fn print_cpu(&self) {
        printo!("CPU:\t\t{}\n", self.user.cpu)
    }

    fn print_desktop(&self) {
        printo!("Desktop:\t{}\n", self.user.desktop)
    }

    fn print_home(&self) {
        printo!("Home:\t\t{}\n", self.user.home)
    }

    fn print_kernel(&self) {
        println!("Kernel:\t\t{}", self.uname.release)
    }

    fn print_mem(&self) {
        printo!("Memory:\t\t{} MiB\n", self.user.mem)
    }

    fn print_host(&self) {
        println!("Hostname:\t{}", self.uname.nodename)
    }

    fn print_os(&self) {
        println!("OS:\t\t{}", self.uname.sysname)
    }

    fn print_shell(&self) {
        printo!("Shell:\t\t{}\n", self.user.shell)
    }

    fn print_session(&self) {
        printo!("Session:\t{}\n", self.user.session)
    }

    fn print_time(&self) {
        printo!("Uptime:\t\t{}\n", self.user.uptime)
    }

    fn print_name(&self) {
        printo!("User:\t\t{}\n", self.user.name)
    }

    fn print_distro(&self) {
        println!("Distro:\t\t{}", self.osrel.name)
    }
}

const HELP: &str = "
FLAGS:
\t-a\t\tView system architecture
\t-b\t\tView system board family
\t-c\t\tView system CPU
\t-d\t\tView desktop environment
\t-D\t\tView Linux Distribution
\t-h, --help\tView this help information
\t-H\t\tView current user home directory
\t-k\t\tView system kernel
\t-m\t\tView system memory
\t-n\t\tView system host name
\t-o\t\tView system OS
\t-s\t\tView user shell
\t-S\t\tView current graphics session
\t-t\t\tView system uptime
\t-u\t\tView user name
\t-v, --version\tView rfetch version
";
