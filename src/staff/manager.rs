use crate::traits::{Employee, Person, Supervisor};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use uuid::Uuid;

pub struct Manager {
    id: Uuid,
}

impl Person for Manager {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Supervisor for Manager {
    fn hire(&self, _employee: RefCell<Box<dyn Employee>>) -> Result<()> {
        todo!()
    }

    fn layoff(&self, _employee: RefCell<Box<dyn Employee>>) -> Result<()> {
        todo!()
    }

    fn send_resources(&self, _amount: Resource, _employee: RefCell<Box<dyn Employee>>) -> Resource {
        todo!()
    }
}
