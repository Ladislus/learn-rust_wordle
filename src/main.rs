extern crate core;

use std::process::exit;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, stdin, Stdin, stdout, Write};
use rand::{seq::SliceRandom, thread_rng};
use colored::{Colorize};

/// Reads lines from a file (delimited by newlines) and returns a vector of strings.
/// # Arguments
/// * `input_file_path` - The path to the file to read from.
/// # Returns
/// A vector of strings, each string being a line from the file.
fn get_lines(input_file_path: &str) -> Vec<String> {
    print!("[DEBUG] Reading input file: {} => ", input_file_path);
    // Open file with options (read only)
    let mut file: File = match OpenOptions::new().read(true).open(input_file_path) {
        // File was opened successfully
        Ok(file) => {
            println!("(SUCCESS)");
            file
        },
        // File could not be opened & print specific error message for some error types
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    println!("(NOT FOUND)");
                    exit(1);
                }
                ErrorKind::PermissionDenied => {
                    println!("(PERMISSION DENIED)");
                    exit(1);
                }
                _ => {
                    println!("(ERROR {:?})", error);
                    exit(1);
                }
            }
        }
    };
    // Read file into a string
    print!("[DEBUG] Reading content => ");
    let mut content: String = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {
            println!("(SUCCESS)");
            content.lines().map(|line| line.to_string()).collect()
        },
        Err(error) => {
            println!("(ERROR {:?})", error);
            exit(1);
        }
    }
}

// Struct that contains game information
struct Wordle {
    word: String,
    lives: u8,
    guesses: Vec<String>
}

// Constructor for Wordle struct with default values
impl Wordle {
    fn new(word: String) -> Wordle {
        Wordle {
            word,
            lives: 5,
            guesses: Vec::new()
        }
    }

    fn check(&self, guess: &String) -> bool {
        if guess.to_string() == self.word {
            println!("{}", guess.to_string().green());
            true
        } else {
            let mut correct_word_chars: Vec<char> = self.word.chars().collect();
            for (index, character) in guess.chars().enumerate() {
                // Can unwrap because we know the two strings are the same size
                if character == self.word.chars().nth(index).unwrap() {
                    correct_word_chars.remove(correct_word_chars.iter().position(|x| *x == character).unwrap());
                    print!("{}", character.to_string().green());
                } else if correct_word_chars.contains(&character) {
                    // Find first element in correct_word_chars that matches character (can unwrap can we know correct_word_chars contains character)
                    correct_word_chars.remove(correct_word_chars.iter().position(|x| *x == character).unwrap());
                    print!("{}", character.to_string().yellow());
                } else {
                    print!("{}", character.to_string().red());
                }
            }
            // Print newline & flush stdout
            println!();
            false
        }
    }
}


fn main() {
    // Load words from file
    println!("Hello on This-Is-Totally-Not-A-Wordle-Knockoff!");
    let words: Vec<String> = get_lines("dictionnaries/french/french_words_115585.txt");

    // Select a random word from the list
    let word: String = match words.choose(&mut thread_rng()) {
        Some(word) => word.to_string(),
        None => {
            println!("No word found!");
            exit(1);
        }
    };
    println!("[DEBUG] Word: {}", word);
    println!("Selected word length: {}", word.len());
    // Initialize game informations
    let mut game: Wordle = Wordle::new(word);
    // Get a handle to the standard input
    let stdin: Stdin = stdin();
    let mut current_guess: String = String::new();
    let mut attempts: u8 = 0;
    while attempts < game.lives && current_guess != game.word {
        print!("Guess {}/{}: ", attempts + 1, game.lives);
        // Force flush stdout because print without new line might be buffered
        stdout().flush().unwrap();
        // Read the next line from stdin and store it in current_guess
        match stdin.read_line(&mut current_guess) {
            Ok(_) => {
                // Clean input
                current_guess = current_guess.trim().to_string();
                // Check if the attempt is a valid one
                if current_guess.len() != game.word.len() {
                    println!("{} is not the same length as the word!", current_guess);
                } else if game.guesses.contains(&current_guess) {
                    println!("You already tried this word!");
                } else {
                    game.guesses.push(current_guess.to_string());
                    attempts += 1;
                    // TODO: Check if the guess is correct & print colored output otherwise
                    if game.check(&current_guess) {
                        exit(0);
                    }
                }
                current_guess.clear();
            }
            Err(error) => {
                println!("[DEBUG] Error reading line: {:?}", error);
            }
        }
    }
    println!("Failed, the word was '{}'", game.word);
}
