#![warn(missing_docs)]

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

/// Minimum unit to use for parsing.
///
/// Includes [KeyCode] and [KeyModifiers].  
pub struct Key {}

#[derive(Clone, Copy, PartialEq)]
/// Wrapper for [KeyModifier].
///
/// ## Example
/// ```
/// use viks::KeyModifiers;
/// use viks::KeyModifier;
///
/// # fn main() {
/// let modifiers = KeyModifiers(KeyModifier::Shift | KeyModifier::Alt);
///
/// assert!(modifiers.is_shift());
/// assert!(!modifiers.is_ctrl());
/// assert!(modifiers.is_alt());
/// # }
/// ```
pub struct KeyModifiers(KeyModifier);

impl KeyModifiers {
    /// return the shift contains self.
    pub fn is_shift(&self) -> bool {
        self.0 & KeyModifier::Shift == KeyModifier::Shift
    }

    /// return the control contains self.
    pub fn is_ctrl(&self) -> bool {
        self.0 & KeyModifier::Control == KeyModifier::Control
    }

    /// return the control contains self.
    pub fn is_alt(&self) -> bool {
        self.0 & KeyModifier::Alt == KeyModifier::Alt
    }

    /// return the self is none.
    pub fn is_none(&self) -> bool {
        self.0 == KeyModifier::None
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
/// Flag of key modifier.
///
/// It is not recommended to use it alone.  
/// [Key] uses with [KeyModifiers] on wrapped.
///
/// ## Example
/// ```
/// use viks::KeyModifier;
///
/// # fn main() {
/// let modifier = KeyModifier::Shift | KeyModifier::Alt;
/// # }
/// ```
pub enum KeyModifier {
    /// Shift modifier
    Shift = 0b0001,

    /// Control modifier
    Control = 0b0010,

    /// Alt modifier
    Alt = 0b0100,

    /// None modifier
    None = 0b0000,
}

impl std::ops::BitAnd for KeyModifier {
    type Output = KeyModifier;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 & rhs as u8) }
    }
}

impl std::ops::BitOr for KeyModifier {
    type Output = KeyModifier;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { std::mem::transmute(self as u8 | rhs as u8) }
    }
}
