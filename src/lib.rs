//! Sorted vectors.
//!
//! [Repository](https://gitlab.com/spearman/sorted-vec)
//!
//! - `SortedVec` -- sorted from least to greatest, may contain duplicates
//! - `SortedSet` -- sorted from least to greatest, unique elements
//! - `ReverseSortedVec` -- sorted from greatest to least, may contain
//!   duplicates
//! - `ReverseSortedSet` -- sorted from greatest to least, unique elements
//!
//! The `partial` module provides sorted vectors of types that only implement
//! `PartialOrd` where comparison of incomparable elements results in runtime
//! panic.

#![cfg_attr(feature = "serde", feature(is_sorted))]

#[cfg(feature = "serde")]
#[macro_use] extern crate serde;

use std::hash::{Hash, Hasher};

pub mod partial;

/// Forward sorted vector
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", not(feature = "serde-nontransparent")),
  serde(transparent))]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SortedVec <T : Ord> {
  #[cfg_attr(feature = "serde", serde(deserialize_with = "parse_vec"))]
  #[cfg_attr(feature = "serde",
    serde(bound(deserialize = "T : serde::Deserialize <'de>")))]
  vec : Vec <T>
}

#[cfg(feature = "serde")]
fn parse_vec <'de, D, T> (deserializer : D) -> Result <Vec <T>, D::Error> where
  D : serde::Deserializer <'de>,
  T : Ord + serde::Deserialize <'de>
{
  use serde::Deserialize;
  use serde::de::Error;
  let v = Vec::deserialize (deserializer)?;
  if !v.is_sorted() {
    Err (D::Error::custom ("input sequence is not sorted"))
  } else {
    Ok (v)
  }
}

/// Forward sorted set
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "serde", not(feature = "serde-nontransparent")),
  serde(transparent))]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SortedSet <T : Ord> {
  set : SortedVec <T>
}

#[cfg(feature = "serde")]
fn parse_reverse_vec <'de, D, T> (deserializer : D) -> Result <Vec <T>, D::Error> where
  D : serde::Deserializer <'de>,
  T : Ord + serde::Deserialize <'de>
{
  use serde::Deserialize;
  use serde::de::Error;
  let v = Vec::<T>::deserialize (deserializer)?;
  if !v.is_sorted_by (|x,y| Some (x.cmp (y).reverse())) {
    Err (D::Error::custom ("input sequence is not reverse sorted"))
  } else {
    Ok (v)
  }
}

/// Value returned when find_or_insert is used.
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum FindOrInsert {
  /// Contains a found index
  Found(usize),

  /// Contains an inserted index
  Inserted(usize),
}

/// Converts from the binary_search result type into the FindOrInsert type
impl From<Result<usize, usize>> for FindOrInsert {
  fn from(result: Result<usize, usize>) -> Self {
    match result {
      Result::Ok(value) => FindOrInsert::Found(value),
      Result::Err(value) => FindOrInsert::Inserted(value),
    }
  }
}

impl FindOrInsert {

  /// Get the index of the element that was either found or inserted.
  pub fn index(&self) -> usize {
    match self {
      FindOrInsert::Found(value) | FindOrInsert::Inserted(value) => *value
    }
  }

  /// If an equivalent element was found in the container, get the value of
  /// its index. Otherwise get None.
  pub fn found(&self) -> Option<usize> {
    match self {
      FindOrInsert::Found(value) => Some(*value),
      FindOrInsert::Inserted(_) => None
    }
  }

  /// If the provided element was inserted into the container, get the value
  /// of its index. Otherwise get None.
  pub fn inserted(&self) -> Option<usize> {
    match self {
      FindOrInsert::Found(_) => None,
      FindOrInsert::Inserted(value) => Some(*value)
    }
  }

  /// Returns true if the element was found.
  pub fn is_found(&self) -> bool {
    return matches!(self, FindOrInsert::Found(_));
  }

  /// Returns true if the element was inserted.
  pub fn is_inserted(&self) -> bool {
    return matches!(self, FindOrInsert::Inserted(_));
  }

  pub fn map<T, F: FnOnce(usize) -> T>(&self, if_found: F, if_inserted:F) -> T {
    match self {
      FindOrInsert::Found(value) => if_found(*value),
      FindOrInsert::Inserted(value) => if_inserted(*value)
    }
  }
}

