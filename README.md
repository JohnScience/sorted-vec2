# `sorted_vec`

> Create and maintain sorted vectors.

```
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
```
