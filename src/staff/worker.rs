use crate::traits::{CompanyBehaviour, Employee, StaffEntity};
use crate::types::{Company, Resource, Result};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub struct Worker {
    id: Uuid,
    company: Rc<RefCell<Company>>,
}

impl Worker {
    pub fn new(company: Rc<RefCell<Company>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            company,
        }
    }
}

impl StaffEntity for Worker {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_resource_amount(&self) -> Result<Resource> {
        self.company.borrow().get_resource_amount(self.id)
    }

    fn spend(&mut self, amount: Resource) -> Result<Resource> {
        self.company.borrow_mut().spend_resource(self.id, amount)
    }

    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource> {
        self.company
            .as_ref()
            .borrow_mut()
            .transfer_resources(self.id, to, amount)
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<Resource> {
        self.company.borrow_mut().recieve_resource(self.id, amount)
    }
}

impl Employee for Worker {
    fn get_supervisor_id(&self) -> Option<Uuid> {
        self.company.as_ref().borrow().get_supervisor_id(self.id)
    }
}
