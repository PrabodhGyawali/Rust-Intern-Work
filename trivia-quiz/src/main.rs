use std::env;
use std::fs;
use std::collections::HashMap;
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if enough arguments are provided
    if args.len() > 4 {
        println!("Usage: trivia-quiz [<number_of_questions>] [<categories>] [<difficulties>] [<types>]");
        println!("Use -h option for how to choose game options.");
        return;
    }

    // Man page check
    for arg in args.iter() {
        if arg == "-h" {
            if args.len() == 2 {
                let contents =  fs::read_to_string("man-page.txt")
                    .expect("Something went wrong reading the file");
                println!("{}", contents);
                return;
            }
            else {
                println!("Usage: trivia-quiz [<number_of_questions>] [<categories>] [<difficulties>] [<types>] [<encoding>]");
                println!("Use -h option for how to choose game options.");
                return;
            }
        }
    }
    
    let mut url : String = String::from("https://opentdb.com/api.php?amount=");
    // Ask for the number of questions
    println!("How many questions do you want? (default: 10)");
    let mut number_of_questions = String::new();
    std::io::stdin().read_line(&mut number_of_questions).expect("Failed to read input");
    let number_of_questions = number_of_questions.trim().to_lowercase();
    if !number_of_questions.is_empty() {
        url.push_str(&number_of_questions);
    } 
    else {
        url.push_str("10");
    }
    
    // Ask for the category
    println!("Choose a category: (default: any)");
    let mut category = String::new();
    std::io::stdin().read_line(&mut category).expect("Failed to read input");
    let category = category.trim().to_lowercase();
    if !category.is_empty() {
        url.push_str("&category=");
        url.push_str(&category);
    }

    // Ask for the difficulty
    println!("Choose a difficulty: (default: any)");
    let mut difficulty = String::new();
    std::io::stdin().read_line(&mut difficulty).expect("Failed to read input");
    let difficulty = difficulty.trim().to_lowercase();
    if !difficulty.is_empty() {
        url.push_str("&difficulty=");
        url.push_str(&difficulty);
    }

    // Ask for the type
    println!("Choose a type: (default: any)");
    let mut _type = String::new();
    std::io::stdin().read_line(&mut _type).expect("Failed to read input");
    let _type = _type.trim().to_lowercase();
    if !_type.is_empty() {
        url.push_str("&type=");
        url.push_str(&_type);
    }
    println!("{}", url); 
    
       
}

