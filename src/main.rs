use std::fmt;
use std::collections::BTreeSet;
use std::io::{stdin, stdout, Stdin, Write};

extern crate rand;
use rand::prelude::*;

mod scene;
use scene::SCENE;

mod dict;
use dict::WORDS;

fn main() {
    println!("Hangman!");

    let mut rng = rand::thread_rng();
    let word = &WORDS.choose(&mut rng).unwrap();
    let mut game = Game::new(word);

    let outcome = game.play();

    println!("{}\n{}\n{}", game, outcome, word.to_uppercase());
}

struct Game {
    word: String,
    guessed_right: BTreeSet<char>,
    guessed_wrong: BTreeSet<char>,
}

impl Game {
    fn new(word: &str) -> Game {
        Game {
            word: word.to_string(),
            guessed_right: BTreeSet::new(),
            guessed_wrong: BTreeSet::new(),
        }
    }

    fn play(&mut self) -> GameOutcome {
        loop {
            println!("{}", self);
            
            let guess = self.guess(&stdin());
            self.insert(guess);

            if let PlayOutcome::GameOver(outcome) = self.check_game() {
                stdout().flush().expect("Problem writing to stdout!");
                break outcome
            }
        }
    }

    fn guess(&mut self, input: &Stdin) -> char {
        print!("Guess a letter: ");
        stdout().flush().expect("Problem writing to stdout!");

        let mut buf = String::new();
        input.read_line(&mut buf).expect("Problem reading from stdin!");

        match buf.to_lowercase().trim() {
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" |
            "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" => {
                let character = buf.to_lowercase().chars().nth(0).unwrap();
                if self.contains(&character) {
                    println!("You guessed '{}' already!", &character);
                    self.guess(&stdin())
                } else {
                    character
                }
            },
            _ => {
                println!("Invalid guess!");
                self.guess(&stdin())
            }
        }

    }

    fn check_game(&self) -> PlayOutcome {
        if self.word.chars().all(|c| self.guessed_right.contains(&c)) {
            PlayOutcome::GameOver(GameOutcome::Win)
        } else if self.guessed_wrong.len() >= SCENE.len() - 1 {
            PlayOutcome::GameOver(GameOutcome::Lose)
        } else {
            PlayOutcome::Next
        }
    }

    fn insert(&mut self, c: char) {
        if self.word.contains(&c.to_string()) {
            self.guessed_right.insert(c);
        } else {
            self.guessed_wrong.insert(c);
        }
    }

    fn contains(&self, character: &char) -> bool {
        self.guessed_right.contains(character) ||
        self.guessed_wrong.contains(character)
    }
}

#[derive(Debug)]
enum GameOutcome {
    Win,
    Lose,
}

enum PlayOutcome {
    GameOver(GameOutcome),
    Next,
}

impl fmt::Display for GameOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You {:?}!", self)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}\n{}",
            SCENE[self.guessed_wrong.len()],

            self.word.chars().map(|c| {
                if self.guessed_right.contains(&c) {
                    format!("{} ", c.to_uppercase())
                } else {
                    format!("_ " )
                }
            }).collect::<String>(),

            self.guessed_wrong.iter()
                .map(|c| format!("{} ", c))
                .collect::<String>(),
        )
    }
}