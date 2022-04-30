use async_std::{fs, fs::File, io::prelude::*, path::Path};
use pulldown_cmark::{html, Options, Parser};

pub(super) async fn html<P: AsRef<Path>>(path: &P, entry: &fs::DirEntry) -> Result<(), String> {
    Ok(())
}

/// `markdown()` is the public-facing generic function that wraps around `parse_markdown()`
pub(super) async fn markdown<P: AsRef<Path>>(path: &P, entry: &fs::DirEntry) -> Result<(), String> {
    match entry.file_name().to_str() {
        Some(name) => match parse_markdown(path.as_ref()).await {
            Ok(raw) => write_string_to_file(raw, name.to_string()).await,
            Err(e) => Err(e),
        },
        None => Err(format!("unable to parse file")),
    }
}
/// `parse_markdown()` opens
async fn parse_markdown(path: &Path) -> Result<String, String> {
    let mut html_output = String::new();
    if let Ok(raw) = fs::read_to_string(path).await {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(&raw, options);
        html::push_html(&mut html_output, parser);
    }
    Ok(html_output)
}

async fn write_string_to_file(data: String, name: String) -> Result<(), String> {
    if let Ok(mut file) = File::create(format!("./_site/{0}", name)).await {
        return match file.write_all(data.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("{e}")),
        };
    }
    Ok(())
}
