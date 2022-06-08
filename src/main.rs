pub mod alphabet;
pub mod gramatik;

use alphabet::Alphabet;
use gramatik::Gramatik;

#[allow(non_snake_case)]
fn main() {
   let N = Alphabet::from_iter(['C', 'S', 'A'].into_iter());
   let T = Alphabet::from_iter(['a', 'b', 'c'].into_iter()); 
   
   let P = ruleset!(
       A -> a | _,
       C -> c,
       S -> _,
       AC -> CA
   );
   
   let S = 'S';

   println!("{P}");

   let _grammar = Gramatik(N, T, P, S);
}
