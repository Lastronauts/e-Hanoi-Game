use anyhow::{bail, Result};
use serde::Serialize;
use std::fs::File;
use std::io::BufWriter;
use std::process::Command;
use std::env;

pub fn send_clear_time(clear_time: i64) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let token=args.get(1).expect("Error: Too few arguments.");
    json_to_file(token.to_string(), clear_time)?;

    let target = "./graphql.exe";
    match Command::new(target).arg("create_score").spawn() {
        Ok(_) => Ok(()),
        Err(err) => {
            let err_msg = format!(
                "Error: Could not launch e-Hanoi.\ndetails...{}\n指定されたファイル: {}",
                err, target
            );
            bail!(err_msg);
        }
    }
}

#[derive(Serialize)]
struct NewScore {
    token: String,
    clear_time: i64,
}

fn json_to_file(token: String, clear_time: i64) -> Result<()> {
    let file = File::create("tmp/query.json")?;
    let writer = BufWriter::new(file);

    let new_score = NewScore { token, clear_time };

    serde_json::to_writer(writer, &new_score).unwrap();

    Ok(())
}
