use crate::errors::StaffError;
use crate::staff::{ceo::CEO, manager::Manager, worker::Worker};
use crate::traits::StaffEntity;
use uuid::Uuid;

/// A type alias which is used for representing resource type in company hierarchy
pub type Resource = u64;

pub type Result<T> = std::result::Result<T, StaffError>;

pub enum Staff {
    Ceo(CEO),
    Manager(Manager),
    Employee(Worker),
}
impl StaffEntity for Staff {
    fn get_id(&self) -> Uuid {
        match self {
            Staff::Ceo(ceo) => ceo.get_id(),
            Staff::Manager(m) => m.get_id(),
            Staff::Employee(e) => e.get_id(),
        }
    }

    fn get_resource_amount(&self) -> Result<Resource> {
        match self {
            Staff::Ceo(ceo) => ceo.get_resource_amount(),
            Staff::Manager(m) => m.get_resource_amount(),
            Staff::Employee(e) => e.get_resource_amount(),
        }
    }

    fn spend(&mut self, amount: Resource) -> Result<Resource> {
        match self {
            Staff::Ceo(ceo) => ceo.spend(amount),
            Staff::Manager(m) => m.spend(amount),
            Staff::Employee(e) => e.spend(amount),
        }
    }

    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource> {
        match self {
            Staff::Ceo(ceo) => ceo.send_resource(to, amount),
            Staff::Manager(m) => m.send_resource(to, amount),
            Staff::Employee(e) => e.send_resource(to, amount),
        }
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<Resource> {
        match self {
            Staff::Ceo(ceo) => ceo.recieve_resource(amount),
            Staff::Manager(m) => m.recieve_resource(amount),
            Staff::Employee(e) => e.recieve_resource(amount),
        }
    }
}
