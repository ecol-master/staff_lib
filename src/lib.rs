//! `staff_lib` defines the traits and struct for making the simple company hierarchy with
//! opportunities to `hire` and `layoff` staff members and flexible resouce shring between them.
//!
//! Key module elements:
//! - Traits: [`crate::traits::StaffEntity`], [`crate::traits::Employee`],
//!     [`crate::traits::Supervisor`] and [`crate::traits::CompanyBehaviour`]
//! - Staff implementations: [`crate::staff::ceo::CEO`], [`crate::staff::manager::Manager`] and
//!     [`crate::staff::worker::Worker`]
//! - Companies implementations: [`crate::companies::google::Google`]
//! - Defined enums: [`crate::types::Staff`] and [`crate::types::Company`]
//!
//! # Library design overview:
//! - Only `Company` owns all the data stored in it
//! - All staff entites act as `view` objects used for interaction with the company object, but do not hold data themselves.

pub mod company;
pub mod errors;
pub mod traits;

pub use company::Company;
