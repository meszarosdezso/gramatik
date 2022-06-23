use std::collections::HashSet;
use std::hash::Hash;

use crate::alphabet::{Alphabet, Letter};

#[derive(Eq, PartialEq, Hash, Debug)]
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

pub struct Automata<'a>(
    HashSet<State<'a>>,
    Alphabet,
    &'a dyn FnMut(&str, Letter) -> State,
    State<'a>,
    HashSet<State<'a>>,
);

impl<'a> Automata<'a> {
    pub fn new<S, AS>(
        states: S,
        alphabet: Alphabet,
        delta: &'a dyn FnMut(&str, Letter) -> State,
        start_state: State<'a>,
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

        if !states.contains(&start_state) {
            return Err(format!(
                "States should contain the start state ({start_state}) too!"
            ));
        }

        let intersection = states.intersection(&accepted_states);

        if intersection.count() != accepted_states.len() {
            return Err(format!("Accepted states should be a subset of the states."));
        }

        Ok(Self(states, alphabet, delta, start_state, accepted_states))
    }

    pub fn current_state(&self) -> &'a State {
        &(self.3)
    }

    pub fn start(&self) {}
}
