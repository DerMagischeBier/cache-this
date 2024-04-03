//! Macro for simple caching of expressions using the local file system. The expression within
//! the `cache_this` macro will be executed only if no cached result is already locally present. If
//! no result is present yet it will be saved to the local file system and subsequently used.
//!
//! Results are even cached in between program execution enabling rapid prototyping by getting rid
//! of long execution times per iteration.
//!
//! Expression are cache by their token representation. `cache_this!` does not recognize changes to
//! expression results. To remove the cached values delete the `.cache-this` folder.
//!
//! Note that, for serializing and deserializing `cache_this!` relies on `serde_json`. Thus,
//! `cache_this!` requires to be cached expression results to implement the needed traits.
//!
//! # Example
//! ```rust
//! # use cache_this::cache_this;
//! # fn some_long_running_function() -> u8 {0u8}
//! /// `some_long_running_function` will be executed here once
//! let result = cache_this!(some_long_running_function());
//! # assert_eq!(0u8, result);
//! ///...
//! /// in following calls the cached result will be read without
//! /// executing the function
//! # fn some_other_method(a : u8) {}
//! some_other_method(cache_this!(some_long_running_function()));
//!
//! # std::fs::remove_dir_all(".cache-this").unwrap()
//! ```

/// `cache_this` macro enabling easy caching of expressions
pub use cache_this_proc_macro::cache_this;

#[doc(hidden)]
pub use ::serde_json::{to_string, from_str};
