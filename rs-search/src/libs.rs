use std::error::Error;

use clap::Parser;
use tokio::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub query: String,

    pub file_path: String,
}

impl Args {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        // let vec_args: Vec<String> = args.collect();
        // println!("{:?}", vec_args);
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("did not a file_path string"),
        };

        Ok((Args {query, file_path}))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

pub async fn run (config: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).await?;
    let result = search(&config.query, &contents);

    for line in result {
        println!("{line}");
    }
    Ok(())
}

