use crate::company::Company;
use crate::errors::StaffError;
use crate::traits::{StaffEntity, StaffEntityRef};
use crate::types::{Resource, Result, StaffID};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

pub struct CEO {
    resource: Resource,
    company: Rc<RefCell<Company>>,
}

impl CEO {
    pub fn new(company: Rc<RefCell<Company>>) -> Self {
        Self {
            resource: 10000,
            company,
        }
    }

    fn spend(&mut self, amount: Resource) -> Result<()> {
        if self.resource < amount {
            return Err(StaffError::InsufficientResourcesError);
        }

        self.resource -= amount;
        Ok(())
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<()> {
        self.resource += amount;
        Ok(())
    }

    fn hire_manager(&mut self, employee_id: StaffEntityRef) -> Result<()> {
        todo!()
    }

    /// Layoff employee
    fn layoff(&mut self, employee_id: &Uuid) -> Result<StaffEntityRef> {
        todo!()
    }

    /// Sends the resource to employee from the  current supervisor's subordinates list
    fn send_resources(&mut self, amount: Resource, employee_id: &Uuid) -> Result<()> {
        todo!()
    }
}
