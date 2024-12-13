//! Rrovides object implementing the next traits:
//! - `CEO` implements [`crate::traits::Supervisor`] and [`crate::traits::StaffEntity`]
//! - `Manager` implements [`crate::traits::Supervisor`], [`crate::traits::Employee`] and [`crate::traits::StaffEntity`]
//! - `Worker` implements [`crate::traits::Supervisor`] and [`crate::traits::StaffEntity`]

pub mod ceo;
pub mod manager;
pub mod worker;
