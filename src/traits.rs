use crate::types::Staff;
use crate::types::{Resource, Result};
use uuid::Uuid;

/// [`StaffEntity`] provides basic methods for company staff entity
pub trait StaffEntity {
    fn get_id(&self) -> Uuid;

    fn get_resource_amount(&self) -> Result<Resource>;

    /// Method returns an amount of spended resources or [`crate::errors::StaffError::InsufficientResourcesError`]
    fn spend(&mut self, amount: Resource) -> Result<Resource>;

    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource>;

    fn recieve_resource(&mut self, amount: Resource) -> Result<Resource>;
}

/// [`Employee`]
pub trait Employee {
    fn get_supervisor_id(&self) -> Option<Uuid>;
}

/// [`Supervisor`]
pub trait Supervisor {
    fn hire(&mut self, staff_entity: Staff) -> Result<Uuid>;

    fn layoff(&mut self, staff_id: Uuid) -> Result<Staff>;
}
