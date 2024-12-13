use crate::companies::google::Google;
use crate::errors::StaffError;
use crate::staff::{ceo::CEO, manager::Manager, worker::Worker};
use crate::traits::{CompanyBehaviour, StaffEntity};
use std::collections::HashSet;
use uuid::Uuid;

/// A type alias which is used for representing resource type in company hierarchy
pub type Resource = u64;

/// A wrapper for standart library [`std::result::Result`] with templated value and fixed error
/// type [`crate::errors::StaffError`]
pub type Result<T> = std::result::Result<T, StaffError>;

#[derive(Clone, Debug)]
pub enum Staff {
    Ceo(CEO),
    Manager(Manager),
    Employee(Worker),
}

impl Staff {
    fn delegate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&dyn StaffEntity) -> T,
    {
        match self {
            Staff::Ceo(ceo) => f(ceo),
            Staff::Manager(manager) => f(manager),
            Staff::Employee(worker) => f(worker),
        }
    }

    fn delegate_mut<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut dyn StaffEntity) -> T,
    {
        match self {
            Staff::Ceo(ceo) => f(ceo),
            Staff::Manager(manager) => f(manager),
            Staff::Employee(worker) => f(worker),
        }
    }
}

impl StaffEntity for Staff {
    fn get_id(&self) -> Uuid {
        self.delegate(|entity| entity.get_id())
    }

    fn get_resource_amount(&self) -> Result<Resource> {
        self.delegate(|entity| entity.get_resource_amount())
    }

    fn spend(&mut self, amount: Resource) -> Result<Resource> {
        self.delegate_mut(|entity| entity.spend(amount))
    }

    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource> {
        self.delegate_mut(|entity| entity.send_resource(to, amount))
    }

    fn recieve_resource(&mut self, amount: Resource) -> Result<Resource> {
        self.delegate_mut(|entity| entity.recieve_resource(amount))
    }
}

#[derive(Debug)]
pub enum Company {
    Google(Google),
}

impl Company {
    fn delegate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&dyn CompanyBehaviour) -> T,
    {
        match self {
            Company::Google(g) => f(g),
        }
    }

    fn delegate_mut<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut dyn CompanyBehaviour) -> T,
    {
        match self {
            Company::Google(g) => f(g),
        }
    }
}

impl CompanyBehaviour for Company {
    fn set_ceo(&mut self, ceo: Staff) {
        self.delegate_mut(|company| company.set_ceo(ceo))
    }

    fn hire(&mut self, staff_entity: Staff, supervisor_id: Uuid) -> Result<Uuid> {
        self.delegate_mut(|company| company.hire(staff_entity, supervisor_id))
    }

    fn layoff(&mut self, staff_id: Uuid, supervisor_id: Uuid) -> Result<Staff> {
        self.delegate_mut(|company| company.layoff(staff_id, supervisor_id))
    }

    fn transfer_resources(&mut self, from: Uuid, to: Uuid, amount: Resource) -> Result<Resource> {
        self.delegate_mut(|company| company.transfer_resources(from, to, amount))
    }

    fn get_staff_by_id(&mut self, staff_id: Uuid) -> Option<Staff> {
        self.delegate_mut(|company| company.get_staff_by_id(staff_id))
    }

    fn get_supervisor_id(&self, staff_id: Uuid) -> Option<Uuid> {
        self.delegate(|company| company.get_supervisor_id(staff_id))
    }

    fn get_subordinates(&self, supervisor_id: Uuid) -> Option<HashSet<Uuid>> {
        self.delegate(|company| company.get_subordinates(supervisor_id))
    }

    fn get_resource_amount(&self, staff_id: Uuid) -> Result<Resource> {
        self.delegate(|company| company.get_resource_amount(staff_id))
    }

    fn spend_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource> {
        self.delegate_mut(|company| company.spend_resource(staff_id, amount))
    }

    fn recieve_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource> {
        self.delegate_mut(|company| company.recieve_resource(staff_id, amount))
    }
}
