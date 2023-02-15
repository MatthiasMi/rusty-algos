# Rusty data structures and algorithms
This repository contains [Rust](https://www.rust-lang.org)-implementations of a few data structures and algorithms on top of simpler types for educational purposes.

## Tests
To run the tests,
```sh
cargo test
```

## Data Structures
 * [Stack](./src/data.rs)
 * [Queue](./src/data.rs)

A memory-safe implementation of two common data-types in computer science,
i.e, `Queue` and `Stack`, based on Rust's contiguous growable array type `Vec`:
 - `new` constructors return an instance represented as an empty `Vec<T>`.
 - `len` returns the number of elements currently stored in the data-type.
 - `mty` returns a `bool` if the data-type is empty or not.
 - `push` resp. `enq` adds an item of type T to the mutable data-type and returns `true`.
 - `peek` returns a wrapped immutable reference `Some<&T>` to an element of type `T`,
from the correct end, depending on LIFO / FIFO order, if it is non-empty, else `None`.
 - `pop` resp. `deq` removes and returns a `Some<T>`,if non-empty, else `None`.

## Algorithms

### Graphs
 * [Breadth-First Search (BFS)](./src/graph.rs)
 * [Depth First Search (DFS)](./src/graph.rs)

`breadth_first_search` traverses the given `Graph` using an intermediate [`Queue`](#data-structures), while `depth_first_search` traverses the given `Graph` using an intermediate [`Stack`](#data-structures).

### Search
 * [Linear](./src/search.rs)
 * [Binary](./src/search.rs)

`binary_search` requires `PartialEq` for checking `<`, and returns index of `t` in (asc) sorted list `L = [L_1,...]`, else `None`.
`linear_search` requires `PartialEq` for checking `==` and returns the index of `t` in list, else `None`.

### Sorting
 * [InsertionSort](./src/sort.rs)
 * [MergeSort](./src/sort.rs)
 * [QuickSort](./src/sort.rs)

 `insertion_sort` sorts the list partly `L[0..p]` by swapping elements from unsorted parts to current correct position `p`.
 `merge_sort` divides in the middle, conquers the left, then the right list, and merges the result into a new list.
 `quick_sort` follows the "divide-and-conquer"-strategy, picking a pivot element `p` which splits the list in two. It then puts `p` at its correct position in `L`, swaps all smaller elements before, and all those greater to a position after `p`.


### Subarray maximum

 * [MaxSubarray](./src/max_subarray.rs)

`max_subarray` returns the `start`, maximum `sum` and `end` of a sliced `isize`-list to describe the subarray-boundaries which is maximal when summing along the contiguous slice. The algorithm runs in `O(n)`, where `n` is the length of the input-list.