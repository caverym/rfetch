use std::fs::File;
use std::io::{Error, Read};

#[derive(Debug)]
pub struct OsRelease {
    pub name: Option<String>,
    pub pretty_name: Option<String>,
    pub id: Option<String>,
    pub build_id: Option<String>,
    pub ansi_colors: Option<Vec<String>>,
    pub home_url: Option<String>,
    pub doc_url: Option<String>,
    pub support_url: Option<String>,
    pub bug_rl: Option<String>,
}

impl OsRelease {
    pub fn open() -> Result<OsRelease, Error> {
        let mut osrelf: File = File::open("/etc/os-release")?;
        let mut buffer: String = String::new();
        osrelf.read_to_string(&mut buffer)?;
        let lines: Vec<String> = buffer
            .split('\n')
            .filter_map(|l| {
                let l = l.trim();

                if l == "" {
                    None
                } else {
                    Some(l.to_string())
                }
            })
            .collect();

        let mut osrels: OsRelease = OsRelease {
            ..Default::default()
        };

        for line in lines {
            let row: Vec<&str> = line.split('=').collect();
            if row.len() == 2 {
                match row[0] {
                    "NAME" => osrels.name = Some(row[1].trim_matches('"').to_string()),
                    "PRETTY_NAME" => {
                        osrels.pretty_name = Some(row[1].trim_matches('"').to_string())
                    }
                    "ID" => osrels.id = Some(row[1].to_string()),
                    "BUILD_ID" => osrels.build_id = Some(row[1].trim_matches('"').to_string()),
                    "ANSI_COLOR" => {
                        osrels.ansi_colors = row[1]
                            .trim_matches('"')
                            .split(';')
                            .map(|c| Some(c.to_string()))
                            .collect()
                    }
                    "HOME_URL" => osrels.home_url = Some(row[1].trim_matches('"').to_string()),
                    "DOCUMENTATION_URL" => {
                        osrels.doc_url = Some(row[1].trim_matches('"').to_string())
                    }
                    "SUPPORT_URL" => {
                        osrels.support_url = Some(row[1].trim_matches('"').to_string())
                    }
                    "BUG_REPORT_URL" => osrels.bug_rl = Some(row[1].trim_matches('"').to_string()),
                    _ => (),
                }
            }
        }

        Ok(osrels)
    }
}

impl Default for OsRelease {
    fn default() -> Self {
        Self {
            name: Default::default(),
            pretty_name: Default::default(),
            id: Default::default(),
            build_id: Default::default(),
            ansi_colors: Default::default(),
            home_url: Default::default(),
            doc_url: Default::default(),
            support_url: Default::default(),
            bug_rl: Default::default(),
        }
    }
}
