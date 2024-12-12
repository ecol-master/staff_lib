use crate::company::Company;
use crate::staff::employee::Employee;
use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

pub struct Manager {
    id: Uuid,
    resource: Resource,
    subordinates: HashSet<Uuid>,
    company: Rc<RefCell<Company>>,
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

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn hire_employee(&self, employee: Rc<RefCell<Employee>>) -> Result<Rc<RefCell<Employee>>> {
        self.company.as_ref().borrow_mut().add_employee(employee)
    }

    pub fn hire_manager(&self, manager: Rc<RefCell<Manager>>) -> Result<Rc<RefCell<Manager>>> {
        self.company.as_ref().borrow_mut().add_manager(manager)
    }

    pub fn layoff_employee(
        &self,
        employee: Rc<RefCell<Employee>>,
    ) -> Result<Rc<RefCell<Employee>>> {
        todo!()
    }

    pub fn layoff_manager(&self, employee: Rc<RefCell<Employee>>) -> Result<Rc<RefCell<Employee>>> {
        todo!()
    }
}
