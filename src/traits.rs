use crate::staff::{manager::Manager, worker::Worker};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

/// [`StaffEntity`] provides basic methods for company staff entity
pub trait StaffEntity {
    fn get_id(&self) -> Uuid;

    fn get_resource_amount(&self) -> Resource;

    /// Method returns an amount of spended resources or [`crate::errors::StaffError::InsufficientResourcesError`]
    fn spend(&mut self, amount: Resource) -> Result<Resource>;

    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource>;

    fn recieve_resource(&mut self, amount: Resource) -> Result<()>;
}

/// [`Employee`]
pub trait Employee {
    fn get_supervisor_id(&self) -> Option<Uuid>;
}

/// [`Supervisor`]
pub trait Supervisor {
    fn hire_employee(&mut self, employee: Rc<RefCell<Worker>>) -> Result<Rc<RefCell<Worker>>>;
    fn hire_manager(&mut self, manager: Rc<RefCell<Manager>>) -> Result<Rc<RefCell<Manager>>>;

    fn layoff_employee(&mut self, employee_id: Uuid) -> Result<Rc<RefCell<Worker>>>;
    fn layoff_manager(&mut self, manager_id: Uuid) -> Result<Rc<RefCell<Manager>>>;

    fn assume_subordinates(&mut self, subordinates: HashSet<Uuid>);
    fn release_subordinates(&mut self) -> HashSet<Uuid>;
}
