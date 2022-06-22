use crate::alphabet::Alphabet;
use crate::alphabet::EPSILON;
use crate::alphabet::Letter;
use crate::rules::Rule;
use crate::rules::RuleSet;


/// Represents a G<N, T, P, S> grammar.
///   N: Alphabet of non terminal symbols
///   T: Alphabet of terminal symbols
///   P: The grammar rules
///   S: Start symbol
pub struct Gramatik(pub Alphabet, pub Alphabet, pub RuleSet, pub Letter);

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
            if rule.start.len() != 1   {
                return false;
            }

            let start =  rule.start.chars().nth(0).unwrap();
            if !self.0.contains(start) && start != self.3 {
                return false;
            }
            true
        };

        match class {
            // aaaAbbb => aaaBbbb y
            // aaaAbbb => aaabbb x
            1 => {
                let mut kes = false;
                for rule in self.2.rules.iter() {
                    if rule.end.contains(self.3) {
                        if kes {
                            return false;
                        }
                        kes = true;
                    }
                }
                true
            },
            2 => {
                self.2.rules.iter().all(check_start)
            }
            3 => {
                for rule in self.2.rules.iter() {
                    if !check_start(rule) {
                        return false
                    }

                    for (i, c) in rule.end.chars().enumerate() {
                        if c != EPSILON && !self.1.contains(c) {
                            if i != rule.end.len() - 1 {
                                return false
                            }
                        }
                    }
                }
                true
            }
            _ => panic!("Invalid class argument! Only valid classes are: 0, 1, 2, 3")
        }
    }
}