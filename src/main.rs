use std::{io, time::Duration, thread};
use clap::Parser;
use indicatif::ProgressBar;

// Error E0277 =P
#[derive(Debug)]
// Search for a patern in a file and display the lines that contain it.
#[derive(Parser)]
    struct Aidk {
    // The patern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct Error(String);

fn main() -> Result<(), Error> {
    // Loading...
    println!("Loading bullshit...hope it fails lol");
    let pb = ProgressBar::new(1024);
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    // Get the path
    let path = "test.txt";
    // Read the file
    let content = std::fs::read_to_string(path)
    // Bloody errors TwT
        .map_err(|err| Error(format!("Annnnd the error is `{}`: {} TwT", path, err)))?;
    // Welcome screen
    println!("Welcome to hell uvu\n");
    // Variable to store the word
    let mut word = String::new();
    // Reading the line
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line x-x");
    // Clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // Print out
    println!("{}", word);
    // End screen
    println!("Enjoy you're stay <3\n");
    // Print content
    println!("Oh ya, the file contents are {:#?} :^", content);
    Ok(())
}