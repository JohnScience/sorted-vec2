//! Sorted vectors of types implementing `PartialOrd`.
//!
//! It is a runtime panic if an incomparable element is compared.

use {std};

/// Forward sorted vector
#[derive(Clone,Debug,PartialEq)]
pub struct SortedVec <T : PartialOrd> {
  vec : Vec <T>
}

/// Reverse sorted vector
#[derive(Clone,Debug,PartialEq)]
pub struct ReverseSortedVec <T : PartialOrd> {
  vec : Vec <T>
}

/// Unwraps a `partial_cmp`
fn partial_compare <T : PartialOrd> (lhs : &T, rhs : &T) -> std::cmp::Ordering {
  lhs.partial_cmp (rhs).unwrap()
}

impl <T : PartialOrd> SortedVec <T> {
  #[inline]
  pub fn new() -> Self {
    SortedVec { vec: Vec::new() }
  }
  #[inline]
  pub fn with_capacity (capacity : usize) -> Self {
    SortedVec { vec: Vec::with_capacity (capacity) }
  }
  /// Uses `sort_unstable_by()` to sort in place.
  #[inline]
  pub fn from_unsorted (mut vec : Vec <T>) -> Self {
    vec.sort_unstable_by (partial_compare);
    SortedVec { vec }
  }
  /// Insert an element into sorted position, returning the order index at which
  /// it was placed.
  ///
  /// If the element was already present, the order index is returned as an
  /// `Err`, otherwise it is returned with `Ok`.
  ///
  /// Partial order comparison panics if items are not comparable.
  pub fn insert (&mut self, element : T) -> Result <usize, usize> {
    match &self.vec[..].binary_search_by (
      |other_element| partial_compare (other_element, &element)
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
      |other_item| partial_compare (other_item, item)
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
    self.vec.binary_search_by (|y| partial_compare (y, x))
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
impl <T : PartialOrd> Default for SortedVec <T> {
  fn default() -> Self {
    Self::new()
  }
}
impl <T : PartialOrd> std::ops::Deref for SortedVec <T> {
  type Target = Vec <T>;
  fn deref (&self) -> &Vec <T> {
    &self.vec
  }
}
impl <T : PartialOrd> Extend <T> for SortedVec <T> {
  fn extend <I : IntoIterator <Item = T>> (&mut self, iter : I) {
    for t in iter {
      let _ = self.insert (t);
    }
  }
}

impl <T : PartialOrd> ReverseSortedVec <T> {
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
    vec.sort_unstable_by (|x,y| partial_compare (x,y).reverse());
    ReverseSortedVec { vec }
  }
  /// Insert an element into (reverse) sorted position, returning the order
  /// index at which it was placed.
  ///
  /// If the element was already present, the order index is returned as an
  /// `Err`, otherwise it is returned with `Ok`.
  pub fn insert (&mut self, element : T) -> Result <usize, usize> {
    match &self.vec[..].binary_search_by (
      |other_element| partial_compare (other_element, &element).reverse()
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
      |other_item| partial_compare (other_item, item).reverse()
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
    self.vec.binary_search_by (|y| partial_compare (y, x).reverse())
  }
  #[inline]
  pub fn pop (&mut self) -> Option <T> {
    self.vec.pop()
  }
  #[inline]
  pub fn clear (&mut self) {
    self.vec.clear()
  }
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
impl <T : PartialOrd> Default for ReverseSortedVec <T> {
  fn default() -> Self {
    Self::new()
  }
}
impl <T : PartialOrd> std::ops::Deref for ReverseSortedVec <T> {
  type Target = Vec <T>;
  fn deref (&self) -> &Vec <T> {
    &self.vec
  }
}
impl <T : PartialOrd> Extend <T> for ReverseSortedVec <T> {
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
    assert_eq!(v.insert (5.0), Ok (0));
    assert_eq!(v.insert (3.0), Ok (0));
    assert_eq!(v.insert (4.0), Ok (1));
    assert_eq!(v.insert (4.0), Err (1));
    assert_eq!(v.len(), 4);
    v.dedup();
    assert_eq!(v.len(), 3);
    assert_eq!(v.binary_search (&3.0), Ok (0));
    assert_eq!(*SortedVec::from_unsorted (
      vec![  5.0, -10.0, 99.0, -11.0,  2.0, 17.0, 10.0]),
      vec![-11.0, -10.0,  2.0,   5.0, 10.0, 17.0, 99.0]);
    let mut v = SortedVec::new();
    v.extend(vec![5.0, -10.0, 99.0, -11.0, 2.0, 17.0, 10.0].into_iter());
    assert_eq!(
      v.drain(..).collect::<Vec <f32>>(),
      vec![-11.0, -10.0, 2.0, 5.0, 10.0, 17.0, 99.0]);
  }

  #[test]
  fn test_reverse_sorted_vec() {
    let mut v = ReverseSortedVec::new();
    assert_eq!(v.insert (5.0), Ok (0));
    assert_eq!(v.insert (3.0), Ok (1));
    assert_eq!(v.insert (4.0), Ok (1));
    assert_eq!(v.insert (6.0), Ok (0));
    assert_eq!(v.insert (4.0), Err (2));
    assert_eq!(v.len(), 5);
    v.dedup();
    assert_eq!(v.len(), 4);
    assert_eq!(v.binary_search (&3.0), Ok (3));
    assert_eq!(*ReverseSortedVec::from_unsorted (
      vec![5.0, -10.0, 99.0, -11.0, 2.0,  17.0,  10.0]),
      vec![99.0, 17.0, 10.0,   5.0, 2.0, -10.0, -11.0]);
    let mut v = ReverseSortedVec::new();
    v.extend(vec![5.0, -10.0, 99.0, -11.0, 2.0, 17.0, 10.0].into_iter());
    assert_eq!(
      v.drain(..).collect::<Vec <f32>>(),
      vec![99.0, 17.0, 10.0, 5.0, 2.0, -10.0, -11.0]);
  }
}
