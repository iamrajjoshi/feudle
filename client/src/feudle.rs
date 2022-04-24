use std::{collections::HashMap, io};
use std::io::Write;
struct Letter {
    guessed: bool,
    in_word: bool,
    in_position: bool,
}

pub struct Feudle {
    letter_map: HashMap<char, Letter>,
    word: String,
    total_guesses: u32,
    guesses: u32,
    guess: String,
}

impl Feudle {
    //new func
    pub fn new() -> Feudle {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut letters = HashMap::new();
        for letter in alphabet.chars() {
            letters.insert(letter, Letter {
                guessed: false,
                in_word: false,
                in_position: false,
            });
        }
        Feudle {
            letter_map: letters,
            word: String::new(),
            guess: String::new(),
            total_guesses: 6,
            guesses: 0,
        }
    }
    
    pub fn set_word(&mut self, word: String) {
        self.word =  word.to_ascii_uppercase();
    }

    //guess func take word 
    pub fn guess(&mut self, word_guess: &String) -> bool {
        let upper_case_word_guess = word_guess.to_ascii_uppercase();
        self.guess = upper_case_word_guess.clone();
        for ch in upper_case_word_guess.chars() {
            //check if an alphabet
            if ch.is_alphabetic() {
                let mut letter = self.letter_map.get_mut(&ch).unwrap();
                letter.guessed = true;
                if self.word.contains(ch) {
                    letter.in_word = true;
                }
                
                
            }
        }
        //set if letter is in position of word
        for (i, c) in upper_case_word_guess.chars().enumerate() {
            let mut letter = self.letter_map.get_mut(&c).unwrap();
            if c == self.word.to_string().chars().nth(i).unwrap() {
                letter.in_position = true;
            }
        }
            self.guesses += 1;
            self.check_win()
        }

    pub fn check_win(&self) -> bool {
        return self.guess == self.word;
    }

    pub fn check_lose(&self) -> bool {
        if self.guesses >= self.total_guesses {
            return true;
        }
        return false;
    }

    pub fn print_word(&self) {
        for letter in self.word.chars() {
            let cap = letter.to_ascii_uppercase();
            let alpha = self.letter_map.get(&letter).unwrap();
            if alpha.in_position {
                print!("{}", cap);
            } else {
                print!("_");
            }
        }
        println!("");
        io::stdout().flush().unwrap();
    }

    // pub fn update(&mut self, should_update : bool) {
        
    // }
}