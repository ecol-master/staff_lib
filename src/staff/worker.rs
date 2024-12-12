use crate::company::Company;
use crate::errors::StaffError;
use crate::traits::{Employee, StaffEntity};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub struct Worker {
    id: Uuid,
    resource: Resource,
    company: Rc<RefCell<Company>>,
}

impl Worker {
    pub fn new(company: Rc<RefCell<Company>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            resource: 0,
            company,
        }
    }
}

impl StaffEntity for Worker {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_resource_amount(&self) -> Resource {
        self.resource
    }

    fn spend(&mut self, amount: Resource) -> Result<Resource> {
        if self.resource < amount {
            return Err(StaffError::InsufficientResourcesError(self.id));
        }

        self.resource -= amount;
        Ok(amount)
    }

    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource> {
        self.company.as_ref().borrow().transfer(self.id, to, amount)
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<()> {
        self.resource -= amount;
        Ok(())
    }
}

impl Employee for Worker {
    fn get_supervisor_id(&self) -> Option<Uuid> {
        self.company.as_ref().borrow().get_supervisor_id(self.id)
    }
}
