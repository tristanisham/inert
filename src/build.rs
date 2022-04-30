use async_std::{fs, fs::File, io::prelude::*, path::Path, path::PathBuf};
use pulldown_cmark::{html, Options, Parser};
use std::env;

pub(super) async fn html<P: AsRef<Path>>(path: &P, entry: &fs::DirEntry) -> Result<(), String> {
    Ok(())
}

/// `markdown()` is the public-facing generic function that wraps around `parse_markdown()`
pub(super) async fn markdown<P: AsRef<Path>>(path: &P, entry: &fs::DirEntry) -> Result<(), String> {
    match entry.path().to_str() {
        Some(name) => match parse_markdown(path.as_ref()).await {
            Ok(raw) => write_string_to_file(&raw, name, "html").await,
            Err(e) => Err(e.to_string()),
        },
        None => Err(format!("unable to parse file")),
    }
}
/// `parse_markdown()` opens
async fn parse_markdown(path: &Path) -> Result<String, &str> {
    let mut html_output = String::new();
    if let Ok(raw) = fs::read_to_string(path).await {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&raw, options);
        html::push_html(&mut html_output, parser);
    }
    Ok(html_output)
}

async fn write_string_to_file(data: &str, name: &str, extension: &str) -> Result<(), String> {
    let (_path, file_name) = get_workable_path(name).await;
    let path = format!("./_site/{0}.{1}", file_name, extension);
    if Path::new(&path).exists().await {
        if let Err(e) = fs::remove_file(&path).await {
            panic!("{}", e);
        }
    }
    if let Ok(mut file) = File::create(&path).await {
        return match file.write_all(data.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{e}")),
        };
    }
    Ok(())
}

/// `get_workable_path()` takes any path (as a String) and return a tuple containing:
/// 0. Vector of each individual path segment
/// 1. File name
async fn get_workable_path<'a>(path: &str) -> (Vec<String>, String) {

    let current_dir: PathBuf = match env::current_dir() {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => panic!("init failed. Current grab current directory."),
    };
    let cwd = match current_dir.to_str() {
        Some(s) => s,
        None => ".",
    };

    let path_segments: Vec<&str> = path.split(std::path::MAIN_SEPARATOR).collect();
    let last_path_item = path_segments.last();
    let mut file_name = "";
    if let Some(s) = last_path_item {
        let file_semantics: Vec<&str> = s.split(".").collect();
        file_name = file_semantics[0];
    }
    // I know this isn't fuckup proof
    let mut paths: Vec<String> = Vec::with_capacity(path_segments.len());
    for i in path_segments {
        paths.push(String::from(i));
    }

    (paths, file_name.to_owned())
}
