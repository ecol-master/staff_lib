use uuid::Uuid;

#[derive(Debug)]
pub enum StaffError {
    InsufficientResourcesError(Uuid),

    StaffNotFound(Uuid),

    StaffAlreadyExists(Uuid),

    StaffHasNoPermission(Uuid, Uuid),

    CeoAlreadyExists(Uuid),
}