//
//  impl SortedVec
//

impl <T : Ord> SortedVec <T> {
  #[inline]
  pub fn new() -> Self {
    SortedVec { vec: Vec::new() }
  }
  #[inline]
  pub fn with_capacity (capacity : usize) -> Self {
    SortedVec { vec: Vec::with_capacity (capacity) }
  }
  /// Uses `sort_unstable()` to sort in place.
  #[inline]
  pub fn from_unsorted (mut vec : Vec <T>) -> Self {
    vec.sort_unstable();
    SortedVec { vec }
  }
  /// Insert an element into sorted position, returning the order index at which
  /// it was placed.
  pub fn insert (&mut self, element : T) -> usize {
    let insert_at = match self.binary_search (&element) {
      Ok (insert_at) | Err (insert_at) => insert_at
    };
    self.vec.insert (insert_at, element);
    insert_at
  }
  /// Find the element and return the index with `Ok`, otherwise insert the
  /// element and return the new element index with `Err`.
  pub fn find_or_insert (&mut self, element : T) -> FindOrInsert {
    self.binary_search (&element).map_err (|insert_at| {
      self.vec.insert (insert_at, element);
      insert_at
    }).into()
  }
  #[inline]
  pub fn remove_item (&mut self, item : &T) -> Option <T> {
    match self.vec.binary_search (item) {
      Ok  (remove_at) => Some (self.vec.remove (remove_at)),
      Err (_)         => None
    }
  }
  /// Panics if index is out of bounds
  #[inline]
  pub fn remove_index (&mut self, index : usize) -> T {
    self.vec.remove (index)
  }
  #[inline]
  pub fn pop (&mut self) -> Option <T> {
    self.vec.pop()
  }
  #[inline]
  pub fn clear (&mut self) {
    self.vec.clear()
  }
  #[inline]
  pub fn dedup (&mut self) {
    self.vec.dedup();
  }
  #[inline]
  pub fn dedup_by_key <F, K> (&mut self, key : F) where
    F : FnMut (&mut T) -> K,
    K : PartialEq <K>
  {
    self.vec.dedup_by_key (key);
  }
  #[inline]
  pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    R : std::ops::RangeBounds <usize>
  {
    self.vec.drain (range)
  }
  #[inline]
  pub fn retain <F> (&mut self, f : F) where F : FnMut (&T) -> bool {
    self.vec.retain (f)
  }
  /// NOTE: to_vec() is a slice method that is accessible through deref, use
  /// this instead to avoid cloning
  #[inline]
  pub fn into_vec (self) -> Vec <T> {
    self.vec
  }
  /// Apply a closure mutating the sorted vector and use `sort_unstable()`
  /// to re-sort the mutated vector
  pub fn mutate_vec <F, O> (&mut self, f : F) -> O where
    F : FnOnce (&mut Vec <T>) -> O
  {
    let res = f (&mut self.vec);
    self.vec.sort_unstable();
    res
  }
}
impl <T : Ord> Default for SortedVec <T> {
  fn default() -> Self {
    Self::new()
  }
}
impl <T : Ord> From <Vec <T>> for SortedVec <T> {
  fn from (unsorted : Vec <T>) -> Self {
    Self::from_unsorted (unsorted)
  }
}
impl <T : Ord> std::ops::Deref for SortedVec <T> {
  type Target = Vec <T>;
  fn deref (&self) -> &Vec <T> {
    &self.vec
  }
}
impl <T : Ord> Extend <T> for SortedVec <T> {
  fn extend <I : IntoIterator <Item = T>> (&mut self, iter : I) {
    for t in iter {
      let _ = self.insert (t);
    }
  }
}
impl <T : Ord + Hash> Hash for SortedVec <T> {
  fn hash <H : Hasher> (&self, state : &mut H) {
    let v : &Vec <T> = self.as_ref();
    v.hash (state);
  }
}

//
//  impl SortedSet
//

