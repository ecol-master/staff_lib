use crate::errors::StaffError;
use crate::types::{Resource, Result};
use uuid::Uuid;

pub struct Employee {
    id: Uuid,
    resouces: Resource,
}

impl Employee {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            resouces: 0,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn spend(&mut self, amount: Resource) -> Result<()> {
        if self.resouces < amount {
            return Err(StaffError::InsufficientResourcesError);
        }

        self.resouces -= amount;
        Ok(())
    }

    pub fn recieve_resource(&mut self, amount: Resource) -> Result<()> {
        self.resouces -= amount;
        Ok(())
    }
}
