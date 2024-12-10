use crate::errors::StaffError;
use crate::traits::{Employee, Person, Supervisor};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use uuid::Uuid;

pub struct Consultant {
    id: Uuid,
    resouces: Resource,
    sv: RefCell<Box<dyn Supervisor>>,
}

impl Person for Consultant {
    fn get_id(&self) -> Uuid {
        self.id
    }
}

impl Employee for Consultant {
    fn set_supervisor(&mut self, sv: RefCell<Box<dyn Supervisor>>) -> Result<()> {
        self.sv = sv;
        Ok(())
    }

    /// Method returns the amount of spended resources
    fn spend(&mut self, amount: Resource) -> Result<()> {
        if self.resouces < amount {
            return Err(StaffError::InsufficientResourcesError(String::from(
                "employee has no much resources to spend",
            )));
        }

        self.resouces -= amount;

        Ok(())
    }
}
