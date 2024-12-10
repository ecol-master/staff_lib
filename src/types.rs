use crate::errors::StaffError;

/// A type alias which is used for representing resource type in company hierarchy
pub type Resource = u64;

pub type Result<T> = std::result::Result<T, StaffError>;
