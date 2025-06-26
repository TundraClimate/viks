#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum KeyCode {
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
    pub(crate) fn from_ascii(ascii: u8) -> KeyCode {
        if !matches!(ascii, 0..128) {
            panic!("not ascii");
        }

        unsafe { std::mem::transmute(ascii) }
    }

    pub(crate) fn as_ascii(&self) -> char {
        std::char::from_u32(*self as u32).unwrap()
    }
}
