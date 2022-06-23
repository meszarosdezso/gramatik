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

    println!("Grammar class: {}", grammar.class());

    let delta = delta_fn!(
        (q0, a) -> q1
        (q1, a) -> q0
        (q1, b) -> q2
    );

    let automata = Automata::new(
        ["q0", "q1", "q2"],
        ['a', 'b', 'c'].into(),
        &delta,
        "q0".into(),
        ["q2"],
    )
    .unwrap();

    println!("Current state: {}", automata.current_state());

    let word = "aaab";
    println!(
        "After processing '{word}', the automata ended in {} state.",
        automata.process_word(word)
    );
}
