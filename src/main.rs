#![allow(unused)]
use crate::prelude::*;
use std::io::prelude::*;
use std::{io, time::Duration, thread};
use std::io::BufReader;
use std::fs::File;
use clap::Parser;
use indicatif::ProgressBar;

mod error;
mod prelude;
mod utils;

#[derive(Debug)]
#[derive(Parser)]
    struct Aidk {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    println!("Loading aidk...hope it fails lol");

    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }

    println!("Welcome to hell uvu\n");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line x-x");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("{}", word);

    println!("Enjoy you're stay <3\n");

    println!("Oh ya, the file contents are :^\n");

    let f = File::open("test.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("Damn the first line is {} bytes long XD", len);
    
    Ok(())
}
