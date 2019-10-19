use std::collections::BTreeSet;
use std::fmt;
use std::io::{stdin, stdout, Stdin, Write};

use rand::prelude::*;

mod game;
use game::*;

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
    let word = loop {
        let rand_word = WORDS.choose(&mut rng).unwrap().to_owned();
        if let Ok(word) = Word::try_from(rand_word) {
            break word;
        }
    };

    println!("{} letters:", word.len());

    let mut game = Game::new(&word);

    let outcome = game.play();

    println!("{}\n{}\n{}", game, outcome, word);
}

