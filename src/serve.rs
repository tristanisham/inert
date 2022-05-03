use actix_files as fs;
use actix_web::{App, HttpServer};
use async_std::path::Path;
// use env_logger;
use termion::color;

/// I know this is overkill. I have to skim this down into a simple file server.

#[tokio::main]
pub(super) async fn listen(port: u16) -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    if !Path::new("_site").exists().await {
        eprintln!("Error: default dir ./_site was not found. Try running $ inert build to make it");
        std::process::exit(1);
    }
    println!(
        "Webserver starting on... http://localhost:{0}{port}{1}",
        color::Fg(color::Green),
        color::Fg(color::Reset)
    );
    HttpServer::new(|| {
        App::new()
            // .wrap(Logger::default())
            .service(fs::Files::new("/", "./_site").index_file("index.html"))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
