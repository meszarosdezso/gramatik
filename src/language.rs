use std::collections::HashSet;

use crate::{alphabet::Word, automata::Automata};

/// Represents a language.
pub struct Language {
    words: HashSet<Word>,
}

impl From<Automata<'_>> for Language {
    fn from(_automata: Automata) -> Self {
        let words = HashSet::new();

        Self { words }
    }
}

impl std::fmt::Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.words)
    }
}
