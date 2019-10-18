use super::*;
use std::{io, fmt};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Word {
    pub letters: Vec<Letter>,
}

impl Word {
   pub fn chars(&self) -> std::slice::Iter<Letter> {
        self.letters.iter()
    }

   pub fn contains(&self, letter: &Letter) -> bool {
       self.chars().any(|l| l == letter)
   }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.chars().collect::<String>())
    }
}

impl<'a> FromIterator<&'a Letter> for String {
    fn from_iter<I: IntoIterator<Item=&'a Letter>>(iter: I) -> Self {
        iter.into_iter().map(|l| l.to_string()).collect()
    }
}

impl TryFrom<&str> for Word {
    type Error = io::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut letters = Vec::new();

        for c in s.chars() {
            letters.push(Letter::try_from(c)?);
        }

        Ok(Word {
            letters
        })
    }
}
