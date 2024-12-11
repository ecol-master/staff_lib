use crate::errors::StaffError;
use crate::traits::{Employee, EmployeeRef, StaffEntity, Supervisor, SupervisorRef};
use crate::types::{Resource, Result, StaffID};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Manager {
    id: Uuid,
    sv: Option<SupervisorRef>,
    resource: Resource,
    subordinates: HashMap<Uuid, EmployeeRef>,
}

impl StaffEntity for Manager {
    fn get_id(&self) -> Uuid {
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

impl Supervisor for Manager {
    /// Hire new employee to the company
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
        let employee = match self.subordinates.get_mut(employee_id) {
            Some(e) => e,
            None => return Err(StaffError::EmployeeNotFound(*employee_id)),
        };

        if self.resource < amount {
            return Err(StaffError::InsufficientResourcesError);
        }

        self.resource -= amount;
        employee.borrow_mut().recieve_resource(amount)?;
        Ok(())
    }
}

impl Employee for Manager {
    fn set_supervisor(&mut self, sv: SupervisorRef) -> Result<()> {
        self.sv = Some(sv);
        Ok(())
    }

    fn on_layoff(&mut self) -> Result<()> {
        if self.sv.is_none() {
            return Ok(());
        }

        let sv = match self.sv.as_ref().unwrap().upgrade() {
            Some(s) => s,
            None => return Err(StaffError::EmployeeHasNoSupervisor(self.id)),
        };

        for (_, employee) in self.subordinates.iter() {
            employee
                .borrow_mut()
                .set_supervisor(self.sv.as_ref().unwrap().clone())?;
            sv.borrow_mut().hire(employee.clone())?;
        }

        sv.borrow_mut().recieve_resource(self.resource)?;
        self.resource = 0;

        Ok(())
    }
}
