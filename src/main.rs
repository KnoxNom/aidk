use console::style;
use std::io;

fn main() {
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
}
