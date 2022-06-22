pub mod alphabet;
pub mod gramatik;
pub mod rules;

use gramatik::Gramatik;

#[allow(non_snake_case)]
fn main() {
   let N = ['A', 'B', 'C', 'S'].into();
   let T = ['a', 'b', 'c'].into();
   
   let P = ruleset!(
       S -> _
       A -> abc | aB
       B -> b
   );
   
   let S = 'S';

   let grammar = Gramatik(N, T, P, S);

   println!("{}", grammar.is_class(2)); // true
   println!("{}", grammar.class()); // 3
}
