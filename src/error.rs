//! Custom error

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Error return type
pub type Error = Box<dyn StdError>;

/// Fail structure
/// ```
/// use kern::Fail;
///
/// fn do_something() -> Result<(), Fail> {
///     let err = Fail::new("This is an error");
///     Fail::from(err)
/// }
///
/// println!("{}", do_something().unwrap_err());
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Fail(String);

// Fail implementation
impl Fail {
    /// Create Fail from any Display
    pub fn new<E>(err: E) -> Self
    where
        E: Display,
    {
        Fail(err.to_string())
    }

    /// Create Result with Fail from any Display
    pub fn from<T, E>(err: E) -> Result<T, Self>
    where
        E: Display,
    {
        Err(Self::new(err))
    }

    /// Get error message
    pub fn err_msg(&self) -> &str {
        &self.0
    }
}

/// std::error::Error implementation for Fail
impl StdError for Fail {}

/// Display implementation for Fail
impl Display for Fail {
    // fmt implementation
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{}", self.0)
    }
}
