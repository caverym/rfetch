use std::fs::File;
use std::io::{ErrorKind, Result};
use std::{env::var, io::Read};

use crate::error;

/// Ecos system 'Ecosystem' struct contains user's account and desktop
/// information.
/// All information is contained in `Option`, if information is unavailable
/// the item is `None`.
#[derive(Debug)]
pub struct Ecos {
    pub name: Option<String>,
    pub home: Option<String>,
    pub shell: Option<String>,
    pub desktop: Option<String>,
    pub session: Option<String>,
    pub cpu: Option<String>,
    pub board: Option<String>,
    pub mem: Option<String>,
    pub uptime: Option<String>,
}

impl Ecos {
    /// Collects all information, unavailable infromation is `None`
    pub fn new() -> Self {
        Self {
            name: Self::getuser(),
            home: Self::gethome(),
            shell: Self::getshell(),
            desktop: Self::getdesktop(),
            session: Self::getsession(),
            cpu: Self::getcpu(),
            board: Self::getproduct(),
            mem: Self::getmem(),
            uptime: Self::getuptime(),
        }
    }

    fn getuser() -> Option<String> {
        var("USER").ok()
    }

    fn gethome() -> Option<String> {
        var("HOME").ok()
    }

    fn getshell() -> Option<String> {
        let path = var("SHELL").ok()?;
        Some(path.split('/').last()?.to_string().to_title())
    }

    fn getdesktop() -> Option<String> {
        let mut desktop = var("DESKTOP_SESSION").ok()?;
        desktop.make_ascii_uppercase();
        Some(desktop)
    }

    fn getsession() -> Option<String> {
        Some(var("XDG_SESSION_TYPE").ok()?.to_title())
    }

    fn getcpu() -> Option<String> {
        read_cpu().ok()
    }

    fn getproduct() -> Option<String> {
        read_product().ok()
    }

    ///
    /// Also very inefficiet. Shouldn't read the file four times.
    fn getmem() -> Option<String> {
        let totalstr = read_memory("MemTotal").ok()?;
        let freestr = read_memory("MemFree").ok()?;
        let bufferstr = read_memory("Buffers").ok()?;
        let cachedstr = read_memory("Cached").ok()?;

        let total: u32 = totalstr.parse().ok()?;
        let free: u32 = freestr.parse().ok()?;
        let buffers: u32 = bufferstr.parse().ok()?;
        let cached: u32 = cachedstr.parse().ok()?;
        // let avail: u32 = availstr.parse().ok()?;

        Some(format!(
            "{}/{}",
            (total - free - buffers - cached) / 1024,
            (total) / 1024
        ))
    }

    fn getuptime() -> Option<String> {
        read_uptime().ok()
    }
}

fn read_cpu() -> Result<String> {
    let cpu = read_file("/proc/cpuinfo")?;

    let v: Vec<&str> = cpu.split('\n').collect();
    for l in v {
        if l.contains("model name") {
            return Ok(get_special(l, ':', 1));
        }
    }

    error!("failed to read CPU")
}

fn read_product() -> Result<String> {
    read_file("/sys/devices/virtual/dmi/id/product_family")
}

fn read_memory(p: &str) -> Result<String> {
    let meminfo = read_file("/proc/meminfo")?;

    let v: Vec<&str> = meminfo.split('\n').collect();
    for l in v {
        if l.contains(p) {
            let tmp = get_special(l, ':', 1);
            return Ok(get_special(&tmp, ' ', 0));
        }
    }

    error!("failed to read meminfo")
}

fn read_uptime() -> Result<String> {
    let mut uptimestr = read_file("/proc/uptime")?;
    uptimestr = uptimestr.split('.').collect::<Vec<&str>>()[0].to_string();

    if let Ok(secondes) = uptimestr.parse::<u32>() {
        let heures = secondes / 3600;
        let minutes = (secondes - (3600 * heures)) / 60;
        let sec = secondes - (3600 * heures) - (minutes * 60);

        let mut ut = format!("{}s", sec);

        if minutes > 1 || heures > 0 {
            ut = format!("{}m {}", minutes, ut);
        }

        if heures > 0 {
            ut = format!("{}h {}", heures, ut);
        }
        return Ok(ut);
    }

    error!("failed to parse uptime")
}

fn get_special(s: &str, split: char, v: usize) -> String {
    let n: Vec<&str> = s.split(split).collect();
    return n[v].trim().into();
}

fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let string = match String::from_utf8(buf) {
        Ok(s) => s.trim().into(),
        Err(e) => error!(e)?,
    };

    Ok(string)
}

/// This trait exists purely to change a String to have a capital first
/// letter using a method.
trait Title {
    fn to_title(&self) -> Self;
}

impl Title for String {
    fn to_title(&self) -> Self {
        let mut c = self.chars();

        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
