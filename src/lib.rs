//! Sorted vectors.
//!
//! - `SortedVec` -- sorted from least to greatest
//! - `ReverseSortedVec` -- sorted from greatest to least
//!
//! The `partial` module provides sorted vectors of types that only implement
//! `PartialOrd` where comparison of incomparable elements results in runtime
//! panic.

#[cfg(feature = "serde")]
#[macro_use] extern crate serde;

pub mod partial;

/// Forward sorted vector
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone,Debug,PartialEq)]
pub struct SortedVec <T : Ord> {
  vec : Vec <T>
}

/// Reverse sorted vector
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone,Debug,PartialEq)]
pub struct ReverseSortedVec <T : Ord> {
  vec : Vec <T>
}

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
  ///
  /// If the element was already present, the order index is returned as an
  /// `Err`, otherwise it is returned with `Ok`.
  pub fn insert (&mut self, element : T) -> Result <usize, usize> {
    match &self.vec[..].binary_search (&element) {
      Ok  (insert_at) => {
        self.vec.insert (*insert_at, element);
        Err (*insert_at)
      }
      Err (insert_at) => {
        self.vec.insert (*insert_at, element);
        Ok  (*insert_at)
      }
    }
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
  pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    R : std::ops::RangeBounds <usize>
  {
    self.vec.drain (range)
  }
}
impl <T : Ord> Default for SortedVec <T> {
  fn default() -> Self {
    Self::new()
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

impl <T : Ord> ReverseSortedVec <T> {
  #[inline]
  pub fn new() -> Self {
    ReverseSortedVec { vec: Vec::new() }
  }
  #[inline]
  pub fn with_capacity (capacity : usize) -> Self {
    ReverseSortedVec { vec: Vec::with_capacity (capacity) }
  }
  /// Uses `sort_unstable_by()` to sort in place.
  #[inline]
  pub fn from_unsorted (mut vec : Vec <T>) -> Self {
    vec.sort_unstable_by (|x,y| x.cmp (y).reverse());
    ReverseSortedVec { vec }
  }
  /// Insert an element into (reverse) sorted position, returning the order
  /// index at which it was placed.
  ///
  /// If the element was already present, the order index is returned as an
  /// `Err`, otherwise it is returned with `Ok`.
  pub fn insert (&mut self, element : T) -> Result <usize, usize> {
    match &self.vec[..].binary_search_by (
      |other_element| other_element.cmp (&element).reverse()
    ) {
      Ok  (insert_at) => {
        self.vec.insert (*insert_at, element);
        Err (*insert_at)
      }
      Err (insert_at) => {
        self.vec.insert (*insert_at, element);
        Ok  (*insert_at)
      }
    }
  }
  #[inline]
  pub fn remove_item (&mut self, item : &T) -> Option <T> {
    match self.vec.binary_search_by (
      |other_item| other_item.cmp (&item).reverse()
    ) {
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
  pub fn binary_search (&self, x : &T) -> Result <usize, usize> {
    self.vec.binary_search_by (|y| y.cmp (&x).reverse())
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
  pub fn drain <R> (&mut self, range : R) -> std::vec::Drain <T> where
    R : std::ops::RangeBounds <usize>
  {
    self.vec.drain (range)
  }
}
impl <T : Ord> Default for ReverseSortedVec <T> {
  fn default() -> Self {
    Self::new()
  }
}
impl <T : Ord> std::ops::Deref for ReverseSortedVec <T> {
  type Target = Vec <T>;
  fn deref (&self) -> &Vec <T> {
    &self.vec
  }
}
impl <T : Ord> Extend <T> for ReverseSortedVec <T> {
  fn extend <I : IntoIterator <Item = T>> (&mut self, iter : I) {
    for t in iter {
      let _ = self.insert (t);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sorted_vec() {
    let mut v = SortedVec::new();
    assert_eq!(v.insert (5), Ok (0));
    assert_eq!(v.insert (3), Ok (0));
    assert_eq!(v.insert (4), Ok (1));
    assert_eq!(v.insert (4), Err (1));
    assert_eq!(v.len(), 4);
    v.dedup();
    assert_eq!(v.len(), 3);
    assert_eq!(v.binary_search (&3), Ok (0));
    assert_eq!(*SortedVec::from_unsorted (
      vec![5, -10, 99, -11, 2, 17, 10]),
      vec![-11, -10, 2, 5, 10, 17, 99]);
    let mut v = SortedVec::new();
    v.extend(vec![5, -10, 99, -11, 2, 17, 10].into_iter());
    assert_eq!(
      v.drain(..).collect::<Vec <i32>>(), vec![-11, -10, 2, 5, 10, 17, 99]);
  }

  #[test]
  fn test_reverse_sorted_vec() {
    let mut v = ReverseSortedVec::new();
    assert_eq!(v.insert (5), Ok (0));
    assert_eq!(v.insert (3), Ok (1));
    assert_eq!(v.insert (4), Ok (1));
    assert_eq!(v.insert (6), Ok (0));
    assert_eq!(v.insert (4), Err (2));
    assert_eq!(v.len(), 5);
    v.dedup();
    assert_eq!(v.len(), 4);
    assert_eq!(v.binary_search (&3), Ok (3));
    assert_eq!(*ReverseSortedVec::from_unsorted (
      vec![5, -10, 99, -11, 2, 17, 10]),
      vec![99, 17, 10, 5, 2, -10, -11]);
    let mut v = ReverseSortedVec::new();
    v.extend(vec![5, -10, 99, -11, 2, 17, 10].into_iter());
    assert_eq!(
      v.drain(..).collect::<Vec <i32>>(), vec![99, 17, 10, 5, 2, -10, -11]);
  }
}
