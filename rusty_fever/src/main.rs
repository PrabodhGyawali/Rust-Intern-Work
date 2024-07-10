use rusty_fever::game::play;

use std::process::Command;

fn main() {
    println!("
██████╗░██╗░░░██╗███████ ████████╗██    ██╗░
██╔══██╗██║░░░██║██      ╚══██╔══╝ ██  ██═╝░
██████╦╝██║░░░██║███████░░░░██║░░░  ████═╝░
██╔══██╗██║░░░██║     ██░░░░██║░░░   ██═╝░░
██║░░██║╚██████╔╝███████ ░░░██║░░░   ██║░░░
╚═╝░░╚═╝░╚═════╝░╚══════╝░░░╚═╝░░░   ╚═╝░░░
    ");
    println!("Press Enter to play or 'exit' to quit...");
    let stdin = std::io::stdin();
    let mut input = String::new();
    input = stdin.read_line(&mut input).unwrap();
    while input.trim() != "exit" {
        if input.trim() == "" {
            Command::new("clear").status().unwrap();
            println!("Starting game...");
        } else {
            Command::new("clear").status().unwrap();
            println!("
██████╗░██╗░░░██╗███████ ████████╗██    ██╗░
██╔══██╗██║░░░██║██      ╚══██╔══╝ ██  ██═╝░
██████╦╝██║░░░██║███████░░░░██║░░░  ████═╝░
██╔══██╗██║░░░██║     ██░░░░██║░░░   ██═╝░░
██║░░██║╚██████╔╝███████ ░░░██║░░░   ██║░░░
╚═╝░░╚═╝░╚═════╝░╚══════╝░░░╚═╝░░░   ╚═╝░░░
    ");
    println!("Press Enter to play or 'exit' to quit...");
        }
        play();
        input = stdin.read_line(&mut input).unwrap();
    }
}