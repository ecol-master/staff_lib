use crate::company::Company;
use crate::traits::{Employee, StaffEntity, Supervisor};
use crate::types::{Resource, Result, Staff};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Manager {
    id: Uuid,
    company: Rc<RefCell<Company>>,
}

impl Manager {
    pub fn new(company: Rc<RefCell<Company>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            company,
        }
    }
}

impl StaffEntity for Manager {
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

impl Employee for Manager {
    fn get_supervisor_id(&self) -> Option<Uuid> {
        self.company.as_ref().borrow().get_supervisor_id(self.id)
    }
}

impl Supervisor for Manager {
    fn hire(&mut self, staff_entity: Staff) -> Result<Uuid> {
        self.company
            .as_ref()
            .borrow_mut()
            .hire(staff_entity, self.id)
    }

    fn layoff(&mut self, employee_id: Uuid) -> Result<Staff> {
        self.company
            .as_ref()
            .borrow_mut()
            .layoff(employee_id, self.id)
    }
}
