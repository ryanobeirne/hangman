use std::fmt;
use std::collections::BTreeSet;
use std::io::{stdin, stdout, Stdin, Write};

extern crate rand;
use rand::prelude::*;

mod scene;
use scene::SCENE;

mod dict;
use dict::WORDS;

mod letters;
use letters::*;

mod words;
use words::*;

fn main() {
    println!("Hangman!");

    let mut rng = rand::thread_rng();
    let word = loop{
        let rand_word = WORDS.choose(&mut rng).unwrap().to_owned();
        if let Ok(word) = Word::try_from(rand_word) {
            break word;
        }
    };

    let mut game = Game::new(&word);

    let outcome = game.play();

    println!("{}\n{}\n{}", game, outcome, word);
}

struct Game {
    word: Word,
    guessed_right: BTreeSet<Letter>,
    guessed_wrong: BTreeSet<Letter>,
}

impl Game {
    fn new(word: &Word) -> Game {
        Game {
            word: word.to_owned(),
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

    fn guess(&mut self, input: &Stdin) -> Letter {
        print!("Guess a letter: ");
        stdout().flush().expect("Problem writing to stdout!");

        let mut buf = String::new();
        input.read_line(&mut buf).expect("Problem reading from stdin!");

        match Letter::try_from(buf.as_str()) {
            Ok(letter) => {
                if self.contains(&letter) {
                    println!("You guessed '{}' already!", &letter);
                    self.guess(&stdin())
                } else {
                    letter
                }
            },
            Err(_) => {
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

    fn insert(&mut self, c: Letter) {
        if self.word.contains(&c) {
            self.guessed_right.insert(c);
        } else {
            self.guessed_wrong.insert(c);
        }
    }

    fn contains(&self, letter: &Letter) -> bool {
        self.guessed_right.contains(letter) ||
        self.guessed_wrong.contains(letter)
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
                    format!("{} ", c)
                } else {
                    format!("_ ")
                }
            }).collect::<String>(),

            self.guessed_wrong.iter()
                .map(|c| format!("{} ", c))
                .collect::<String>(),
        )
    }
}
