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

#[derive(Debug)]
/// viks error type.
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

/// viks result alias.
pub type Result<T> = std::result::Result<T, Error>;

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

#[repr(u8)]
#[derive(Clone, Copy)]
enum KeyCode {
    Backspace = 8,
    Tab = 9,
    Enter = 13,
    Esc = 27,
    Space = 32,
    ExclamationMark = 33,
    QuotationMark = 34,
    NumberSign = 35,
    DollarSign = 36,
    PercentSign = 37,
    Ampersand = 38,
    Apostrophe = 39,
    LeftParenthesis = 40,
    RightParenthesis = 41,
    Asterisk = 42,
    PlusSign = 43,
    Comma = 44,
    HyphenMinus = 45,
    FullStop = 46,
    Solidus = 47,
    Digit0 = 48,
    Digit1 = 49,
    Digit2 = 50,
    Digit3 = 51,
    Digit4 = 52,
    Digit5 = 53,
    Digit6 = 54,
    Digit7 = 55,
    Digit8 = 56,
    Digit9 = 57,
    Colon = 58,
    Semicolon = 59,
    LessThanSign = 60,
    EqualSign = 61,
    GreaterThanSign = 62,
    QuestionMark = 63,
    CommercialAt = 64,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftSquareBracket = 91,
    ReverseSolidas = 92,
    RightSquareBracket = 93,
    CircumflexAccent = 94,
    LowLine = 95,
    GraveAccent = 96,
    LeftCurlyBracket = 123,
    VirticalLine = 124,
    RightCurlyBracket = 125,
    Tilde = 126,
    Delete = 127,
}
impl KeyCode {
    fn from_ascii(ascii: u8) -> KeyCode {
        if !matches!(ascii, 0..128) {
            panic!("not ascii");
        }

        unsafe { std::mem::transmute(ascii) }
    }

    fn as_ascii(&self) -> char {
        std::char::from_u32(*self as u32).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq)]
struct KeyModifiers(KeyModifier);

impl KeyModifiers {
    pub fn is_shift(&self) -> bool {
        self.0 & KeyModifier::Shift == KeyModifier::Shift
    }

    pub fn is_ctrl(&self) -> bool {
        self.0 & KeyModifier::Control == KeyModifier::Control
    }

    pub fn is_alt(&self) -> bool {
        self.0 & KeyModifier::Alt == KeyModifier::Alt
    }

    pub fn is_none(&self) -> bool {
        self.0 == KeyModifier::None
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum KeyModifier {
    Shift = 0b0001,
    Control = 0b0010,
    Alt = 0b0100,
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
