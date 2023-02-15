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

### Sorting
 * [InsertionSort](./src/sort.rs)
 * [MergeSort](./src/sort.rs)
 * [QuickSort](./src/sort.rs)

### Graphs
 * [Breadth-First Search (BFS)](./src/graph.rs)
 * [Depth First Search (DFS)](./src/graph.rs)

### Search
 * [Linear](./src/search.rs)
 * [Binary](./src/search.rs)