use crate::errors::StaffError;
use uuid::Uuid;

/// A type alias which is used for representing resource type in company hierarchy
pub type Resource = u64;

/// Type alias for StaffID
pub type StaffID = Uuid;

pub type Result<T> = std::result::Result<T, StaffError>;
