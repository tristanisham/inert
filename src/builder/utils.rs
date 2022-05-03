use async_std::{fs, fs::File, io::prelude::*, path::Path};

pub(super) async fn write_string_to_file(
    data: &str,
    name: &str,
    extension: &str,
) -> Result<(), String> {
    if let Ok(cwd) = std::env::current_dir() {
        let cwd = cwd.to_string_lossy();
        // Checks if default dir is created, and if not creates it.
        if !Path::new("./_site/").exists().await {
            fs::create_dir_all("_site").await.unwrap();
        }
        let (pth, file_name, _) = get_workable_path(name).await;
        let path = format!("./_site/{0}/{1}.{2}", pth, file_name, extension);
        if Path::new(&path).exists().await {
            if let Err(e) = fs::remove_file(&path).await {
                eprintln!("{}", e);
            }
        }
        if let Err(_) = fs::create_dir_all(format!("{cwd}/_site/{pth}")).await {
            eprintln!("unable to create {path}");
        }
        if let Ok(mut file) = File::create(&path).await {
            return match file.write_all(data.as_bytes()).await {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{e}")),
            };
        }
    }
    Ok(())
}

/// `get_workable_path()` takes any path (as a String) and return a tuple containing:
///
/// 1. Path before file
/// 2. File name
/// 3. Extension
pub(super) async fn get_workable_path<'a>(path: &str) -> (String, String, String) {
    // let current_dir: PathBuf = match env::current_dir() {
    //     Ok(dir) => PathBuf::from(dir),
    //     Err(_) => panic!("init failed. Current grab current directory."),
    // };
    // let cwd = match current_dir.to_str() {
    //     Some(s) => s,
    //     None => ".",
    // };

    let path_segments: Vec<&str> = path.split(std::path::MAIN_SEPARATOR).collect();
    let mut last_path_item = "";
    let mut file_name = "";
    if let Some(s) = path_segments.last() {
        last_path_item = s;
        let file_semantics: Vec<&str> = s.split(".").collect();
        file_name = file_semantics[0];
    }

    let mut pre_string = String::new();
    for i in 0..path_segments.len() - 1 {
        let mut n = 0;
        for b in path_segments[i].chars() {
            if b == '.' {
                continue;
            }
            if n == 0 && b == '/' {
                continue;
            }
            pre_string.push(b);
            n += 1
        }
        pre_string.push(std::path::MAIN_SEPARATOR);
    }
    (pre_string, file_name.to_owned(), last_path_item.to_owned())
}
