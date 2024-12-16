//! Defines [`crate::errors::Error`] enum.

/// Enum representing errors related to staff management in the company.
#[derive(Debug)]
pub enum Error<ID, R> {
    /// Error indicating that there are not enough resources in staff entity's balance.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity.
    /// * `required`: required resoucre amount to apply action.
    /// * `available `: required resoucre amount exists in staff member's balance.
    InsufficientResourcesError {
        staff_id: ID,
        required: R,
        available: R,
    },

    /// Error indicating that the staff entity with the given id not found.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity.
    StaffNotFound { staff_id: ID },

    /// Error indicating that the staff entity already exists in the system.
    ///
    /// Is is commonly cause when supervisor tries to hire the staff member which is already in the
    /// company.
    ///
    /// # Parameters
    /// * `staff_id`: The unique identifier of the staff entity.
    StaffAlreadyExists { staff_id: ID },

    /// Error indicating that a staff entity does not have the necessary permissions to do some
    /// operations.
    ///
    /// Is is commonly cause when supervisor tries to layoff the staff entity which is not in its
    /// subordinates set.
    ///
    /// # Parameters
    /// * `id`: The unique identifier of the staff entity attempting the operation.
    StaffHasNoPermission { staff_id: ID },

    /// Error indicating that a staff entity tries to fire the company's ceo.
    CannotFireCeo,

    /// Error indicating that a staff entity can not be a supervisor for another
    HierarchyConflict { staff_id: ID, supervisor_id: ID },
}
