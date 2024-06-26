use std::env;
use std::fs;
use std::collections::HashMap;
use reqwest;
use serde_json;

use rand::seq::SliceRandom;

struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}

fn valid_input_range(input: &str, min: u8, max: u8) -> bool {
    let mut input = input.to_string();
    if input.is_empty() {
        return true;
    }
    else if {
        input.parse::<u8>().is_err() || input.parse::<u8>().unwrap() < min || input.parse::<u8>().unwrap() > max
    } {
        return true;
    }
    else {
        return false;
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
        let number_of_questions = String::from("10");
    }
    
    // Ask for the category
    println!("Choose a category: (default: any)");
    let mut category = String::new();
    std::io::stdin().read_line(&mut category).expect("Failed to read input");
    category = category.trim().to_lowercase();
    if !category.is_empty() {
        while valid_input_range(&category, 1, 20) {
            println!("Invalid input. \nRewrite your answer: ");
            category = String::new();
            std::io::stdin().read_line(&mut category).expect("Failed to read input");
        }
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

    url.push_str("&type=multiple");

    // Send API GET request
    let response = reqwest::blocking::get(&url)
        .expect("Failed to send GET request");
    let body = response.text()
        .expect("Failed to retrieve response body");
    println!("{}", body);
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
            println!("Enter your answer: ");
            let mut answer = String::new();
            std::io::stdin().read_line(&mut answer).expect("Failed to read input");
            while answer.trim().is_empty() || answer.trim().parse::<usize>().is_err() || answer.trim().parse::<usize>().unwrap() > options.len() || answer.trim().parse::<usize>().unwrap() < 1{
                println!("Invalid input. \nRewrite your answer: ");
                answer = String::new();
                std::io::stdin().read_line(&mut answer).expect("Failed to read input");
            }
            let answer = answer.trim().parse::<usize>().expect("Failed to parse answer");

            if options[answer - 1] == question.answer {
                println!("Correct!");
                score += 1;
            }
            else {
                println!("Incorrect!");
            }
        }
        println!("Your score: {}/{}", score, number_of_questions);
    }
    else {
        println!("API Response code: {}", json["response_code"]);
        return;
    }
}