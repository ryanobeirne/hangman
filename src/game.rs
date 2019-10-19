use super::*;

pub struct Game {
    word: Word,
    guessed_right: BTreeSet<Letter>,
    guessed_wrong: BTreeSet<Letter>,
}

impl Game {
    pub fn new(word: &Word) -> Game {
        Game {
            word: word.to_owned(),
            guessed_right: BTreeSet::new(),
            guessed_wrong: BTreeSet::new(),
        }
    }

    pub fn play(&mut self) -> GameOutcome {
        loop {
            println!("{}", self);

            let guess = self.guess(&stdin());
            self.insert(guess);

            if let PlayOutcome::GameOver(outcome) = self.check_game() {
                stdout().flush().expect("Problem writing to stdout!");
                break outcome;
            }
        }
    }

    fn guess(&mut self, input: &Stdin) -> Letter {
        print!("Guess a letter: ");
        stdout().flush().expect("Problem writing to stdout!");

        let mut buf = String::new();
        input
            .read_line(&mut buf)
            .expect("Problem reading from stdin!");

        match Letter::try_from(buf.as_str()) {
            Ok(letter) => {
                if self.contains(&letter) {
                    println!("You guessed '{}' already!", &letter);
                    self.guess(&stdin())
                } else {
                    letter
                }
            }
            Err(_) => {
                println!("Invalid guess!");
                self.guess(&stdin())
            }
        }
    }

    fn check_game(&self) -> PlayOutcome {
        if self.word.letters().all(|l| self.guessed_right.contains(&l)) {
            let score = self.score();
            PlayOutcome::GameOver(GameOutcome::Win(score))
        } else if self.guessed_wrong.len() >= SCENE.len() - 1 {
            PlayOutcome::GameOver(GameOutcome::Lose)
        } else {
            PlayOutcome::Next
        }
    }

    pub fn score(&self) -> usize {
        let correct = self.guessed_right.len();
        let diff = correct as isize - self.guessed_wrong.len() as isize;
        if diff > 0 {
            correct + diff as usize
        } else {
            correct
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
        self.guessed_right.contains(letter) || self.guessed_wrong.contains(letter)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum GameOutcome {
    Win(usize),
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
            self.word
                .letters()
                .map(|l| {
                    if self.guessed_right.contains(&l) {
                        format!("{} ", l)
                    } else {
                        format!("_ ")
                    }
                })
                .collect::<String>(),
            self.guessed_wrong
                .iter()
                .map(|l| format!("{} ", l.to_lowercase()))
                .collect::<String>(),
        )
    }
}
