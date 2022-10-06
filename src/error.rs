//! Custom error

use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

/// Error return type
pub type Error = Box<dyn StdError>;

/// Result return type
pub type Result<T> = StdResult<T, Box<dyn StdError>>;

/// Fail structure
/// ```
/// use kern::{Fail, Result};
///
/// fn do_something() -> Result<()> {
///     let err = Fail::new("This is an error");
///     Fail::from(err)
/// }
///
/// println!("{}", do_something().unwrap_err());
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Fail(pub String);

// Fail implementation
impl Fail {
    /// Create bxoed Fail from any Display
    pub fn new<E>(err: E) -> Box<Self>
    where
        E: Display,
    {
        Box::new(Fail(err.to_string()))
    }

    /// Create Result with boxed Fail from any Display
    pub fn from<T, E>(err: E) -> Result<T>
    where
        E: Display,
    {
        Err(Self::new(err))
    }

    /// Create StdResult with Fail from any Display
    pub fn std<T, E>(err: E) -> StdResult<T, Self>
    where
        E: Display,
    {
        Err(Fail(err.to_string()))
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
