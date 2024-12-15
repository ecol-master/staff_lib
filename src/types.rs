//! Module provides the pre-defined types fot representing the most frequently used types.
//! - `Resource`
//! - `Result`
//!
//! Also defines the enumerations: [`crate::types::Staff`] and [`crate::types::Company`] which
//! helps to work with different entities uniformally.

use crate::errors::StaffError;

/// A wrapper for standart library [`std::result::Result`] with templated value and fixed error
/// type [`crate::errors::StaffError`]
pub type Result<T, ID, R> = std::result::Result<T, StaffError<ID, R>>;
