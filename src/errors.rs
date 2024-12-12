use uuid::Uuid;

#[derive(Debug)]
pub enum StaffError {
    InsufficientResourcesError,

    EmployeeNotFound(Uuid),

    EmployeeAlreadyExists(Uuid),

    EmployeeHasNoSupervisor(Uuid),
}
