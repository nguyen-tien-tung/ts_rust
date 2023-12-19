// fn main() {
//   let array: Vec<isize> = vec![1,2,3].iter().map(|x| x+1).collect();
//   print!("{:?}", array);
// }

use std::collections::{HashSet, HashMap};

fn main(){
  let data = vec![1,2,3];
  let mut vector = data.iter().map(|x: &i32| x+1);
  let mut new_vector = vec![];

  while let Some(x) = vector.next() {
      new_vector.push(x);
  }

  println!("{:?}", new_vector);

  let foo: String = vec!["I", "am", "a", "string"].into_iter().collect();
  println!("{0}", foo);
  
  let hs: HashSet<isize> = vec![1,2,3].into_iter().collect();
  println!("{:?}", hs);

  vec![1,2,5,9,4].into_iter().skip(2).take_while(|&x| x > 4).for_each(|x| println!("{}", x));

  let what_about_this = vec![1,2,3].iter().filter(|&x| x % 2 == 0).count();

  let hm: HashMap<&str, usize> = vec!["this", "is", "a", "test"].into_iter().enumerate().map(|(index, item)| (item, index)).collect();
  println!("{:?}", what_about_this);
  println!("{:?}", hm);
}