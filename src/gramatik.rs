use std::collections::HashSet;

use crate::alphabet::{Alphabet, Letter, Word};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Rule {
    pub start: Word,
    pub end: Word
}

impl Rule {
    pub fn new(start: Word, end: Word) -> Self {
        Self { start, end }
    }
}

pub struct RuleSet {
    rules: HashSet<Rule>
}

impl RuleSet {
    pub fn new() -> Self {
        Self { rules: HashSet::new() }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.insert(rule);
    }
}

impl std::fmt::Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut output = String::new();
        for rule in self.rules.iter() {
            output.push_str(&format!("{} -> {}\n", rule.start, rule.end));
        }
        write!(f, "{}", output)
    }
}

impl std::ops::Add for RuleSet {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut rules = self.rules;
        for rule in other.rules.into_iter() {
            rules.insert(rule);
        }
        Self { rules }
    }
}

#[macro_export]
macro_rules! ruleset {
    ($a:ident -> $b:stmt) => {
        { 
            let mut ruleset = gramatik::RuleSet::new();

            let start = stringify!($a).to_owned();
            let targets = stringify!($b).split("|").map(|r| r.trim().replace(";", "")).collect::<Vec<String>>();

            for target in targets.into_iter() {
                let rule = gramatik::Rule::new(start.clone(), target);
                ruleset.add_rule(rule);
            }

            ruleset
        }
    };
    ($a:ident -> $b:stmt, $($c:ident -> $d:stmt),+) => {
        ruleset!($a -> $b) + ruleset!($($c -> $d),+)
    }
}

/// Represents a G<N, T, P, S> grammar.
///   N: Alphabet of non terminal symbols
///   T: Aplhabet of terminal symbols
///   P: The grammar rules
///   S: Start symbol
pub struct Gramatik(pub Alphabet, pub Alphabet, pub RuleSet, pub Letter);

impl Gramatik {
    pub fn new(terminals: Alphabet, non_terminal: Alphabet, ruleset: RuleSet, start: Letter) -> Self {
        Self(terminals, non_terminal, ruleset, start)
    }
}
