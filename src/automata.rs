use std::collections::HashSet;
use std::hash::Hash;

use crate::alphabet::{Alphabet, Letter};

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

type DeltaFn<'a> = dyn Fn(State<'a>, Letter) -> State;

pub struct Automata<'a>(
    HashSet<State<'a>>,
    Alphabet,
    &'a DeltaFn<'a>,
    State<'a>,
    HashSet<State<'a>>,
);

impl<'a> Automata<'a> {
    pub fn new<S, AS>(
        states: S,
        alphabet: Alphabet,
        delta: &'a DeltaFn<'a>,
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
        &self.3
    }

    pub fn process_word(&'a self, word: &str) -> State {
        let mut state = self.current_state().clone();
        for c in word.chars() {
            if !self.1.contains(c) {
                panic!("{c} is not in the automata's alphabet ({}).", self.1)
            }
            state = self.2(state, c);
        }
        return state;
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
                ) => State(stringify!($r)),
            )*
            _ => panic!("Invalid state: cannot handle '{a}' in {q} state."),
        }}
    };
}
