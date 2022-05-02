use async_std::fs::File;
use async_std::io::WriteExt;
use serde::{Deserialize, Serialize};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub alias: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from<P: AsRef<Path> + Copy>(path: P) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        // Probably shouldn't be fatal. Will just return an empty struct if reading from file is found.
        let config: Self = match serde_json::from_str(&contents) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Failed reading from provided file to configuration struct. Acting on default paramaters");
                Self {
                    ..Default::default()
                }
            }
        };

        config
    }

    pub async fn add_dependency<T: std::fmt::Display + Clone + Copy>(
        &mut self,
        pkg: T,
    ) -> Result<(), String> {
        let dep = match Dependency::from_str(&pkg)?
            .fetch_dependency_from_host()
        {
            Err(e) => return Err(e),
            Ok(v) => v,
        };

        // removes potential duplicates
        self.dependencies.clear();
        self.dependencies.push(dep);
        Ok(())
    }
}

impl Dependency {
    pub(super) fn from_str<T: std::fmt::Display + Clone + Copy>(pkg: T) -> Result<Self, String> {
        let path = format!("{pkg}");
        if path.contains("@") {
            let segments: Vec<&str> = path.split("@").collect();
            if segments.len() < 2 {
                return Err(
                    "Error: package paths are typically <pkg_name>@<pkg_version>".to_owned(),
                );
            }
            let dpn = Self {
                name: segments[0].to_owned(),
                version: segments[1].to_owned(),
                ..Default::default()
            };

            Ok(dpn)
        } else {
            return Err("package paths are typically <pkg_name>@<pkg_version>".to_owned());
        }
    }
    #[tokio::main]
    pub(super) async fn fetch_dependency_from_host(self) -> Result<Self, String> {
        let host = match std::env::var("INERT_HOST") {
            Ok(h) => h,
            Err(_) => String::from("https://github.com/tailwindlabs/tailwindcss/releases/download"),
        };
        let _body = match &self.to_string() {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        // println!("{0}/{1}/{2}", host, &self.name, &self.version)./;
        let target = format!("_build/{0}@{1}", &self.name, &self.version);
        let mut buf: Vec<u8> = Vec::new();
        let client = reqwest::Client::new();
        if let Ok(res) = client
            .get(format!("{0}/{1}/{2}", host, &self.name, &self.version))
            .send()
            .await
        {
            if let Ok(b) = res.bytes().await {
                buf = b[..].to_vec()
            }
        }
        // todo!("Need to write extension to _build, handle parsing extension commands, and create service to host extensions");
        let mut file = match File::create(&target).await {
            Ok(f) => f,
            Err(_) => return Err("unable to write {self.name}".to_owned()),
        };

        // returns final result
        match file.write_all(&buf).await {
            Ok(_) =>  {
                fs::set_permissions(&target, fs::Permissions::from_mode(0o755)).expect(&format!("unable to change file permissions. To automate your {0} operations, try running chmod +x _build/{0}", &self.name));
                return Ok(self);
            },
            Err(_) => return Err("unable to write {self.name}".to_owned()),
        }
    }

    pub fn to_string(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Err(_) => Err("unable to deserialize target string".to_owned()),
            Ok(s) => Ok(s),
        }
    }
}
