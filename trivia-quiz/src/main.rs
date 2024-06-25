use std::env;
use std::fs::File;
use stdbuffer::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments are provided
    if args.len() == 1 || args.len() > 6 {
        println!("Usage: trivia-quiz [<number_of_questions>] [<categories>] [<difficulties>] [<types>] [<encoding>]");
        println!("Use -h option for how to choose game options.");
        return;
    }

    // Parse command line arguments
    
    for arg in args.iter() {
        println!("{}", arg);
        if arg == "-h" {
            let man_page_path = String::from("man_page.txt");
            let contents = fs::read_to_string(man_page_path)
                .expect("Something went wrong reading the man page.");
            println!("{}", contents);
            return;
        }
    }
    let number_of_questions: u32 = args[1].parse().unwrap();
    let categories: &str = &args[2];
    let difficulties: &str = &args[3];
    let types: &str = &args[4];
    let encoding: &str = &args[5];

    // TODO: Implement the trivia quiz game logic using the provided settings

    println!("Trivia Quiz Game");
    println!("Number of Questions: {}", number_of_questions);
    println!("Categories: {}", categories);
    println!("Difficulties: {}", difficulties);
    println!("Types: {}", types);
}

