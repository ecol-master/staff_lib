use crate::staff::{manager::Manager, worker::Worker};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

/// [`StaffEntity`] provides basic methods for company staff entity
pub trait StaffEntity {
    fn get_id(&self) -> Uuid;

    /// Method returns an [`crate::errors::StaffError::InsufficientResourcesError`]
    fn spend(&mut self, amount: Resource) -> Result<()>;

    fn recieve_resource(&mut self, amount: Resource) -> Result<()>;
}

/// [`Employee`]
pub trait Employee {
    fn get_supervisor_id(&self) -> Option<Uuid>;

    fn set_supervisor_id(&mut self, supervisor_id: Option<Uuid>);
}

/// [`Supervisor`]
pub trait Supervisor {
    fn hire_employee(&self, employee: Rc<RefCell<Worker>>) -> Result<Rc<RefCell<Worker>>>;
    fn hire_manager(&self, manager: Rc<RefCell<Manager>>) -> Result<Rc<RefCell<Manager>>>;

    fn layoff_employee(&self, employee: Rc<RefCell<Worker>>) -> Result<Rc<RefCell<Worker>>>;
    fn layoff_manager(&self, employee: Rc<RefCell<Worker>>) -> Result<Rc<RefCell<Worker>>>;
}
