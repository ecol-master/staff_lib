use crate::traits::{CompanyBehaviour, StaffEntity, Supervisor};
use crate::types::{Company, Resource, Result, Staff};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct CEO {
    id: Uuid,
    company: Rc<RefCell<Company>>,
}

impl CEO {
    pub fn new(company: Rc<RefCell<Company>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            company,
        }
    }
}

impl StaffEntity for CEO {
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
            .borrow_mut()
            .transfer_resources(self.id, to, amount)
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<Resource> {
        self.company.borrow_mut().recieve_resource(self.id, amount)
    }
}

impl Supervisor for CEO {
    fn hire(&mut self, staff_entity: Staff) -> Result<Uuid> {
        self.company.borrow_mut().hire(staff_entity, self.id)
    }

    fn layoff(&mut self, staff_id: Uuid) -> Result<Staff> {
        self.company.borrow_mut().layoff(staff_id, self.id)
    }

    fn get_subordinates(&self) -> Option<HashSet<Uuid>> {
        self.company.borrow().get_subordinates(self.id)
    }
}
