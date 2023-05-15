use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum EngineError {
	DimensionError,
}

impl fmt::Display for EngineError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::DimensionError => write!(f, "dimensions for 2D objects must be greater than 0"),
		}
	}
}

impl error::Error for EngineError {}
