use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum EngineError {
	DimensionError,
	EventLoopElapsed,
}

impl fmt::Display for EngineError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::DimensionError =>
				write!(f, "dimensions for 2D objects must be greater than 0"),
			Self::EventLoopElapsed =>
				write!(f, "cannot run application, event loop has already elapsed"),
		}
	}
}

impl error::Error for EngineError {}
