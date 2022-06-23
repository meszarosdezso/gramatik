use std::collections::HashSet;
use std::hash::Hash;

use crate::alphabet::{Alphabet, Letter, Word};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct State<'a>(pub &'a str);

impl<'a> std::fmt::Display for State<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for State<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

type DeltaFn<'a> = dyn Fn(State<'a>, Letter) -> Option<State>;

pub struct Automata<'a> {
    states: HashSet<State<'a>>,
    alphabet: Alphabet,
    delta_fn: &'a DeltaFn<'a>,
    initial_state: State<'a>,
    accepted_states: HashSet<State<'a>>,

    current_state: State<'a>,
    current_letter_index: usize,
}

impl<'a> Automata<'a> {
    pub fn new<S, AS>(
        states: S,
        alphabet: Alphabet,
        delta_fn: &'a DeltaFn<'a>,
        initial_state: State<'a>,
        accepted_states: AS,
    ) -> Result<Self, String>
    where
        <S as IntoIterator>::Item: Hash + Eq + PartialEq + Into<State<'a>>,
        S: IntoIterator,
        <AS as IntoIterator>::Item: Hash + Eq + PartialEq + Into<State<'a>>,
        AS: IntoIterator,
    {
        let states = HashSet::from_iter(states.into_iter().map(|e| e.into()));
        let accepted_states = HashSet::from_iter(accepted_states.into_iter().map(|e| e.into()));

        if states.is_empty() {
            return Err(format!("States should not be empty."));
        }

        if !states.contains(&initial_state) {
            return Err(format!(
                "Initial state ({initial_state}) is not a valid state of the automata!"
            ));
        }

        let intersection = states.intersection(&accepted_states);

        if intersection.count() != accepted_states.len() {
            return Err(format!("Accepted states should be a subset of the states."));
        }

        let current_state = initial_state.clone();

        Ok(Self {
            states,
            alphabet,
            delta_fn,
            initial_state,
            accepted_states,

            current_state,
            current_letter_index: 0,
        })
    }

    pub fn initial_state(&'a self) -> &'a State<'a> {
        &self.initial_state
    }

    pub fn current_state(&'a self) -> &'a State<'a> {
        &self.current_state
    }

    pub fn check_state(&'a self, state: &'a State<'a>) -> bool {
        self.accepted_states.contains(state)
    }

    pub fn process_word(&'a mut self, word: &str) -> WordProcessor {
        return WordProcessor::new(self, word);
    }
}

impl<'a> Iterator for Automata<'a> {
    type Item = Word;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct WordProcessor<'a> {
    automata: &'a mut Automata<'a>,
    word: String,
}

impl<'a> WordProcessor<'a> {
    fn new(automata: &'a mut Automata<'a>, word: &str) -> Self {
        Self {
            automata,
            word: word.to_string(),
        }
    }
}

impl<'a> Iterator for WordProcessor<'a> {
    type Item = State<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let delta = self.automata.delta_fn;
        if let Some(letter) = self.word.chars().nth(self.automata.current_letter_index) {
            if !self.automata.alphabet.contains(letter) {
                panic!("{letter} is not part of the automata's alphabet.")
            }

            if let Some(state) = delta(self.automata.current_state.clone(), letter) {
                if !self.automata.states.contains(&state) {
                    panic!(
                        "{state} is not a valid state for the automata. Valid states are: {:?}",
                        self.automata.states
                    )
                }

                self.automata.current_letter_index += 1;
                self.automata.current_state = state;

                return Some(self.automata.current_state.clone());
            }
        }

        None
    }
}

#[macro_export]
macro_rules! delta_fn {
    ($(($q:ident, $a:ident) -> $r:ident)*) => {
        |q: State, a: alphabet::Letter| {
            let a = a.to_string();
            let a = a.as_str();

            match (q.0, a) {
            $(
                (
                    stringify!($q),
                    stringify!($a)
                ) => Some(State(stringify!($r))),
            )*
            _ => None,
        }}
    };
}
