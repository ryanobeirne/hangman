use std::collections::BTreeSet;
use std::fmt;
use std::io::{stdin, stdout, Stdin, Write};

mod game;
use game::*;

mod scene;
use scene::SCENE;

mod dict;
use dict::rand_word;

mod letters;
use letters::*;

mod words;
use words::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hangman!");

    let mut output = stdout();

    let mut multi = MultiGame::new();

    while let GameOutcome::Win(score) = play_new_game() {
        multi.game_count += 1;
        multi.score += score;
        multi.status(&mut output)?;
    }

    multi.status(&mut output)?;

    Ok(())
}

fn play_new_game() -> GameOutcome {
    let word = match std::env::var("WORD") {
        Ok(word) => Word::try_from(word.as_str()).unwrap_or(rand_word()),
        Err(_) => rand_word(),
    };
    println!("{} letters:", word.len());

    let mut game = Game::new(&word);

    let outcome = game.play();

    println!("{}\n{}\n{}", game, outcome, word);

    outcome
}

#[derive(Debug)]
struct MultiGame {
    game_count: usize,
    score: usize,
}

impl MultiGame {
    fn new() -> Self {
        MultiGame {
            game_count: 0,
            score: 0,
        }
    }

    fn status<W: Write>(&self, w: &mut W) -> std::io::Result<()> {
        writeln!(w, "Games: {} | Score: {}", self.game_count, self.score)
    }
}
