use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::rc::Rc;

/// [`StaffEntity`] provides basic methods for company staff entity
pub trait StaffEntity {
    /// Method returns an [`crate::errors::StaffError::InsufficientResourcesError`] if staff entity resource is not enough
    fn spend(&mut self, amount: Resource) -> Result<()>;

    ///
    fn recieve_resource(&mut self, amount: Resource) -> Result<()>;
}

pub type StaffEntityRef = Rc<RefCell<dyn StaffEntity>>;

/*
/// [`Supervisor`] extends the [`StaffEntity`] object
pub trait Supervisor: StaffEntity {
    /// Hire new employee to the company
    fn hire(&mut self, employee: EmployeeRef) -> Result<()>;

    /// Layoff employee
    fn layoff(&mut self, employee_id: &StaffID) -> Result<EmployeeRef>;

    /// Sends the resource to employee from the  current supervisor's subordinates list
    fn send_resources(&mut self, amount: Resource, employee_id: &StaffID) -> Result<()>;

    fn as_any(&self) -> &dyn Any;
}

pub trait Employee: StaffEntity {
    /// Set supervisor for concrete employee
    fn set_supervisor(&mut self, sv: SupervisorRef) -> Result<()>;

    fn on_layoff(&mut self) -> Result<()>;

    fn as_any(&self) -> &dyn Any;
}

pub trait Manager: Supervisor + Employee {}

pub trait TopManager: Supervisor + StaffEntity {}

pub trait StaffStorage{
    fn
}
*/
