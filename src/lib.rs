//! `staff_lib` defines the traits and struct for making the simple company hierarchy with
//! opportunities to `hire` and `layoff` staff members and flexible resouce shring between them.
//!
//! Key module elements:
//! - Traits: [`crate::traits::StaffEntity`]
//! - Struct: [`crate::company::Company`]
//!
//!
//! # Library design overview:
//! - Only `Company` owns all the data stored in it
//! - All staff entites act as `view` objects used for interaction with the company object, but do not hold data themselves.

pub mod company;
pub mod errors;
pub mod traits;

pub use company::Company;
pub use traits::StaffEntity;