impl <T : Ord> SortedSet <T> {
  #[inline]
  pub fn new() -> Self {
    SortedSet { set: SortedVec::new() }
  }
  #[inline]
  pub fn with_capacity (capacity : usize) -> Self {
    SortedSet { set: SortedVec::with_capacity (capacity) }
  }
  /// Uses `sort_unstable()` to sort in place and `dedup()` to remove
  /// duplicates.
  #[inline]
  pub fn from_unsorted (vec : Vec <T>) -> Self {
    let mut set = SortedVec::from_unsorted (vec);
    set.dedup();
    SortedSet { set }
  }
  /// Insert an element into sorted position, returning the order index at which
  /// it was placed.
  #[inline]
  pub fn insert (&mut self, element : T) -> usize {
    match self.set.vec.binary_search(&element) {
      Ok (existing_index) => {
        unsafe {
          // If binary_search worked correctly, then this must be the index of a
          // valid element to get from the vector.
          *self.set.vec.get_unchecked_mut(existing_index) = element;
        }
        existing_index
      },
      Err (insert_index) => {
        self.set.vec.insert(insert_index, element);
        insert_index
      }
    }
  }
  /// Find the element and return the index with `Ok`, otherwise insert the
  /// element and return the new element index with `Err`.
  #[inline]
  pub fn find_or_insert (&mut self, element : T) -> FindOrInsert {
    self.set.find_or_insert (element).into()
  }
  #[inline]
  pub fn remove_item (&mut self, item : &T) -> Option <T> {
    self.set.remove_item (item)
  }
  /// Panics if index is out of bounds
  #[inline]
  pub fn remove_index (&mut self, index : usize) -> T {
    self.set.remove_index (index)
  }
  #[inline]
  pub fn pop (&mut self) -> Option <T> {
    self.set.pop()
  }
  #[inline]
  pub fn clear (&mut self) {
    self.set.clear()
  }
  #[inline]
  pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    R : std::ops::RangeBounds <usize>
  {
    self.set.drain (range)
  }
  #[inline]
  pub fn retain <F> (&mut self, f : F) where F : FnMut (&T) -> bool {
    self.set.retain (f)
  }
  /// NOTE: to_vec() is a slice method that is accessible through deref, use
  /// this instead to avoid cloning
  #[inline]
  pub fn into_vec (self) -> Vec <T> {
    self.set.into_vec()
  }
  /// Apply a closure mutating the sorted vector and use `sort_unstable()`
  /// to re-sort the mutated vector and `dedup()` to remove any duplicate
  /// values
  pub fn mutate_vec <F, O> (&mut self, f : F) -> O where
    F : FnOnce (&mut Vec <T>) -> O
  {
    let res = self.set.mutate_vec (f);
    self.set.dedup();
    res
  }
}
impl <T : Ord> Default for SortedSet <T> {
  fn default() -> Self {
    Self::new()
  }
}
impl <T : Ord> From <Vec <T>> for SortedSet <T> {
  fn from (unsorted : Vec <T>) -> Self {
    Self::from_unsorted (unsorted)
  }
}
impl <T : Ord> std::ops::Deref for SortedSet <T> {
  type Target = SortedVec <T>;
  fn deref (&self) -> &SortedVec <T> {
    &self.set
  }
}
impl <T : Ord> Extend <T> for SortedSet <T> {
  fn extend <I : IntoIterator <Item = T>> (&mut self, iter : I) {
    for t in iter {
      let _ = self.insert (t);
    }
  }
}
impl <T : Ord + Hash> Hash for SortedSet <T> {
  fn hash <H : Hasher> (&self, state : &mut H) {
    let v : &Vec <T> = self.as_ref();
    v.hash (state);
  }
}

//
//  Reverse Containers
//
pub type ReverseSortedVec<T> = SortedVec<std::cmp::Reverse<T>>;
pub type ReverseSortedSet<T> = SortedSet<std::cmp::Reverse<T>>;

#[cfg(test)]
mod tests {
  use super::*;
  use std::cmp::Reverse;

