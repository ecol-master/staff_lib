use crate::types::{Resource, Result};
use std::cell::RefCell;
use uuid::Uuid;

pub trait Person {
    fn get_id(&self) -> Uuid;
}

pub trait Supervisor: Person {
    fn hire(&self, employee: RefCell<Box<dyn Employee>>) -> Result<()>;

    fn layoff(&self, employee: RefCell<Box<dyn Employee>>) -> Result<()>;

    fn send_resources(&self, amount: Resource, employee: RefCell<Box<dyn Employee>>) -> Resource;
}

pub trait Employee: Person {
    fn set_supervisor(&mut self, sv: RefCell<Box<dyn Supervisor>>) -> Result<()>;

    /// Method returns the amount of spended resources
    fn spend(&mut self, amount: Resource) -> Result<()>;
}
