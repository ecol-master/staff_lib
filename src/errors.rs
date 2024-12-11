use crate::types::StaffID;

pub enum StaffError {
    InsufficientResourcesError,

    EmployeeNotFound(StaffID),

    EmployeeHasNoSupervisor(StaffID),
}
