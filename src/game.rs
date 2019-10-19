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

    pub fn play<W: Write, R: BufRead>(&mut self, w: &mut W, r: &mut R) -> io::Result<GameOutcome> {
        loop {
            writeln!(w, "{}", self)?;

            let guess = self.guess(r, w)?;
            self.insert(guess);

            if let PlayOutcome::GameOver(outcome) = self.check_game() {
                stdout().flush().expect("Problem writing to stdout!");
                break Ok(outcome);
            }
        }
    }

    fn guess<R: BufRead, W: Write>(&mut self, r: &mut R, w: &mut W) -> io::Result<Letter> {
        write!(w, "Guess a letter: ")?;
        w.flush().expect("Problem writing to stdout!");

        let mut buf = String::new();
        r.read_line(&mut buf)?;
        //let mut bytes = r.bytes().filter_map(|b| b.ok());
        //while let Some(b) = bytes.next() {
            //if b == 0x0A || b == 0x0D {
                //break;
            //} else {
                //buf.push(b as char);
            //}
        //}

        match Letter::try_from(buf.as_str()) {
            Ok(letter) => {
                if self.contains(&letter) {
                    writeln!(w, "You guessed '{}' already!", &letter)?;
                    self.guess(r, w)
                } else {
                    Ok(letter)
                }
            }
            Err(_) => {
                writeln!(w, "Invalid guess: {:#?}", &buf.trim())?;
                self.guess(r, w)
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
