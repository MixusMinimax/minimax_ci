use std::error::Error;
use std::fmt::{Display, Formatter};

use service_traits::ServiceKey;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ServiceNotFoundError(pub ServiceKey);

impl Display for ServiceNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Service with key '{}' not found", self.0)?;
        Ok(())
    }
}

impl Error for ServiceNotFoundError {}
