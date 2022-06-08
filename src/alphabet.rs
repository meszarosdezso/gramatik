use std::iter::IntoIterator;
use std::collections::HashSet;
use std::hash::Hash;
use std::fmt;

pub const EPSILON: &str = "";

pub type Letter = char;
pub type Word = String;

pub struct Alphabet {
    letters: HashSet<Letter>
}

impl Alphabet {
    pub fn from_iter<I>(i: I) -> Self 
    where <I as IntoIterator>::Item: Hash, I: std::iter::Iterator<Item = char> {
        Self {
            letters: HashSet::from_iter(i)
        }
    }
}

impl std::ops::Add for Alphabet {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let letters = self.letters.union(&other.letters).copied().collect();
        Self { letters }
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self.letters)
    }
}


