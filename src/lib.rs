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

#[cfg(feature = "serde")]
pub mod serde_impl;

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
            return Err(Error::new(tag, "unsupported key format"));
        }

        if tag.is_empty() {
            return Err(Error::new(tag, "format is empty"));
        }

        if tag.len() == 1 {
            let Ok(tag_char) = char::from_str(tag) else {
                return Err(Error::new(tag, "unsupported key format"));
            };

            let modifier = if tag_char.is_ascii_uppercase() {
                KeyModifier::Shift
            } else {
                KeyModifier::None
            };

            let tag_uppercase = tag_char.to_ascii_uppercase();

            let code = match tag_uppercase {
                'A'..='Z' => KeyCode::from_ascii(tag_uppercase as u8),
                '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | '?' | '_'
                | '`' | '|' | '~' | '{' | '}' | '-' | '[' | ']' | ',' | '.' | '/' | ':' | ';'
                | '>' | '=' | '@' | '\\' | '^' => KeyCode::from_ascii(tag_uppercase as u8),

                tag_char if tag_char.is_ascii_digit() => KeyCode::from_ascii(tag_char as u8),

                _ => return Err(Error::new(tag, "unsupported key format")),
            };

            return Ok(Key {
                code,
                modifiers: KeyModifiers(modifier),
            });
        }

        let is_special = tag.starts_with("<") && tag.ends_with(">");

        if !is_special || tag.len() == 2 {
            return Err(Error::new(tag, "unsupported key format"));
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
            _ => return Err(Error::new(tag, "unsupported key format")),
        };

        Ok(Key {
            code,
            modifiers: KeyModifiers(modifier),
        })
    }

    /// Returns `true` if this `Key` is the alphabetic.
    ///
    /// # Example
    /// ```
    /// # use viks::Key;
    /// # fn main() {
    /// assert!(Key::new("a").unwrap().is_alpha());
    /// assert!(Key::new("<c-a>").unwrap().is_alpha());
    /// assert!(Key::new("A").unwrap().is_alpha());
    /// assert!(!Key::new("~").unwrap().is_alpha());
    /// assert!(!Key::new("1").unwrap().is_alpha());
    /// assert!(!Key::new(":").unwrap().is_alpha());
    /// # }
    /// ```
    pub fn is_alpha(&self) -> bool {
        self.code.as_ascii().is_uppercase()
    }

    /// Returns `true` if this `Key` code in '0'..='9'.
    ///
    /// # Example
    /// ```
    /// # use viks::Key;
    /// # fn main() {
    /// assert!(!Key::new("a").unwrap().is_digit());
    /// assert!(!Key::new("<c-a>").unwrap().is_digit());
    /// assert!(!Key::new("A").unwrap().is_digit());
    /// assert!(!Key::new("~").unwrap().is_digit());
    /// assert!(Key::new("1").unwrap().is_digit());
    /// assert!(Key::new("9").unwrap().is_digit());
    /// assert!(!Key::new(":").unwrap().is_digit());
    /// # }
    /// ```
    pub fn is_digit(&self) -> bool {
        self.code.as_ascii().is_ascii_digit()
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
                &format!("a-{code}")
            } else if self.modifiers.is_ctrl() {
                &format!("c-{code}")
            } else if is_shift && !is_alpha {
                &format!("s-{code}")
            } else {
                &code.to_string()
            };

            if is_special || is_modded || is_shift && !is_alpha {
                format!("<{code}>")
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

#[derive(Clone, PartialEq)]
/// Wrapper of [Vec]<[Key]>.
///
/// This only has parse func, please use `as_vec` to access to inner.
///
/// # Example
///
/// ```
/// use viks::Keymap;
///
/// # fn main() {
/// let exit_map = Keymap::new("ZZ").unwrap();
/// let exit_map_alt = Keymap::new("<s-z>Z").unwrap();
///
/// assert_eq!(exit_map, exit_map_alt);
/// # }
/// ```
pub struct Keymap(Vec<Key>);

impl Keymap {
    /// Create new Keymap.
    ///
    /// # Example
    ///
    /// ```
    /// use viks::Keymap;
    ///
    /// # fn main() {
    /// let exit_map = Keymap::new("ZZ").unwrap();
    /// let exit_map_alt = Keymap::new("<s-z>Z").unwrap();
    ///
    /// assert_eq!(exit_map, exit_map_alt);
    /// # }
    /// ```
    ///
    /// # Error
    ///
    /// Returns an error if the tag is not closed.
    pub fn new(s: &str) -> self::Result<Self> {
        let mut in_tag = false;
        let mut buf = String::new();
        let mut keys: Vec<Key> = vec![];

        for c in s.chars() {
            if c == '<' {
                in_tag = true;
            }

            if in_tag {
                buf.push(c);
            } else {
                keys.push(Key::new(&c.to_string())?)
            }

            if c == '>' && in_tag {
                in_tag = false;
                keys.push(Key::new(&buf)?);
                buf.clear();
            }
        }

        if in_tag {
            return Err(Error::new(s, "invalid format"));
        }

        Ok(Keymap(keys))
    }

    /// Get inner ref.
    ///
    /// # Example
    ///
    /// ```
    /// use viks::Keymap;
    ///
    /// # fn main() {
    /// let keymap = Keymap::new("ZZ").unwrap();
    ///
    /// for key in keymap.as_vec().iter() {
    ///     // ..
    /// }
    /// # }
    /// ```
    pub fn as_vec(&self) -> &Vec<Key> {
        &self.0
    }
}

impl std::fmt::Display for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .fold(String::new(), |acc, k| format!("{acc}{k}"))
        )
    }
}

