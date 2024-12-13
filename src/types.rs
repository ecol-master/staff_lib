use crate::companies::google::Google;
use crate::errors::StaffError;
use crate::staff::{ceo::CEO, manager::Manager, worker::Worker};
use crate::traits::{CompanyBehaviour, StaffEntity};
use std::collections::HashSet;
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

pub enum Company {
    Google(Google),
}

impl CompanyBehaviour for Company {
    fn set_ceo(&mut self, ceo: Staff) -> Result<()> {
        match self {
            Company::Google(g) => g.set_ceo(ceo),
        }
    }

    fn hire(&mut self, staff_entity: Staff, supervisor_id: Uuid) -> Result<Uuid> {
        match self {
            Company::Google(g) => g.hire(staff_entity, supervisor_id),
        }
    }

    fn layoff(&mut self, staff_id: Uuid, supervisor_id: Uuid) -> Result<Staff> {
        match self {
            Company::Google(g) => g.layoff(staff_id, supervisor_id),
        }
    }

    fn transfer_resources(&mut self, from: Uuid, to: Uuid, amount: Resource) -> Result<Resource> {
        match self {
            Company::Google(g) => g.transfer_resources(from, to, amount),
        }
    }

    fn get_supervisor_id(&self, staff_id: Uuid) -> Option<Uuid> {
        match self {
            Company::Google(g) => g.get_supervisor_id(staff_id),
        }
    }

    fn get_subordinates(&self, supervisor_id: Uuid) -> Option<HashSet<Uuid>> {
        match self {
            Company::Google(g) => g.get_subordinates(supervisor_id),
        }
    }

    fn get_resource_amount(&self, staff_id: Uuid) -> Result<Resource> {
        match self {
            Company::Google(g) => g.get_resource_amount(staff_id),
        }
    }

    fn spend_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource> {
        match self {
            Company::Google(g) => g.spend_resource(staff_id, amount),
        }
    }

    fn recieve_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource> {
        match self {
            Company::Google(g) => g.recieve_resource(staff_id, amount),
        }
    }
}
