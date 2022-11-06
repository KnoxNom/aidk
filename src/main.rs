use console::style;
use std::io;

fn main() {
    println!("{}", style("Welcome to hell uvu\n").bold().green());
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line x-x");

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("{}", style(word).yellow());
    println!("{}", style("Enjoy you're stay <3\n").bold().red());
}
