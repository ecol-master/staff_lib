use crate::errors::StaffError;
use crate::traits::{Employee, StaffEntity, SupervisorRef};
use crate::types::{Resource, Result};
use uuid::Uuid;

pub struct Worker {
    id: Uuid,
    resouces: Resource,
    sv: Option<SupervisorRef>,
}

impl StaffEntity for Worker {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn spend(&mut self, amount: Resource) -> Result<()> {
        if self.resouces < amount {
            return Err(StaffError::InsufficientResourcesError);
        }

        self.resouces -= amount;
        Ok(())
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<()> {
        self.resouces -= amount;
        Ok(())
    }
}

impl Employee for Worker {
    fn set_supervisor(&mut self, sv: SupervisorRef) -> Result<()> {
        self.sv = Some(sv);
        Ok(())
    }

    fn on_layoff(&mut self) -> Result<()> {
        if self.sv.is_none() {
            return Err(StaffError::EmployeeHasNoSupervisor(self.id));
        }

        let sv = match self.sv.as_ref().unwrap().upgrade() {
            Some(s) => s,
            None => return Err(StaffError::EmployeeHasNoSupervisor(self.id)),
        };

        sv.borrow_mut().recieve_resource(self.resouces)?;
        self.resouces = 0;

        Ok(())
    }
}