  #[test]
  fn test_sorted_vec() {
    let mut v = SortedVec::new();
    assert_eq!(v.insert (5), 0);
    assert_eq!(v.insert (3), 0);
    assert_eq!(v.insert (4), 1);
    assert_eq!(v.insert (4), 1);
    assert_eq!(v.find_or_insert (4), FindOrInsert::Found (2));
    assert_eq!(v.find_or_insert (4).index(), 2);
    assert_eq!(v.len(), 4);
    v.dedup();
    assert_eq!(v.len(), 3);
    assert_eq!(v.binary_search (&3), Ok (0));
    assert_eq!(*SortedVec::from_unsorted (
      vec![5, -10, 99, -11, 2, 17, 10]),
      vec![-11, -10, 2, 5, 10, 17, 99]);
    assert_eq!(SortedVec::from_unsorted (
      vec![5, -10, 99, -11, 2, 17, 10]),
      vec![5, -10, 99, -11, 2, 17, 10].into());
    let mut v = SortedVec::new();
    v.extend(vec![5, -10, 99, -11, 2, 17, 10].into_iter());
    assert_eq!(*v, vec![-11, -10, 2, 5, 10, 17, 99]);
    let _ = v.mutate_vec (|v|{
      v[0] = 11;
      v[3] = 1;
    });
    assert_eq!(
      v.drain(..).collect::<Vec <i32>>(),
      vec![-10, 1, 2, 10, 11, 17, 99]);
  }

  #[test]
  fn test_sorted_set() {
    let mut s = SortedSet::new();
    assert_eq!(s.insert (5), 0);
    assert_eq!(s.insert (3), 0);
    assert_eq!(s.insert (4), 1);
    assert_eq!(s.insert (4), 1);
    assert_eq!(s.find_or_insert (4), FindOrInsert::Found (1));
    assert_eq!(s.find_or_insert (4).index(), 1);
    assert_eq!(s.len(), 3);
    assert_eq!(s.binary_search (&3), Ok (0));
    assert_eq!(**SortedSet::from_unsorted (
      vec![5, -10, 99, -10, -11, 10, 2, 17, 10]),
      vec![-11, -10, 2, 5, 10, 17, 99]);
    assert_eq!(SortedSet::from_unsorted (
      vec![5, -10, 99, -10, -11, 10, 2, 17, 10]),
      vec![5, -10, 99, -10, -11, 10, 2, 17, 10].into());
    let mut s = SortedSet::new();
    s.extend(vec![5, -11, -10, 99, -11, 2, 17, 2, 10].into_iter());
    assert_eq!(**s, vec![-11, -10, 2, 5, 10, 17, 99]);
    let _ = s.mutate_vec (|s|{
      s[0] = 5;
      s[3] = 1;
    });
    assert_eq!(
      s.drain(..).collect::<Vec <i32>>(),
      vec![-10, 1, 2, 5, 10, 17, 99]);
  }

