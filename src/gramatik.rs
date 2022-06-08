use crate::alphabet::{Alphabet, Letter};

pub type RuleSet = ();

/// Represents a G<N, T, P, S> grammar.
///   N: Alphabet of non terminal symbols
///   T: Aplhabet of terminal symbols
///   P: The grammar rules
///   S: Start symbol
pub struct Gramatik(Alphabet, Alphabet, RuleSet, Letter);

