#![warn(missing_docs)]
#![allow(dead_code)]

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

mod code;
mod error;
mod modifier;

use code::KeyCode;
pub use error::{Error, Result};
use modifier::{KeyModifier, KeyModifiers};

#[derive(Clone, Copy)]
/// Minimum unit to use for parsing.
///
/// ## Example
/// ```
/// use viks::Key;
///
/// # fn main() {
/// let shift_a_key = Key::new("A").unwrap();
/// let shift_a_key_alt = Key::new("<s-a>").unwrap();
///
/// assert_eq!(shift_a_key, shift_a_key_alt);
/// # }
/// ```
pub struct Key {
    code: KeyCode,
    modifiers: KeyModifiers,
}

impl Key {
    /// Create new key.
    ///
    /// ## Example
    /// ```
    /// use viks::Key;
    ///
    /// # fn main() {
    /// let shift_a_key = Key::new("A").unwrap();
    /// let shift_a_key_alt = Key::new("<s-a>").unwrap();
    ///
    /// assert_eq!(shift_a_key, shift_a_key_alt);
    /// # }
    /// ```
    ///
    /// # Error
    ///
    /// arg is
    /// - not ascii
    /// - empty
    /// - can't convert to char if len == 1
    /// - invisible code, except:
    ///   - Backspace
    ///   - Tab
    ///   - Enter
    ///   - Esc
    ///   - Space
    ///   - Delete
    /// - not surrounded <> if len > 1
    /// - surrounded <> but not available
    pub fn new(tag: &str) -> self::Result<Self> {
        use std::str::FromStr;

        if !tag.is_ascii() {
            return Err(Error(String::from("unsupported key format")));
        }

        if tag.is_empty() {
            return Err(Error(String::from("format is empty")));
        }

        if tag.len() == 1 {
            let Ok(tag) = char::from_str(tag) else {
                return Err(Error(String::from("unsupported key format")));
            };

            let modifier = if tag.is_ascii_uppercase() {
                KeyModifier::Shift
            } else {
                KeyModifier::None
            };

            let tag = tag.to_ascii_uppercase();

            let code = match tag {
                'A'..='Z' => KeyCode::from_ascii(tag as u8),
                '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | '?' | '_'
                | '`' | '|' | '~' | '{' | '}' | '-' | '[' | ']' | ',' | '.' | '/' | ':' | ';'
                | '>' | '=' | '@' | '\\' | '^' => KeyCode::from_ascii(tag as u8),

                tag if tag.is_ascii_digit() => KeyCode::from_ascii(tag as u8),

                _ => return Err(Error(String::from("unsupported key format"))),
            };

            return Ok(Key {
                code,
                modifiers: KeyModifiers(modifier),
            });
        }

        let is_special = tag.starts_with("<") && tag.ends_with(">");

        if !is_special || tag.len() == 2 {
            return Err(Error(String::from("unsupported key format")));
        }

        let is_modded = tag.chars().nth(2).is_some_and(|c| c == '-');
        let base = if is_modded {
            &tag[3..tag.len() - 1]
        } else {
            &tag[1..tag.len() - 1]
        };
        let modifier = if is_modded {
            match tag.chars().nth(1).map(|c| c.to_ascii_lowercase()) {
                Some('a') => KeyModifier::Alt,
                Some('c') => KeyModifier::Control,
                Some('s') => KeyModifier::Shift,
                _ => KeyModifier::None,
            }
        } else {
            KeyModifier::None
        };

        if base.len() == 1 {
            let mut key = Key::new(base)?;

            key.modifiers = KeyModifiers(key.modifiers.0 | modifier);

            return Ok(key);
        }

        let code = match base.to_lowercase().as_str() {
            "enter" | "cr" => KeyCode::Enter,
            "tab" => KeyCode::Tab,
            "esc" => KeyCode::Esc,
            "leader" | "space" => KeyCode::Space,
            "bs" => KeyCode::Backspace,
            "del" => KeyCode::Delete,
            "lt" => KeyCode::LessThanSign,
            _ => return Err(Error(String::from("unsupported key format"))),
        };

        Ok(Key {
            code,
            modifiers: KeyModifiers(modifier),
        })
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let is_special = matches!(
                self.code,
                KeyCode::Enter
                    | KeyCode::Tab
                    | KeyCode::Esc
                    | KeyCode::Space
                    | KeyCode::Backspace
                    | KeyCode::Delete
                    | KeyCode::LessThanSign,
            );
            let is_modded = self.modifiers.is_alt() || self.modifiers.is_ctrl();
            let is_shift = self.modifiers.is_shift();
            let is_alpha = matches!(self.code as u8, 65..=90);

            let code = match &self.code {
                KeyCode::Enter => "CR",
                KeyCode::Tab => "TAB",
                KeyCode::Esc => "ESC",
                KeyCode::Space => "SPACE",
                KeyCode::Backspace => "BS",
                KeyCode::Delete => "DEL",
                KeyCode::LessThanSign => "LT",

                keycode if !is_shift && is_alpha => {
                    &format!("{}", keycode.as_ascii().to_ascii_lowercase())
                }

                keycode => &format!("{}", keycode.as_ascii()),
            };

            let code = if self.modifiers.is_alt() {
                &format!("a-{}", code)
            } else if self.modifiers.is_ctrl() {
                &format!("c-{}", code)
            } else if is_shift && !is_alpha {
                &format!("s-{}", code)
            } else {
                &code.to_string()
            };

            if is_special || is_modded || is_shift && !is_alpha {
                format!("<{}>", code)
            } else {
                code.to_string()
            }
        })
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key {{ code: {}, modifiers: {:#05b} }}",
            self.code as u8, self.modifiers.0 as u8
        )
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.modifiers == other.modifiers
    }
}
