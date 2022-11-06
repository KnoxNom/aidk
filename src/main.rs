use console::style;
use std::io;
use clap::Parser;

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

fn main() {
    // Something something
    let args = Aidk::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    // Welcome screen
    println!("{}", style("Welcome to hell uvu\n").bold().green());
    // Variable to store the word
    let mut word = String::new();
    // Reading the line
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line x-x");
    // Clear terminal
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // Print out
    println!("{}", style(word).yellow());
    // End screen
    println!("{}", style("Enjoy you're stay <3\n").bold().red());
    // Print out args and pattern
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
