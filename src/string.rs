#[cfg(feature = "z_function")]
mod z_function;
#[cfg(feature = "z_function")]
pub use z_function::z_function;

#[cfg(feature = "prefix_function")]
mod prefix_function;
#[cfg(feature = "prefix_function")]
pub use prefix_function::prefix_function;

#[cfg(feature = "kmp")]
mod kmp;
#[cfg(feature = "kmp")]
pub use kmp::kmp;

#[cfg(feature = "trie")]
pub mod trie;

#[cfg(feature = "suffix_automaton")]
pub mod suffix_automaton;
