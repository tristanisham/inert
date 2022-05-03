use crate::builder;
use crate::serve;
use async_std::{fs, fs::File, path::Path, prelude::*};
use builder::config::Config;
use futures::future::{BoxFuture, FutureExt};
use std::env;
use std::path::PathBuf;

/// `help()` prints out the CLI's commands, instructions on how to use each command, and other helpful information
pub fn help() {
    println!("build, b <target> -> Scans, compiles and assembles your static site from <target> directory files in your default directory (./_site)");
    println!("init -> creates a pre-built application structure in the current directory");
    println!("--help ->")
}
/// `version()` prints the application's version
pub fn version() {
    println!("{0}", env!("CARGO_PKG_VERSION"))
}

pub async fn serve(port: u16) -> std::io::Result<()> {
    serve::listen(port)
}

/// `install()` fetches a package from the registry and registers appropriate commands with inert
pub async fn install<T: std::fmt::Display + Clone + Copy>(pkg: T) -> Result<(), String> {
    if let Ok(cwd) = std::env::current_dir() {
        let build_dir = cwd.join("_build/");
        if !build_dir.exists() {
            if let Err(_) = fs::create_dir_all(&build_dir).await {
                return Err(
                    "Error: cannot create _build directory to host dependency files".to_owned(),
                );
            }
        }
        let target = cwd.join("inert.json");
        if target.exists() {
            let mut config = Config::from(&target);
            config.add_dependency(pkg).await?;
            // dbg!(&config);
            if let Err(_) = write_config_file(target, config).await {
                return Err("unable to update inert.json".to_owned());
            }
        } else {
            return Err("Config file 'inert.json' not found. Please fun `$ inert init` or cd into the appropriate directory".to_owned());
        }
    }
    Ok(())
}

async fn write_config_file(target: PathBuf, config: Config) -> std::io::Result<()> {
    if target.exists() {
        fs::remove_file(&target).await?;
        let mut file = File::create(&target).await?;
        let newf = Config { ..config };
        let serialized = serde_json::to_string_pretty(&newf).unwrap();
        file.write_all(serialized.as_bytes()).await?;
    }
    Ok(())
}

pub async fn init() -> std::io::Result<()> {
    if let Ok(cwd) = std::env::current_dir() {
        let target = cwd.join("inert.json");
        if target.exists() {
            let config = Config::from(&target);
            fs::remove_file(&target).await?;
            let mut file = File::create(&target).await?;
            let newf = Config { ..config };
            let serialized = serde_json::to_string_pretty(&newf).unwrap();
            file.write_all(serialized.as_bytes()).await?;
        } else {
            let config = Config::new();
            let mut file = File::create("./inert.json").await?;
            let deserialized = serde_json::to_string_pretty(&config).unwrap();
            file.write_all(deserialized.as_bytes()).await?;
        }
    }

    Ok(())
}

/// `build()` generates inert static files in CONFIG:default_dir
///
/// *standard default dir is* `_site`
pub async fn build<'a>(target: Option<&str>) -> Result<(), String> {
    if let Some(target) = target {
        let parse_target = format!("{0}", target);
        async_builder(&parse_target).await;
    } else {
        async_builder(".").await;
    }
    Ok(())
}
/// `build_holder` is a static container function that facilitates the recursive nature of
/// [async_builder()](#cmd.async_builder)

/// async_builder recursivily scans through the child documents of the provided path, calling itself if it detects a directory
/// this design is to facilitate a 1:1 directory structure copying into the **default** site directory.
fn async_builder<'a>(target: &str) -> BoxFuture<()> {
    async move {
        if let Ok(mut entries) = fs::read_dir(target).await {
            while let Some(entry) = entries.next().await {
                if let Ok(entry) = entry {
                    // check to skip over program specific dir's _site & _build
                    if let Some(s) = Path::new(&entry.path()).file_name() {
                        if let Some(s) = s.to_str() {
                            match s {
                                "_build" | "_site" => continue,
                                _ => (),
                            }
                        }
                    };

                    if let Ok(file_type) = entry.file_type().await {
                        if file_type.is_dir() {
                            let new_path = &entry.path();
                            if let Some(np) = new_path.to_str() {
                                async_builder(np).await;
                                continue;
                            }
                        }
                    }
                    let path = entry.path();
                    if let Some(ext) = Path::new(&path).extension() {
                        match ext.to_str() {
                            Some(".html") | Some("html") => {
                                builder::html::html(&path, &entry).await.unwrap()
                            }
                            Some(".md") | Some("md") => {
                                builder::markdown::markdown(&path, &entry).await.unwrap()
                            }
                            _ => (),
                        }
                    }
                }
            }
        } else {
            eprintln!("Unable to parse {target}");
        }
    }
    .boxed()
}
