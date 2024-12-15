//! Module defines traits for making hierarhy for staff managment system.
//! Defined traits:
//! - [`crate::traits::StaffEntity`]
//! - [`crate::traits::Employee`]
//! - [`crate::traits::Supervisor`],
//! - [`crate::traits::CompanyBehaviour`]

use std::hash::Hash;

/// [`StaffEntity`] provides basic methods for company staff entity
pub trait StaffEntity {
    type ID: Eq + Hash;
    /// Method return the unique identifier of staff entity in company.
    ///
    /// # Example:
    /// ```
    /// let id: Uuid = staff_entity.get_id();
    /// println!("Staff ID: {}", id);
    /// ```
    fn get_id(&self) -> Self::ID;
}
