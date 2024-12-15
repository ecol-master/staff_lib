//! Defines [`crate::errors::StaffError`] enum

/// Enum representing errors related to staff management in the company.
#[derive(Debug)]
pub enum StaffError<ID, R> {
    /// Error indicating that there are not enough resources in staff entity's balance.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity.
    InsufficientResourcesError { id: ID, requied: R, available: R },

    /// Error indicating that the staff entity with the given id not found.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity.
    StaffNotFound { id: ID },

    /// Error indicating that the staff entity already exists in the system.
    ///
    /// Is is commonly cause when supervisor tries to hire the staff member which is already in the
    /// company.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity.
    StaffAlreadyExists { id: ID },

    /// Error indicating that a staff entity does not have the necessary permissions to do some
    /// operations.
    ///
    /// Is is commonly cause when supervisor tries to layoff the staff entity which is not in its
    /// subordinates set.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity attempting the operation.
    StaffHasNoPermission { id: ID },
}
