use std::env;
use std::process;

mod libs; 

#[tokio::main]
async fn main () {
    let config = libs::Args::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("query {}", config.query);
    println!("filePath {}", config.file_path);

    if let Err(e) = libs::run(config).await {
        eprintln!("application error: {e}");
        process::exit(1);
    };
}
