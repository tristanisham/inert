use crate::build;
use async_std::path::PathBuf;
use async_std::{fs, path::Path, prelude::*, future::Future};
use std::env;

// pub fn init() -> Result<(), String> {
//     let current_dir = match env::current_dir() {
//         Ok(dir) => dir,
//         Err(_) => panic!("init failed. Current grab current directory."),
//     };
//     create_init_files(current_dir)
// }

// fn create_init_files(_path: PathBuf) -> Result<(), String> {
//     Ok(())
// }

/// `help()` prints out the CLI's commands, instructions on how to use each command, and other helpful information
pub fn help() {
    println!("init -> creates a pre-built application structure in the current directory");
    println!("--help ->")
}
/// `version()` prints the application's version
pub fn version() {
    println!("{0}", env!("CARGO_PKG_VERSION"))
}

/// `build()` generates inert static files in CONFIG:default_dir
///
/// *standard default dir is* `_site`
pub async fn build(target: Option<&str>) -> Box<dyn Future<Output = std::io::Result<()>>> {
    let current_dir: PathBuf = match env::current_dir() {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => panic!("init failed. Current grab current directory."),
    };
    let target_dir = match target {
        Some(s) => PathBuf::from(s),
        None => current_dir.clone(),
    };

    let cwd = match current_dir.to_str() {
        Some(s) => s,
        None => ".",
    };
    let curpath = format!("{0}/_site", cwd);
    if !Path::new(&curpath).exists().await {
        fs::create_dir("_site").await;
    }

    if let Ok(mut entries) = fs::read_dir(target_dir).await {
        while let Some(entry) = entries.next().await {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_dir() {
                        let new_path = &entry.path();
                        if let Some(np) = new_path.to_str() {
                            build(Some(np));
                        }
                    }
                }
        
                let path = entry.path();
                if let Some(ext) = Path::new(&path).extension() {
                    match ext.to_str() {
                        Some(".html") | Some("html") => build::html(&path, &entry).await.unwrap(),
                        Some(".md") | Some("md") => build::markdown(&path, &entry).await.unwrap(),
                        _ => (),
                    }
                }
            }
            
        }
    }
    
    Future<Output = Ok(())>
}
