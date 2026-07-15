use std::collections::HashMap;
use std::io;
use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;

enum CharOrStr {
    Char(char),
    Str(String),
}

fn main() {

    startup();
    let word = get_word();
    let mut letters = Vec::<(u8, char)>::new();
    let mut found = Vec::<bool>::new();
    let mut count = 0;
    for c in word.chars() {
        letters.push((count, c));
        found.push(false);
        count += 1;
    }    
    let wordmap: HashMap<u8, char> = letters.into_iter().collect();
    let mut bad_guesses: u8 = 0;


    loop {

        status(&word, bad_guesses); //print current status of hanged man
        if bad_guesses >= 6 {
            break
        }

        let mut index: u8 = 0;
        for i in &found {
            if !i {
                print!("_");
                index += 1;
            } else {
                let c = &wordmap.get(&index);
                print!("{}", c.unwrap());
                index += 1;
            }
        }
        println!("\n");

        println!("Guess a letter or a word: ");
        match get_guess() {
            CharOrStr::Char(c) => {
                if wordmap.values().any(|&val| val == c) {
                    println!("Good guess! '{c}' is in the word.");
                    let keys: Vec<usize> = wordmap
                        .iter()
                        .filter(|&(_, value)| *value == c)
                        .map(|(key, _)| *key as usize)
                        .collect::<Vec<usize>>();
                    for key in keys {
                        let target_index = key;
                        if let Some(element) = found.get_mut(target_index){
                            *element = true;
                        }
                    }
                } else {
                    println!("Bad guess! '{c}' is not in the word.");
                    bad_guesses += 1;
                }
            },
            CharOrStr::Str(s) => {
                if s == word {
                    println!("The word was '{word}'. You win!");
                    break
                } else {
                    println!("Bad guess! Try again.");
                    bad_guesses += 1;
                }
            },
        }
        if found.iter().all(|&x| x == true) {
            println!("You solved the puzzle! The word was '{word}'.");
            break
        }
    }
}

fn startup() {
     println!("
    Welcome to Hangman!
    Find the hidden word by guessing which letters spell it out.
    Each incorrect guess gets you one step closer to the gallows :(
    Choose 'Guess the word' when you feel confident, but be warned...
    No coming back from a bad guess. Good luck!");
}

fn get_word() -> String {
    let contents = fs::read_to_string("words.txt").expect("Failed to read file.");

    let words: Vec<&str> = contents.lines().collect();

    let word = words.choose(&mut thread_rng()).expect("Word list is empty.");

    word.to_string()
}

fn get_guess() -> CharOrStr {
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess = guess.trim().to_string();

        let is_all_letters = !guess.is_empty() && guess.chars().all(char::is_alphabetic);
        if is_all_letters {
            return match guess.chars().count() {
                1 => CharOrStr::Char(guess.chars().nth(0).unwrap()),
                _ => CharOrStr::Str(guess),
            };
        } else {
            println!("Not a valid input. Please try again.");
        };
    }
}

fn status(word: &str, bad_guesses: u8) {
    match bad_guesses {
        0 => {
            print!("  
                ________
               |       |
               |
               |
               |
            ___|___         ");
        } 
        1 => {
            print!("  
                ________
               |       |
               |       0
               |
               |
            ___|___         ");
        } 
        2 => {
            print!("  
                ________
               |       |
               |       0
               |       |
               |
            ___|___         ");
        } 
        3 => {
            print!("  
                ________
               |       |
               |       0
               |      /|
               |
            ___|___         ");
        } 
        4 => {
            print!("  
                ________
               |       |
               |       0
               |      /|\\
               |
            ___|___         ");
        } 
        5 => {
            print!("  
                ________
               |       |
               |       0
               |      /|\\
               |      / 
            ___|___         ");
        } 
        _ => {
            println!("  
                ________
               |       |
               |       0
               |      /|\\
               |      / \\
            ___|___         ");
            println!("You lose! womp womp... Answer was '{word}'");
        }
    }
}