impl std::fmt::Debug for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|k| format!("{k:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl From<Vec<Key>> for Keymap {
    fn from(value: Vec<Key>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn new_key() {
        let key1 = Key::new("A");
        let key2 = Key::new(";");
        let key3 = Key::new("<lt>");
        let key4 = Key::new("<BS>");
        let key5 = Key::new("<s-a>");
        let key6 = Key::new("<a-a>");
        let key7 = Key::new("<c-a>");
        let key8 = Key::new("<c-CR>");
        let key9 = Key::new("<leader>");

        assert!(key1.is_ok());
        assert!(key2.is_ok());
        assert!(key3.is_ok());
        assert!(key4.is_ok());
        assert!(key5.is_ok());
        assert!(key6.is_ok());
        assert!(key7.is_ok());
        assert!(key8.is_ok());
        assert!(key9.is_ok());
    }

    #[test]
    fn eq_keys() {
        let a_key = Key::new("A").unwrap();
        let a_s_key = Key::new("A").unwrap();
        let a_a_key = Key::new("<s-a>").unwrap();
        let a_a_b_key = Key::new("<s-A>").unwrap();

        assert_eq!(a_key, a_s_key);
        assert_eq!(a_key, a_a_key);
        assert_eq!(a_a_key, a_a_b_key);
    }

    #[test]
    fn invalid_key() {
        let key1 = Key::new("");
        let key2 = Key::new("ÿ");
        let key3 = Key::new("aa");
        let key4 = Key::new("<BOO>");
        let key5 = Key::new("<B");
        let key6 = Key::new(" ");

        assert!(key1.is_err());
        assert!(key2.is_err());
        assert!(key3.is_err());
        assert!(key4.is_err());
        assert!(key5.is_err());
        assert!(key6.is_err());
    }

    #[test]
    fn new_keymap() {
        let keys1 = Keymap::new("NewYonk");
        let keys2 = Keymap::new("<s-Z>Z");
        let keys3 = Keymap::new("<lt>HappyNewYear>");
        let keys4 = Keymap::new("<a-~><c-#><s-&>");
        let keys5 = Keymap::new("<leader><cr>");

        assert!(keys1.is_ok());
        assert!(keys2.is_ok());
        assert!(keys3.is_ok());
        assert!(keys4.is_ok());
        assert!(keys5.is_ok());
    }

    #[test]
    fn eq_keymaps() {
        let keys1 = Keymap::new("<cr>HiWorld<enter>").unwrap();
        let keys2 = Keymap::new("<enter>HiWorld<cr>").unwrap();

        assert_eq!(keys1, keys2);
    }

    #[test]
    fn invalid_keymap() {
        let keymap = Keymap::new("<leader");

        assert!(keymap.is_err());
    }

    #[test]
    fn display_key() {
        let key1 = Key::new("A").unwrap();
        let key2 = Key::new(";").unwrap();
        let key3 = Key::new("<lt>").unwrap();
        let key4 = Key::new("<leader>").unwrap();
        let key5 = Key::new("<DEL>").unwrap();
        let key6 = Key::new("<Enter>").unwrap();

        assert_eq!(key1.to_string(), "A".to_string());
        assert_eq!(key2.to_string(), ";".to_string());
        assert_eq!(key3.to_string(), "<LT>".to_string());
        assert_eq!(key4.to_string(), "<SPACE>".to_string());
        assert_eq!(key5.to_string(), "<DEL>".to_string());
        assert_eq!(key6.to_string(), "<CR>".to_string());
    }

    #[test]
    fn debug_key() {
        let key1 = Key::new("A").unwrap();
        let key2 = Key::new(";").unwrap();
        let key3 = Key::new("<CR>").unwrap();
        let key4 = Key::new("<a-lt>").unwrap();

        assert_eq!(
            format!("{key1:?}"),
            "Key { code: 65, modifiers: 0b001 }".to_string()
        );
        assert_eq!(
            format!("{key2:?}"),
            "Key { code: 59, modifiers: 0b000 }".to_string()
        );
        assert_eq!(
            format!("{key3:?}"),
            "Key { code: 13, modifiers: 0b000 }".to_string()
        );
        assert_eq!(
            format!("{key4:?}"),
            "Key { code: 60, modifiers: 0b100 }".to_string()
        );
    }

    #[test]
    fn display_keymap() {
        let keys1 = Keymap::new("<leader><CR>").unwrap();
        let keys2 = Keymap::new("<lt>abAs<s-b>").unwrap();

        assert_eq!(keys1.to_string(), "<SPACE><CR>");
        assert_eq!(keys2.to_string(), "<LT>abAsB");
    }

    #[test]
    fn debug_keymap() {
        let keys1 = Keymap::new("aa").unwrap();
        let keys2 = Keymap::new("<lt>>").unwrap();
        let keys3 = Keymap::new("<cr>,").unwrap();

        assert_eq!(
            format!("{keys1:?}"),
            "[Key { code: 65, modifiers: 0b000 }, Key { code: 65, modifiers: 0b000 }]".to_string()
        );
        assert_eq!(
            format!("{keys2:?}"),
            "[Key { code: 60, modifiers: 0b000 }, Key { code: 62, modifiers: 0b000 }]".to_string()
        );
        assert_eq!(
            format!("{keys3:?}"),
            "[Key { code: 13, modifiers: 0b000 }, Key { code: 44, modifiers: 0b000 }]".to_string()
        );
    }
}
