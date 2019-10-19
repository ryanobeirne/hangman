pub use std::convert::TryFrom;
use std::{fmt, io};
use Letter::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Letter {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl Letter {
    pub fn to_lowercase(&self) -> String {
        self.to_string().to_lowercase()
    }
}

impl TryFrom<&str> for Letter {
    type Error = std::io::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(match s.to_uppercase().trim() {
            "A" => A,
            "B" => B,
            "C" => C,
            "D" => D,
            "E" => E,
            "F" => F,
            "G" => G,
            "H" => H,
            "I" => I,
            "J" => J,
            "K" => K,
            "L" => L,
            "M" => M,
            "N" => N,
            "O" => O,
            "P" => P,
            "Q" => Q,
            "R" => R,
            "S" => S,
            "T" => T,
            "U" => U,
            "V" => V,
            "W" => W,
            "X" => X,
            "Y" => Y,
            "Z" => Z,
            _ => return Err(io::Error::from(io::ErrorKind::InvalidInput)),
        })
    }
}

impl TryFrom<char> for Letter {
    type Error = std::io::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        Letter::try_from(c.to_string().as_str())
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<char> for Letter {
    fn into(self) -> char {
        match self {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
            K => 'K',
            L => 'L',
            M => 'M',
            N => 'N',
            O => 'O',
            P => 'P',
            Q => 'Q',
            R => 'R',
            S => 'S',
            T => 'T',
            U => 'U',
            V => 'V',
            W => 'W',
            X => 'X',
            Y => 'Y',
            Z => 'Z',
        }
    }
}

#[test]
fn from_char() {
    assert_eq!(Letter::try_from('x').unwrap(), X);
    assert_eq!(Letter::try_from('X').unwrap(), X);

    assert!(Letter::try_from('1').is_err());
    assert!(Letter::try_from('@').is_err());
}

#[test]
fn from_str() {
    assert_eq!(Letter::try_from("x").unwrap(), X);
    assert_eq!(Letter::try_from("X").unwrap(), X);
    assert_eq!(Letter::try_from(" x\n").unwrap(), X);

    assert!(Letter::try_from("1").is_err());
    assert!(Letter::try_from("@").is_err());
}
