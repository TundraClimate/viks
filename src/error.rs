type Format = String;
type Cause = String;

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
pub struct Error(Format, Cause);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cause())
    }
}

impl Error {
    pub(crate) fn new(format: &str, cause: &str) -> Self {
        Self(format.to_string(), cause.to_string())
    }

    /// Get format of Error.
    ///
    /// # Example
    /// ```
    /// # use viks::Key;
    /// # fn main() {
    /// let key = Key::new("<ES");
    ///
    /// if let Err(e) = key {
    ///     println!("{}", e.format());
    /// }
    /// # }
    /// ```
    pub fn format(&self) -> &str {
        &self.0
    }

    /// Get error cause.
    ///
    /// # Example
    /// ```
    /// # use viks::Key;
    /// # fn main() {
    /// let key = Key::new("<ES");
    ///
    /// if let Err(e) = key {
    ///     println!("{}", e.cause());
    /// }
    /// # }
    /// ```
    pub fn cause(&self) -> &str {
        &self.1
    }
}

impl std::error::Error for Error {}

/// viks result alias.
pub type Result<T> = std::result::Result<T, Error>;
