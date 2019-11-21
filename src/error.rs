use std::error;
use std::fmt;

/// Error structure
/// ```
/// extern crate kern;
/// use kern::Error;
///
/// fn do_something() -> Result<(), Error> {
///     let err = Error::new("This is an error");
///     Error::from(err)
/// }
///
/// println!("{}", do_something().unwrap_err());
/// ```
#[derive(Clone, Debug)]
pub struct Error(String);

// Error implementation
impl Error {
    /// Create Error from any Display
    pub fn new<E>(err: E) -> Self
    where
        E: fmt::Display,
    {
        Error(err.to_string())
    }

    /// Create Result with Error from any Display
    pub fn from<T, E>(err: E) -> Result<T, Self>
    where
        E: fmt::Display,
    {
        Err(Self::new(err))
    }
}

/// std::error::Error implementation for Error
impl error::Error for Error {}

/// Display implementation for Error
impl fmt::Display for Error {
    // fmt implementation
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
