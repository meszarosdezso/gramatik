pub mod alphabet;
pub mod automata;
pub mod gramatik;
pub mod language;
pub mod rules;

use automata::Automata;
use gramatik::Gramatik;

use crate::automata::State;

#[allow(non_snake_case)]
fn main() {
    let N = ['A', 'B', 'C', 'S'].into();
    let T = ['a', 'b', 'c'].into();

    let P = ruleset!(
        S -> _
        aaB -> aaab
    );

    let S = 'S';

    let grammar = Gramatik::new(N, T, P, S).unwrap();

    println!("1. {}", grammar.is_class(1));
    println!("2. {}", grammar.is_class(2));
    println!("3. {}", grammar.is_class(3));

    println!("{}", grammar.class());

    let automata = Automata::new(
        ["q0", "q1", "q2"],
        ['a', 'b', 'c'].into(),
        &|q, _a| State(q),
        "q0".into(),
        ["q1"],
    )
    .unwrap();

    println!("{}", automata.current_state());
}
