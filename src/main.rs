use std::collections::HashMap;

struct Letter {
    guessed: bool,
    in_word: bool,
    in_position: bool,
}
struct FeudleGame {
    letter_map: HashMap<char, Letter>,
    word: String,
    total_guesses: u32,
    guesses: u32,
}

impl FeudleGame {
    //new func
    fn new(word: String) -> FeudleGame {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut letters = HashMap::new();
        for letter in alphabet.chars() {
            letters.insert(letter, Letter {
                guessed: false,
                in_word: false,
                in_position: false,
            });
        }
        FeudleGame {
            letter_map: letters,
            word: word.to_ascii_uppercase(),
            total_guesses: 6,
            guesses: 0,
        }
    }
    //guess func take word 
    fn guess(&mut self, word_guess: &String) -> bool {
        for ch in word_guess.chars() {
            //check if an alphabet
            if ch.is_alphabetic() {
                let mut letter = self.letter_map.get_mut(&ch).unwrap();
                letter.guessed = true;
                if self.word.contains(ch) {
                    letter.in_word = true;
                }
                //set if letter is in position of word
                for (i, c) in self.word.chars().enumerate() {
                    if c == self.word.to_string().chars().nth(0).unwrap() {
                        letter.in_position = true;
                    }
                }
            }
        }
            self.guesses += 1;
            self.check_win()
        }

    fn check_win(&self) -> bool {
        for letter in self.word.chars() {
            let letter = self.letter_map.get(&letter).unwrap();
            if !letter.in_position {
                return false;
            }
        }
        return  true;
    }

    fn print_word(&self) {
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
    }

}

fn main() {
    let mut game = FeudleGame::new("hello".to_string());
    let mut word_guess = String::new();
    loop {
        println!("Guess a letter");
        
        std::io::stdin().read_line(&mut word_guess).expect("Failed to read line");
        game.guess(&word_guess);
        game.print_word();
        if game.check_win() {
            println!("You win!");
            break;
        }
        if game.guesses == game.total_guesses {
            println!("You lose!");
            break;
        }
    }
}
