use std::{io, time::Duration, thread};
use clap::Parser;
use indicatif::ProgressBar;

#[derive(Debug)]
#[derive(Parser)]
    struct Aidk {
    pattern: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct Error(String);

fn main() -> Result<(), Error> {
    println!("Loading aidk...hope it fails lol");

    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }

    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| Error(format!("Annnnd the error is `{}`: {} TwT", path, err)))?;

    println!("Welcome to hell uvu\n");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line x-x");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("{}", word);

    println!("Enjoy you're stay <3\n");

    println!("Oh ya, the file contents are {:#?} :^", content);
    Ok(())
}
