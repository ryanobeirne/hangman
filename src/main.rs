use std::collections::BTreeSet;
use std::fmt;
use std::io::{self, stdin, stdout, BufRead, BufReader, Write};

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
    let mut output = stdout();
    let mut input = BufReader::new(stdin());

    write!(&mut output, "Hangman!")?;

    let mut multi = MultiGame::new();

    while let GameOutcome::Win(score) = play_new_game(&mut output, &mut input)? {
        multi.game_count += 1;
        multi.score += score;
        multi.status(&mut output)?;
    }

    multi.status(&mut output)?;

    Ok(())
}

fn play_new_game<W: Write, R: BufRead>(w: &mut W, r: &mut R) -> io::Result<GameOutcome> {
    let word = match std::env::var("WORD") {
        Ok(word) => Word::try_from(word.as_str()).unwrap_or(rand_word()),
        Err(_) => rand_word(),
    };

    writeln!(w, "{} letters:", word.len())?;

    let mut game = Game::new(&word);

    let outcome = game.play(w, r)?;

    writeln!(w, "{}\n{}\n{}", game, outcome, word)?;

    Ok(outcome)
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

    fn status<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "Games: {} | Score: {}", self.game_count, self.score)
    }
}
