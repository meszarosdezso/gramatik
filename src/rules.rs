use std::collections::HashSet;

use crate::alphabet::Word;

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

#[derive(Default)]
pub struct RuleSet {
    pub rules: HashSet<Rule>
}

impl RuleSet {
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
     (
        $($a:ident -> $b:pat_param$(| $c:pat_param)*$(,)?)*
     ) => {
        {
            let mut ruleset = rules::RuleSet::default();

            $(
                let start = stringify!($a).to_owned();
                let mut target = stringify!($b).to_owned();
                
                if target == "_" {
                    target = alphabet::EPSILON.to_string();
                }
                
                ruleset.add_rule(rules::Rule::new(start, target));
    
                ruleset = ruleset $(+ ruleset!($a -> $c))*;
            )*
            
            ruleset
        }
    };
}
