use uuid::Uuid;

#[derive(Debug)]
pub enum StaffError {
    InsufficientResourcesError(Uuid),

    EmployeeNotFound(Uuid),

    EmployeeAlreadyExists(Uuid),

    EmployeeHasNoSupervisor(Uuid),
}
