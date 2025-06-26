#[derive(Debug)]
/// viks error type.
///
/// # Example
///
/// ```
/// # use viks::Key;
///
/// let key = Key::new("A");
///
/// if let Err(err) = key {
///     eprintln!("incorrect syntax: {}", err);
/// }
/// ```
pub struct Error(pub(crate) String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

/// viks result alias.
pub type Result<T> = std::result::Result<T, Error>;
