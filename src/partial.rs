use {std};

#[derive(Debug,PartialEq)]
pub struct SortedVec <T : PartialOrd> {
  vec : Vec <T>
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
  #[inline]
  pub fn as_slice (&self) -> &[T] {
    self.vec.as_slice()
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
      |other_element| Self::partial_compare (other_element, &element)
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
      |other_item| Self::partial_compare (other_item, item)
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
  #[inline]
  pub fn len (&self) -> usize {
    self.vec.len()
  }
  #[inline]
  pub fn is_empty (&self) -> bool {
    self.vec.is_empty()
  }
  #[inline]
  pub fn dedup (&mut self) {
    self.vec.dedup();
  }
  //
  //  private methods
  //
  fn partial_compare (lhs : &T, rhs : &T) -> std::cmp::Ordering {
    lhs.partial_cmp (rhs).unwrap()
  }
}

impl <T : PartialOrd> Default for SortedVec <T> {
  fn default() -> Self {
    Self::new()
  }
}

impl <T : PartialOrd> AsRef <Vec <T>> for SortedVec <T> {
  fn as_ref (&self) -> &Vec <T> {
    &self.vec
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
  }
}
