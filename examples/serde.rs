use sorted_vec::*;
use serde_json;

fn main() {
  let v = SortedVec::from_unsorted (vec![5, -10, 99, -11, 2, 17, 10]);
  println!("{}", serde_json::to_string (&v).unwrap());
}
