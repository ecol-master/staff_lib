use crate::errors::StaffError;
use crate::traits::{EmployeeRef, StaffEntity, Supervisor};
use crate::types::{Resource, Result, StaffID};
use std::collections::HashMap;

pub struct CEO {
    id: StaffID,
    resource: Resource,
    subordinates: HashMap<StaffID, EmployeeRef>,
}

impl StaffEntity for CEO {
    fn get_id(&self) -> StaffID {
        self.id
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
}

impl Supervisor for CEO {
    fn hire(&mut self, employee: EmployeeRef) -> Result<()> {
        let id = employee.borrow().get_id();
        match self.subordinates.insert(id, employee) {
            Some(_) => Ok(()),
            None => Err(StaffError::EmployeeNotFound(id)),
        }
    }

    /// Layoff employee
    fn layoff(&mut self, employee_id: &StaffID) -> Result<EmployeeRef> {
        match self.subordinates.remove(employee_id) {
            Some(e) => {
                e.borrow_mut().on_layoff()?;
                Ok(e)
            }
            None => Err(StaffError::EmployeeNotFound(*employee_id)),
        }
    }

    /// Sends the resource to employee from the  current supervisor's subordinates list
    fn send_resources(&mut self, amount: Resource, employee_id: &StaffID) -> Result<()> {
        if self.resource < amount {
            return Err(StaffError::InsufficientResourcesError);
        }

        let employee = match self.subordinates.get(employee_id) {
            Some(e) => e,
            None => return Err(StaffError::EmployeeNotFound(*employee_id)),
        };

        employee.borrow_mut().recieve_resource(amount)?;
        self.resource -= amount;

        Ok(())
    }
}
