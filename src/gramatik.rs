use crate::alphabet::Alphabet;
use crate::alphabet::Letter;
use crate::alphabet::EPSILON;
use crate::rules::Rule;
use crate::rules::RuleSet;

/// Represents a G<N, T, P, S> grammar.
///   N: Alphabet of non terminal symbols
///   T: Alphabet of terminal symbols
///   P: The grammar rules
///   S: Start symbol
pub struct Gramatik(pub Alphabet, pub Alphabet, pub RuleSet, pub Letter);

impl Gramatik {
    #[allow(non_snake_case)]
    /// Creates a new grammar.
    ///
    /// Panics if the non-terminal alphabet does not contain
    /// the start symbol or if one of the rules has a letter
    /// outside of the grammar's terminal or non-terminal
    /// alphabet.
    ///
    /// Example:
    ///
    /// ```
    /// let gramatik = Gramatik::new(
    ///     ["A", "B", "C", "S"].into(),
    ///     ["a", "b", "c"].into(),
    ///     ruleset!(
    ///         S -> A | B
    ///         A -> a,
    ///         B -> _
    ///     ),
    ///     "S"
    /// ).unwrap();
    /// ```
    pub fn new(N: Alphabet, T: Alphabet, P: RuleSet, S: Letter) -> Result<Self, String> {
        if !N.contains(S) {
            return Err(format!(
                "The start symbol ({S}) must be in the non terminal alphabet ({N})."
            ));
        }

        for rule in P.rules.iter() {
            for c in rule.start.chars().chain(rule.end.chars()) {
                if !(N.contains(c) || T.contains(c) || c == EPSILON) {
                    return Err(format!("{c} is not part of the grammer at {rule}."));
                }
            }
        }

        Ok(Self(N, T, P, S))
    }
}

impl Gramatik {
    pub fn class(&self) -> u8 {
        for i in (0..=3).rev() {
            if self.is_class(i) {
                return i;
            }
        }

        panic!("This gramatik has no class.")
    }

    pub fn is_class(&self, class: u8) -> bool {
        let check_start = |rule: &Rule| -> bool {
            if rule.start.len() != 1 {
                return false;
            }

            let start = rule.start.chars().nth(0).unwrap();
            if !self.0.contains(start) && start != self.3 {
                return false;
            }
            true
        };

        match class {
            1 => {
                let kes = self.2.rules.iter().any(|r| r.end.contains(self.3));

                for rule in self.2.rules.iter() {
                    if kes && rule.start == self.3.to_string() && rule.end == EPSILON.to_string() {
                        return false;
                    }

                    if rule.start.len() > rule.end.len() {
                        return false;
                    }

                    for (s, e) in rule.start.chars().zip(rule.end.chars()) {
                        if s != e && !self.0.contains(s) {
                            return false;
                        }
                    }
                }
                true
            }
            2 => self.2.rules.iter().all(check_start),
            3 => {
                for rule in self.2.rules.iter() {
                    if !check_start(rule) {
                        return false;
                    }

                    for (i, c) in rule.end.chars().enumerate() {
                        if c != EPSILON && !self.1.contains(c) {
                            if i != rule.end.len() - 1 {
                                return false;
                            }
                        }
                    }
                }
                true
            }
            _ => panic!("Invalid class argument! Only valid classes are: 0, 1, 2, 3"),
        }
    }
}
