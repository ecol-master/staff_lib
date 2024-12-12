use crate::errors::StaffError;
use crate::staff::{ceo::CEO, employee::Employee, manager::Manager};
use crate::types::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

pub struct Company {
    ceo: Option<Rc<RefCell<CEO>>>,
    managers: HashMap<Uuid, Rc<RefCell<Manager>>>,
    employes: HashMap<Uuid, Rc<RefCell<Employee>>>,
}

impl Company {
    pub fn new() -> Self {
        Self {
            ceo: None,
            managers: HashMap::new(),
            employes: HashMap::new(),
        }
    }

    pub fn set_ceo(&mut self, ceo: Rc<RefCell<CEO>>) -> Result<()> {
        self.ceo = Some(ceo);
        Ok(())
    }

    pub fn add_manager(&mut self, manager: Rc<RefCell<Manager>>) -> Result<Rc<RefCell<Manager>>> {
        let id = manager.as_ref().borrow().get_id();
        match self.managers.insert(id, manager) {
            Some(s) => Ok(s),
            None => Err(StaffError::EmployeeAlreadyExists(id)),
        }
    }

    pub fn add_employee(
        &mut self,
        employee: Rc<RefCell<Employee>>,
    ) -> Result<Rc<RefCell<Employee>>> {
        let id = employee.as_ref().borrow().get_id();
        match self.employes.insert(id, employee) {
            Some(e) => Ok(e),
            None => Err(StaffError::EmployeeAlreadyExists(id)),
        }
    }
}
