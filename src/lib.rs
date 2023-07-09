#![allow(dead_code)]

#[cfg(any(
    feature = "string",
    feature = "z_function",
    feature = "prefix_function",
    feature = "kmp",
    feature = "trie",
    feature = "suffix_automaton",
))]
pub mod string;

#[cfg(any(
    feature = "bst",
    feature = "segment_tree",
    feature = "fenwick_tree",
    feature = "binary_heap",
))]
pub mod bst;

#[cfg(any(
    feature = "sort",
    feature = "quick",
    feature = "merge",
    feature = "heap",
    feature = "radix",
    feature = "bubble",
))]
pub mod sort;
