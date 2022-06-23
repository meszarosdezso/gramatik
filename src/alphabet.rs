use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;
use std::iter::IntoIterator;

pub const EPSILON: char = 'ε';

pub type Letter = char;
pub type Word = String;

pub struct Alphabet {
    letters: HashSet<Letter>,
}

impl Alphabet {
    pub fn contains(&self, letter: Letter) -> bool {
        self.letters.contains(&letter)
    }

    pub fn iter(&self) -> Iter<Letter> {
        self.letters.iter()
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

impl<I> From<I> for Alphabet
where
    <I as IntoIterator>::Item: Hash + Eq + PartialEq + Into<char>,
    I: IntoIterator,
{
    fn from(i: I) -> Self {
        let letters = i.into_iter().map(|e| e.into());
        Self {
            letters: HashSet::from_iter(letters),
        }
    }
}
