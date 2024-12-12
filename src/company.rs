use crate::errors::StaffError;
use crate::staff::{ceo::CEO, manager::Manager, worker::Worker};
use crate::traits::{StaffEntity, Supervisor};
use crate::types::{Resource, Result};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
use uuid::Uuid;

pub struct Company {
    ceo: Option<Rc<RefCell<CEO>>>,
    managers: HashMap<Uuid, (Rc<RefCell<Manager>>, Uuid)>,
    employes: HashMap<Uuid, (Rc<RefCell<Worker>>, Uuid)>,
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

    pub fn get_supervisor_id(&self, staff_entity_id: Uuid) -> Option<Uuid> {
        if let Some(m) = self.managers.get(&staff_entity_id) {
            return Some(m.1);
        }

        if let Some(e) = self.employes.get(&staff_entity_id) {
            return Some(e.1);
        }

        None
    }

    /// Use with methods with `hire_...` methods in objects which implement [`crate::traits::Supervisor`]
    pub fn add_manager(
        &mut self,
        manager: Rc<RefCell<Manager>>,
        supervisor_id: Uuid,
    ) -> Result<Rc<RefCell<Manager>>> {
        let id = manager.as_ref().borrow().get_id();
        if self.managers.contains_key(&id) {
            return Err(StaffError::EmployeeAlreadyExists(id));
        }

        self.managers.insert(id, (manager.clone(), supervisor_id));
        Ok(manager)
    }

    pub fn add_employee(
        &mut self,
        employee: Rc<RefCell<Worker>>,
        supervisor_id: Uuid,
    ) -> Result<Rc<RefCell<Worker>>> {
        let id = employee.as_ref().borrow().get_id();

        if self.employes.contains_key(&id) {
            return Err(StaffError::EmployeeAlreadyExists(id));
        }

        self.employes.insert(id, (employee.clone(), supervisor_id));
        Ok(employee)
    }

    fn update_subordinates_supervisor(
        &mut self,
        subordinates: HashSet<Uuid>,
        supervisor_id: Uuid,
    ) -> Result<()> {
        for id in subordinates {
            if let Some(m) = self.managers.get_mut(&id) {
                m.1 = supervisor_id;
                continue;
            } else if let Some(e) = self.employes.get_mut(&id) {
                e.1 = supervisor_id;
                continue;
            } else {
                return Err(StaffError::EmployeeNotFound(id));
            }
        }

        Ok(())
    }

    ///
    pub fn layoff_manager(
        &mut self,
        manager_id: Uuid,
        supervisor_id: Uuid,
    ) -> Result<Rc<RefCell<Manager>>> {
        let manager = match self.managers.get(&manager_id) {
            Some(m) => m.0.clone(),
            None => return Err(StaffError::EmployeeNotFound(manager_id)),
        };

        if self.ceo.is_some() && self.ceo.as_ref().unwrap().borrow().get_id() == supervisor_id {
            return self.layoff_manager_by_sv(manager, self.ceo.as_ref().unwrap().clone());
        }

        let supervisor = match self.managers.get_mut(&manager_id) {
            Some(m) => m.clone().0,
            None => return Err(StaffError::EmployeeNotFound(manager_id)),
        };

        self.layoff_manager_by_sv(manager, supervisor)
    }

    fn layoff_manager_by_sv<T>(
        &mut self,
        manager: Rc<RefCell<Manager>>,
        sv: Rc<RefCell<T>>,
    ) -> Result<Rc<RefCell<Manager>>>
    where
        T: Supervisor + StaffEntity,
    {
        self.transfer_resources(
            manager.clone(),
            sv.clone(),
            manager.borrow().get_resource_amount(),
        )?;
        let subordinates = sv.borrow_mut().release_subordinates();

        sv.as_ref()
            .borrow_mut()
            .assume_subordinates(subordinates.clone());
        self.update_subordinates_supervisor(subordinates, sv.as_ref().borrow().get_id())?;

        Ok(manager)
    }

    pub fn layoff_worker(
        &mut self,
        worker_id: Uuid,
        supervisor_id: Uuid,
    ) -> Result<Rc<RefCell<Worker>>> {
        let worker = match self.employes.get(&worker_id) {
            Some(m) => m.0.clone(),
            None => return Err(StaffError::EmployeeNotFound(worker_id)),
        };

        if self.ceo.is_some() && self.ceo.as_ref().unwrap().borrow().get_id() == supervisor_id {
            return self.layoff_worker_by_sv(worker, self.ceo.as_ref().unwrap().clone());
        }

        let supervisor = match self.managers.get_mut(&worker_id) {
            Some(m) => m.clone().0,
            None => return Err(StaffError::EmployeeNotFound(worker_id)),
        };

        self.layoff_worker_by_sv(worker, supervisor)
    }

    fn layoff_worker_by_sv<T>(
        &mut self,
        worker: Rc<RefCell<Worker>>,
        sv: Rc<RefCell<T>>,
    ) -> Result<Rc<RefCell<Worker>>>
    where
        T: StaffEntity,
    {
        self.transfer_resources(
            worker.clone(),
            sv.clone(),
            worker.borrow().get_resource_amount(),
        )?;

        Ok(worker)
    }

    pub fn transfer(&self, from: Uuid, to: Uuid, amount: Resource) -> Result<Resource> {
        if let Some(m) = self.managers.get(&from) {
            return self.transfer_to(m.clone().0, to, amount);
        } else if let Some(e) = self.employes.get(&from) {
            return self.transfer_to(e.clone().0, to, amount);
        } else if let Some(ceo) = self.ceo.as_ref() {
            if ceo.borrow().get_id() == from {
                return self.transfer_to(ceo.clone(), to, amount);
            }
        }

        return Err(StaffError::EmployeeNotFound(to));
    }

    fn transfer_to<F>(&self, from: Rc<RefCell<F>>, to: Uuid, amount: Resource) -> Result<Resource>
    where
        F: StaffEntity,
    {
        if let Some(m) = self.managers.get(&to) {
            return self.transfer_resources(from, m.clone().0, amount);
        } else if let Some(e) = self.employes.get(&to) {
            return self.transfer_resources(from, e.clone().0, amount);
        } else if let Some(ceo) = self.ceo.as_ref() {
            if ceo.borrow().get_id() == to {
                return self.transfer_resources(from, ceo.clone(), amount);
            }
        }

        return Err(StaffError::EmployeeNotFound(to));
    }

    /// Method transfer resources between staff entities in company
    fn transfer_resources<F, T>(
        &self,
        from: Rc<RefCell<F>>,
        to: Rc<RefCell<T>>,
        amount: Resource,
    ) -> Result<Resource>
    where
        F: StaffEntity,
        T: StaffEntity,
    {
        from.as_ref().borrow_mut().spend(amount)?;
        to.as_ref().borrow_mut().recieve_resource(amount)?;
        Ok(amount)
    }
}