  #[test]
  fn test_reverse_sorted_vec() {
    let mut v = ReverseSortedVec::new();
    assert_eq!(v.insert (Reverse(5)), 0);
    assert_eq!(v.insert (Reverse(3)), 1);
    assert_eq!(v.insert (Reverse(4)), 1);
    assert_eq!(v.find_or_insert (Reverse(6)), FindOrInsert::Inserted (0));
    assert_eq!(v.insert (Reverse(4)), 2);
    assert_eq!(v.find_or_insert (Reverse(4)), FindOrInsert::Found (2));
    assert_eq!(v.len(), 5);
    v.dedup();
    assert_eq!(v.len(), 4);
    assert_eq!(*ReverseSortedVec::from_unsorted (
      vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(17), Reverse(10)]),
      vec![Reverse(99), Reverse(17), Reverse(10), Reverse(5), Reverse(2), Reverse(-10), Reverse(-11)]);
    assert_eq!(ReverseSortedVec::from_unsorted (
      vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(17), Reverse(10)]),
      vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(17), Reverse(10)].into());
    let mut v = ReverseSortedVec::new();
    v.extend(vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(17), Reverse(10)].into_iter());
    assert_eq!(*v, vec![Reverse(99), Reverse(17), Reverse(10), Reverse(5), Reverse(2), Reverse(-10), Reverse(-11)]);
    let _ = v.mutate_vec (|v|{
      v[6] = Reverse(11);
      v[3] = Reverse(1);
    });
    assert_eq!(
      v.drain(..).collect::<Vec <Reverse<i32>>>(),
      vec![Reverse(99), Reverse(17), Reverse(11), Reverse(10), Reverse(2), Reverse(1), Reverse(-10)]);
  }

  #[test]
  fn test_reverse_sorted_set() {
    let mut s = ReverseSortedSet::new();
    assert_eq!(s.insert (Reverse(5)), 0);
    assert_eq!(s.insert (Reverse(3)), 1);
    assert_eq!(s.insert (Reverse(4)), 1);
    assert_eq!(s.find_or_insert (Reverse(6)), FindOrInsert::Inserted (0));
    assert_eq!(s.insert (Reverse(4)), 2);
    assert_eq!(s.find_or_insert (Reverse(4)), FindOrInsert::Found (2));
    assert_eq!(s.len(), 4);
    assert_eq!(s.binary_search (&Reverse(3)), Ok (3));
    assert_eq!(**ReverseSortedSet::from_unsorted (
      vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(99), Reverse(17), Reverse(10), Reverse(-10)]),
      vec![Reverse(99), Reverse(17), Reverse(10), Reverse(5), Reverse(2), Reverse(-10), Reverse(-11)]);
    assert_eq!(ReverseSortedSet::from_unsorted (
      vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(99), Reverse(17), Reverse(10), Reverse(-10)]),
      vec![Reverse(5), Reverse(-10), Reverse(99), Reverse(-11), Reverse(2), Reverse(99), Reverse(17), Reverse(10), Reverse(-10)].into());
    let mut s = ReverseSortedSet::new();
    s.extend(vec![Reverse(5), Reverse(-10), Reverse(2), Reverse(99), Reverse(-11), Reverse(-11), Reverse(2), Reverse(17), Reverse(10)].into_iter());
    assert_eq!(**s, vec![Reverse(99), Reverse(17), Reverse(10), Reverse(5), Reverse(2), Reverse(-10), Reverse(-11)]);
    let _ = s.mutate_vec (|s|{
      s[6] = Reverse(17);
      s[3] = Reverse(1);
    });
    assert_eq!(
      s.drain(..).collect::<Vec <Reverse<i32>>>(),
      vec![Reverse(99), Reverse(17), Reverse(10), Reverse(2), Reverse(1), Reverse(-10)]);
  }
  #[cfg(feature = "serde-nontransparent")]
  #[test]
  fn test_deserialize() {
    let s = r#"{"vec":[-11,-10,2,5,10,17,99]}"#;
    let _ = serde_json::from_str::<SortedVec <i32>> (s).unwrap();
  }
  #[cfg(all(feature = "serde", not(feature = "serde-nontransparent")))]
  #[test]
  fn test_deserialize() {
    let s = "[-11,-10,2,5,10,17,99]";
    let _ = serde_json::from_str::<SortedVec <i32>> (s).unwrap();
  }
  #[cfg(feature = "serde-nontransparent")]
  #[test]
  #[should_panic]
  fn test_deserialize_unsorted() {
    let s = r#"{"vec":[99,-11,-10,2,5,10,17]}"#;
    let _ = serde_json::from_str::<SortedVec <i32>> (s).unwrap();
  }
  #[cfg(all(feature = "serde", not(feature = "serde-nontransparent")))]
  #[test]
  #[should_panic]
  fn test_deserialize_unsorted() {
    let s = "[99,-11,-10,2,5,10,17]";
    let _ = serde_json::from_str::<SortedVec <i32>> (s).unwrap();
  }
  #[cfg(feature = "serde-nontransparent")]
  #[test]
  fn test_deserialize_reverse() {
    let s = r#"{"vec":[99,17,10,5,2,-10,-11]}"#;
    let _ = serde_json::from_str::<ReverseSortedVec <i32>> (s).unwrap();
  }
  #[cfg(all(feature = "serde", not(feature = "serde-nontransparent")))]
  #[test]
  fn test_deserialize_reverse() {
    let s = "[99,17,10,5,2,-10,-11]";
    let _ = serde_json::from_str::<ReverseSortedVec <i32>> (s).unwrap();
  }
  #[cfg(feature = "serde-nontransparent")]
  #[test]
  #[should_panic]
  fn test_deserialize_reverse_unsorted() {
    let s = r#"{vec:[99,-11,-10,2,5,10,17]}"#;
    let _ = serde_json::from_str::<ReverseSortedVec <i32>> (s).unwrap();
  }
  #[cfg(all(feature = "serde", not(feature = "serde-nontransparent")))]
  #[test]
  #[should_panic]
  fn test_deserialize_reverse_unsorted() {
    let s = "[99,-11,-10,2,5,10,17]";
    let _ = serde_json::from_str::<ReverseSortedVec <i32>> (s).unwrap();
  }
}
