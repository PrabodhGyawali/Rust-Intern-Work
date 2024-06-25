use serde::Deserialize;
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;

// Importing for sound using Rodio
use rodio::{Decoder, OutputStream, source::Source};

// Important to understand the Source Trait
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct QuizQuestion {
    song_path: String,
    options: Vec<String>,
    correct_answer: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check the number of arguments
    if !(args.len() == 2 || args.len() == 3) {
        eprintln!("Usage: cargo run <sound_file> [option]");
        return;
    }
    // Check for the help message
    if args[1] == "-h" {
        println!(
            "Help message: This program plays sounds from a file.\n\
                  cargo run [path] [options] -> the path to the file relative to cargo.toml\n\
                  --no-message -> optional parameter to not display message"
        );
        return;
    }
    // println!("{:?}", args);
    if args.len() == 3 && args[2] != "--no-message" {
        eprintln!("Invalid option");
        return;
    } else {
        println!("This is a numerical quiz game on songs.")
    }

    // Get the sound file path from the command line argument
    let sound_file_result: Result<File, std::io::Error> = File::open(&args[1]);

    let sound_file = match sound_file_result {
        Ok(file) => file,
        Err(error) => panic!("File not found: {error:?}"),
    };
    // Get data from the sound_file
    let reader = BufReader::new(sound_file);
    let quiz_data: Vec<QuizQuestion> = match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(_error) => panic!("File not Serializable: {_error:?}"),
    };

    let mut score: i8 = 0;
    // TODO: Add a 10 second timer
    for (index, question) in quiz_data.iter().enumerate() {
        println!(
            "Question {}: Guess the song. Press 'r' to repeat.",
            index + 1
        );

        // Song File path
        let file_path = format!("sounds/{}", question.song_path);
        println!("{:?}", file_path);
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to cargo.toml
        let file = BufReader::new(File::open(&file_path).unwrap());
        // Decode that sound file into a source
        println!("Debug breakpoint 1");
        let source = Decoder::new(file).unwrap();
        println!("Debug breakpoint 2");
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples());
        // Sound is in a seperate audio thread so we must keep it alive while it plays
        std::thread::sleep(std::time::Duration::from_secs(5));

        for (index, option) in question.options.iter().enumerate() {
            println!("{}: {}", index + 1, option);
        }
        // Obtain User Input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        let trimmed_input = user_input.trim();
        let mut trimmed_input_int: usize = match trimmed_input.parse() {
            Ok(val) => val,
            Err(_e) => {
                continue;
            },
        };

        while trimmed_input_int != 1 && trimmed_input_int != 2 && trimmed_input_int != 3 {
            println!("Invalid option. Please enter a valid number (1, 2, or 3):");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read user input");

            let trimmed_input = user_input.trim();
            trimmed_input_int = match trimmed_input.parse() {
                Ok(val) => val,
                Err(_e) => {
                    continue;
                },
            };
        }
        let index: usize = trimmed_input_int - 1;
        if question.options[index] == question.correct_answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect!");
        }
    }
}

// // Get an output stream handle to the default physical sound device
// let (_stream, stream_handle) = OutputStream::try_default().unwrap();
// // Load a sound from a file, using a path relative to Cargo.toml
// println!("{:?}", args)
