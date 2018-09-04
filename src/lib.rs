pub mod partial;

#[derive(Clone,Debug,PartialEq)]
pub struct SortedVec <T : Ord> {
  vec : Vec <T>
}

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
  pub fn dedup (&mut self) {
    self.vec.dedup();
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

impl <T : Ord> ReverseSortedVec <T> {
  #[inline]
  pub fn new() -> Self {
    ReverseSortedVec { vec: Vec::new() }
  }
  #[inline]
  pub fn with_capacity (capacity : usize) -> Self {
    ReverseSortedVec { vec: Vec::with_capacity (capacity) }
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
  }
}
