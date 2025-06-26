//! viks is parser for vim-like key sequence.  
//! e.g. the *noremap line has unique keymap syntax in `.vimrc`.
//!
//! This crate is implmentation for that syntax parsing.
//!
//! ## Example
//!
//! ```
//! use viks::Key;
//! use viks::Keymap;
//!
//! # fn main() {
//! let shift_a_key = Key::new("A").unwrap();
//! let shift_a_key_alt = Key::new("<s-a>").unwrap();
//!
//! assert_eq!(shift_a_key, shift_a_key_alt);
//!
//! let exit_map = Keymap::new("ZZ").unwrap();
//! let exit_map_alt = Keymap::new("<s-z>Z").unwrap();
//!
//! assert_eq!(exit_map, exit_map_alt);
//! # }
//! ```
//!
//! #### Dynamic reading
//!
//! ```
//! use viks::Keymap;
//!
//! # fn main() {
//! // replace to some dynamic reading
//! let line = "<c-b>jj";
//!
//! if let Ok(map) = Keymap::new(line) {
//!     // ..
//! }
//! # }
//! ```
