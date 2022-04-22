use anyhow::Result;
use std::{env, process};

mod queries;
use queries::create_score;

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let args = args.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();

    match args.get(1) {
        Some(&"create_score") => {
            create_score("https://e-hanoi.herokuapp.com/graphql").await?;
        }
        None => {
            eprintln!("Error: Too few arguments.");
            process::exit(1);
        }
        _ => {
            eprintln!("Error: Unknown query was selected.");
            process::exit(1);
        }
    }

    Ok(())
}
