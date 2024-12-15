//! Module defines [`StaffEntity`] trait for implementing the staff members object for company.

use std::hash::Hash;

/// [`StaffEntity`] provides only one method `get_id()`.
pub trait StaffEntity {
    type ID: Eq + Hash + Clone;
    /// Method return the unique identifier of staff entity in company.
    ///
    /// # Example:
    /// ```
    /// let id: Uuid = staff_entity.get_id();
    /// println!("Staff ID: {}", id);
    /// ```
    fn get_id(&self) -> Self::ID;
}
