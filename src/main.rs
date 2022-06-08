pub mod alphabet;
pub mod gramatik;

use alphabet::Alphabet;

fn main() {
   let alph = Alphabet::from_iter(['a', 'b', 'c'].into_iter()); 
   let other = Alphabet::from_iter(['c', 'd', 'e'].into_iter());
   
   println!("{}", alph + other);
}
