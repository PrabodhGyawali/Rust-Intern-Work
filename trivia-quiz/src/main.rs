use std::env;
use std::fs;
use std::collections::HashMap;
use reqwest;
use serde_json;
use std::io::{self, Write};
use rand::seq::SliceRandom;

struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}

fn valid_input_range(input: &str, min: u8, max: u8) -> bool {
    match input.parse::<u8>() {
        Ok(num) => num >= min && num <= max, 
        Err(_) => false, 
    }
}

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
    
    // Initialize the number of questions
    let mut number_of_questions = String::new();
    // Loop for input validation
    loop {
        print!("No of questions (default=10): ");
        io::stdout().flush().unwrap(); // Important to understand flushing mechanism for understanding Stream Handling in Rust and other languages

        std::io::stdin().read_line(&mut number_of_questions).expect("Failed to read input");
        // Input sanitization
        let sanitized_input = number_of_questions.trim();
                                                        // .trim() makes String to &str
        // Input range check
        if valid_input_range(sanitized_input, 10, 50) || sanitized_input.is_empty() {
            number_of_questions = sanitized_input.to_string();
            break;
        }
        else {
            println!("Invalid! Please ensure input number between 10 & 50 or '\\n' for default.");
            number_of_questions.clear();
        }
    }

    // Push to URL
    if !number_of_questions.is_empty() {
        url.push_str(&number_of_questions);
    } 
    else {
        url.push_str("10");
        let number_of_questions = String::from("10");
    }
    
    // Initialize the category
    let mut category = String::new();
    loop {
        print!("Choose a category (default is any): ");
        io::stdout().flush().unwrap(); // Important to understand flushing mechanism for understanding Stream Handling in Rust and other languages
        std::io::stdin().read_line(&mut category).expect("Failed to read input");
        let sanitized_input = category.trim();
        if valid_input_range(sanitized_input, 1, 20) || sanitized_input.is_empty() {
            category = sanitized_input.to_string();
            break;
        }
        else {
            println!("Invalid! Please ensure input is between 1 & 20 or '\\n' for default.");
            category.clear();
        }
    }
    
    if !category.is_empty() {
        url.push_str("&category=");
        url.push_str(&category);
    }

    // Initialize the difficulty
    let mut difficulty = String::new();

    loop {
        print!("Choose a difficulty (default is any): ");
        io::stdout().flush().unwrap(); // Important to understand flushing mechanism for understanding Stream Handling in Rust and other languages
        std::io::stdin().read_line(&mut difficulty);
        let sanitized_input = difficulty.trim();
        if sanitized_input == "easy" || sanitized_input == "medium" || sanitized_input == "hard" || sanitized_input.is_empty() {
            difficulty = sanitized_input.to_string();
            break;
        }
        else {
            println!("Invalid! Please ensure input is either 'easy', 'medium', 'hard' or '\\n' for default.");
            difficulty.clear();
        }
    }
    if !difficulty.is_empty() {
        url.push_str("&difficulty=");
        url.push_str(&difficulty);
    }

    // Set type to multiple for now to NOT OVERCOMPLICATE this task
    url.push_str("&type=multiple");

    // Send API GET request
    let response = reqwest::blocking::get(&url)
        .expect("Failed to send GET request");
    let body = response.text()
        .expect("Failed to retrieve response body");
    //Debugging// println!("{}", body);
    let json : HashMap<String, serde_json::Value> = serde_json::from_str(&body)
        .expect("Failed to parse JSON");

    let mut score = 0;
    if json["response_code"].as_u64() == Some(0) {
        for question in json["results"].as_array().unwrap() {
            let question = Question {
                question: question["question"].as_str().unwrap().to_string(),
                options: question["incorrect_answers"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect(),
                answer: question["correct_answer"].as_str().unwrap().to_string(),
            };
            println!("{}", question.question);
            let mut options = question.options;
            options.push(question.answer.clone());
            let mut rng = rand::thread_rng();
            options.shuffle(&mut rng);
            let mut i = 1;
            for option in options.iter() {
                println!("{}. {}", i, option);
                i += 1;
            }
            let mut answer = String::new();
            print!("Enter your answer: ");
            io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut answer).expect("Failed to read input");
            while answer.trim().is_empty() || answer.trim().parse::<usize>().is_err() || answer.trim().parse::<usize>().unwrap() > options.len() || answer.trim().parse::<usize>().unwrap() < 1{
                print!("Invalid input. \nRewrite your answer: ");
                io::stdout().flush().unwrap();
                answer = String::new();
                std::io::stdin().read_line(&mut answer).expect("Failed to read input");
            }
            let answer = answer.trim().parse::<usize>().expect("Failed to parse answer");

            if options[answer - 1] == question.answer {
                println!("Correct!\n");
                score += 1;
            }
            else {
                println!("Incorrect!\n");
            }
        }
        println!("Your score: {}/{}", score, number_of_questions);
    }
    else {
        println!("API Response code: {}", json["response_code"]);
        return;
    }
}
