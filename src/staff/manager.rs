use crate::company::Company;
use crate::errors::StaffError;
use crate::staff::worker::Worker;
use crate::traits::{Employee, StaffEntity, Supervisor};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

pub struct Manager {
    id: Uuid,
    company: Rc<RefCell<Company>>,
    resource: Resource,
    subordinates: HashSet<Uuid>,
}

impl Manager {
    pub fn new(company: Rc<RefCell<Company>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            resource: 0,
            company,
            subordinates: HashSet::new(),
        }
    }
}

impl StaffEntity for Manager {
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
        self.company.borrow().transfer(self.id, to, amount)
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<()> {
        self.resource += amount;
        Ok(())
    }
}

impl Employee for Manager {
    fn get_supervisor_id(&self) -> Option<Uuid> {
        self.company.as_ref().borrow().get_supervisor_id(self.id)
    }
}

impl Supervisor for Manager {
    fn hire_employee(&mut self, employee: Rc<RefCell<Worker>>) -> Result<Rc<RefCell<Worker>>> {
        let worker = self
            .company
            .as_ref()
            .borrow_mut()
            .add_employee(employee, self.id)?;
        self.subordinates.insert(worker.as_ref().borrow().get_id());
        Ok(worker)
    }

    fn hire_manager(&mut self, manager: Rc<RefCell<Manager>>) -> Result<Rc<RefCell<Manager>>> {
        let manager = self
            .company
            .as_ref()
            .borrow_mut()
            .add_manager(manager, self.id)?;

        self.subordinates.insert(manager.as_ref().borrow().get_id());
        Ok(manager)
    }

    fn layoff_employee(&mut self, employee_id: Uuid) -> Result<Rc<RefCell<Worker>>> {
        self.company
            .as_ref()
            .borrow_mut()
            .layoff_worker(employee_id, self.id)
    }

    fn layoff_manager(&mut self, manager_id: Uuid) -> Result<Rc<RefCell<Manager>>> {
        self.company
            .as_ref()
            .borrow_mut()
            .layoff_manager(manager_id, self.id)
    }

    fn assume_subordinates(&mut self, subordinates: HashSet<Uuid>) {
        self.subordinates.extend(subordinates);
    }

    fn release_subordinates(&mut self) -> HashSet<Uuid> {
        std::mem::replace(&mut self.subordinates, HashSet::new())
    }
}
