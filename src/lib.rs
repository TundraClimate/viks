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

#[repr(u8)]
#[derive(Clone, Copy)]
/// Visualized ascii code of char.
///
/// It is not recommended to uses for other than parse.
pub enum KeyCode {
    /// `Backspace`  
    /// u0008
    Backspace = 8,

    /// `Tab`  
    /// u0009
    Tab = 9,

    /// `Enter`  
    /// u0013
    Enter = 13,

    /// `Esc`  
    /// u01B
    Esc = 27,

    /// `Space`  
    /// u0020
    Space = 32,

    /// `!`  
    /// u0021
    ExclamationMark = 33,

    /// `"`  
    /// u0022
    QuotationMark = 34,

    /// `#`  
    /// u0023
    NumberSign = 35,

    /// `$`  
    /// u0024
    DollarSign = 36,

    /// `%`  
    /// u0025
    PercentSign = 37,

    /// `&`  
    /// u0026
    Ampersand = 38,

    /// `'`  
    /// u0027
    Apostrophe = 39,

    /// `(`  
    /// u0028
    LeftParenthesis = 40,

    /// `)`  
    /// u0029
    RightParenthesis = 41,

    /// `*`  
    /// u002A
    Asterisk = 42,

    /// `+`  
    /// u002B
    PlusSign = 43,

    /// `,`  
    /// u002C
    Comma = 44,

    /// `-`  
    /// u002D
    HyphenMinus = 45,

    /// `.`  
    /// u002E
    FullStop = 46,

    /// `/`  
    /// u002F
    Solidus = 47,

    /// `0`  
    /// u0030
    Digit0 = 48,

    /// `1`  
    /// u0031
    Digit1 = 49,

    /// `2`  
    /// u0032
    Digit2 = 50,

    /// `3`  
    /// u0033
    Digit3 = 51,

    /// `4`  
    /// u0034
    Digit4 = 52,

    /// `5`  
    /// u0035
    Digit5 = 53,

    /// `6`  
    /// u0036
    Digit6 = 54,

    /// `7`  
    /// u0037
    Digit7 = 55,

    /// `8`  
    /// u0038
    Digit8 = 56,

    /// `9`  
    /// u0039
    Digit9 = 57,

    /// `:`  
    /// u003A
    Colon = 58,

    /// `;`  
    /// u003B
    Semicolon = 59,

    /// `<`  
    /// u003C
    LessThanSign = 60,

    /// `=`  
    /// u003D
    EqualSign = 61,

    /// `>`  
    /// u003E
    GreaterThanSign = 62,

    /// `?`  
    /// u003F
    QuestionMark = 63,

    /// `@`  
    /// u0040
    CommercialAt = 64,

    /// `A`  
    /// u0041
    A = 65,

    /// `B`  
    /// u0042
    B = 66,

    /// `C`  
    /// u0043
    C = 67,

    /// `D`  
    /// u0044
    D = 68,

    /// `E`  
    /// u0045
    E = 69,

    /// `F`  
    /// u0046
    F = 70,

    /// `G`  
    /// u0047
    G = 71,

    /// `H`  
    /// u0048
    H = 72,

    /// `I`  
    /// u0049
    I = 73,

    /// `J`  
    /// u004A
    J = 74,

    /// `K`  
    /// u004B
    K = 75,

    /// `L`  
    /// u004C
    L = 76,

    /// `M`  
    /// u004D
    M = 77,

    /// `N`  
    /// u004E
    N = 78,

    /// `O`  
    /// u004F
    O = 79,

    /// `P`  
    /// u0050
    P = 80,

    /// `Q`  
    /// u0051
    Q = 81,

    /// `R`  
    /// u0052
    R = 82,

    /// `S`  
    /// u0053
    S = 83,

    /// `T`  
    /// u0054
    T = 84,

    /// `U`  
    /// u0055
    U = 85,

    /// `V`  
    /// u0056
    V = 86,

    /// `W`  
    /// u0057
    W = 87,

    /// `X`  
    /// u0058
    X = 88,

    /// `Y`  
    /// u0059
    Y = 89,

    /// `Z`  
    /// u005A
    Z = 90,

    /// `[`  
    /// u005B
    LeftSquareBracket = 91,

    /// `\`  
    /// u005C
    ReverseSolidas = 92,

    /// `]`  
    /// u005D
    RightSquareBracket = 93,

    /// `^`  
    /// u005E
    CircumflexAccent = 94,

    /// `_`  
    /// u005F
    LowLine = 95,

    /// `` ` ``  
    /// u0060
    GraveAccent = 96,

    /// `{`  
    /// u007B
    LeftCurlyBracket = 123,

    /// `|`  
    /// u007C
    VirticalLine = 124,

    /// `}`  
    /// u007D
    RightCurlyBracket = 125,

    /// `~`  
    /// u007E
    Tilde = 126,

    /// `Delete`  
    /// u007F
    Delete = 127,
}

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
