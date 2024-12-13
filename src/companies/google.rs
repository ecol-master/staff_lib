use crate::errors::StaffError;
use crate::traits::{CompanyBehaviour, StaffEntity};
use crate::types::{Resource, Result, Staff};
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;

pub struct Google {
    ceo_id: Option<Uuid>,
    staff: HashMap<Uuid, (Staff, Uuid)>,
    subordinates: HashMap<Uuid, HashSet<Uuid>>,
    resources: HashMap<Uuid, Resource>,
}

impl Default for Google {
    fn default() -> Self {
        Self::new()
    }
}

impl Google {
    pub fn new() -> Self {
        Self {
            ceo_id: None,
            staff: HashMap::new(),
            subordinates: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    /// Private methods
    fn is_supervisor_for(&self, supervisor_id: Uuid, staff_id: Uuid) -> Result<()> {
        if self.ceo_id.is_some() && supervisor_id == self.ceo_id.unwrap() {
            Ok(())
        } else if self.subordinates.contains_key(&supervisor_id)
            && self
                .subordinates
                .get(&supervisor_id)
                .unwrap()
                .contains(&staff_id)
        {
            Ok(())
        } else {
            Err(StaffError::StaffHasNoPermission(supervisor_id, staff_id))
        }
    }

    fn add_subordinate(&mut self, supervisor_id: Uuid, staff_id: Uuid) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.subordinates.entry(supervisor_id)
        {
            e.insert(HashSet::from([staff_id]));
        } else {
            self.subordinates
                .get_mut(&supervisor_id)
                .unwrap()
                .insert(staff_id);
        }
    }

    fn move_subordinates(&mut self, supervisor_id: Uuid, staff_id: Uuid) -> Result<()> {
        if !self.subordinates.contains_key(&staff_id) {
            return Ok(());
        }

        let subordinates = std::mem::take(self.subordinates.get_mut(&staff_id).unwrap());

        for id in subordinates.clone() {
            if let Some(s) = self.staff.get_mut(&id) {
                s.1 = supervisor_id;
            }
        }

        self.subordinates
            .get_mut(&supervisor_id)
            .unwrap()
            .extend(subordinates);
        Ok(())
    }

    fn staff_not_exists(&self, staff_id: Uuid) -> Result<()> {
        if self.staff.contains_key(&staff_id) {
            Err(StaffError::StaffAlreadyExists(staff_id))
        } else {
            Ok(())
        }
    }

    fn staff_exists(&self, staff_id: Uuid) -> Result<()> {
        if self.staff.contains_key(&staff_id) {
            Ok(())
        } else {
            Err(StaffError::StaffNotFound(staff_id))
        }
    }
}

impl CompanyBehaviour for Google {
    fn set_ceo(&mut self, ceo: Staff) -> Result<()> {
        let ceo_id = ceo.get_id();

        self.ceo_id = Some(ceo_id);
        self.staff.insert(ceo_id, (ceo, ceo_id));
        self.resources.insert(ceo_id, 0);
        Ok(())
    }

    fn get_resource_amount(&self, staff_id: Uuid) -> Result<Resource> {
        match self.resources.get(&staff_id) {
            Some(r) => Ok(*r),
            None => Err(StaffError::StaffNotFound(staff_id)),
        }
    }

    fn spend_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource> {
        if !self.resources.contains_key(&staff_id)
            || *self.resources.get(&staff_id).unwrap() < amount
        {
            return Err(StaffError::InsufficientResourcesError(staff_id));
        }

        *self.resources.get_mut(&staff_id).unwrap() -= amount;
        Ok(amount)
    }

    fn recieve_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource> {
        match self.resources.get_mut(&staff_id) {
            Some(r) => {
                *r += amount;
                Ok(amount)
            }
            None => Err(StaffError::StaffNotFound(staff_id)),
        }
    }

    fn get_supervisor_id(&self, staff_id: Uuid) -> Option<Uuid> {
        self.staff.get(&staff_id).map(|s| s.1)
    }

    fn get_subordinates(&self, supervisor_id: Uuid) -> Option<HashSet<Uuid>> {
        self.subordinates.get(&supervisor_id).cloned()
    }

    fn hire(&mut self, staff_entity: Staff, supervisor_id: Uuid) -> Result<Uuid> {
        let id = staff_entity.get_id();
        self.staff_not_exists(id)?;

        self.staff.insert(id, (staff_entity, supervisor_id));
        self.add_subordinate(supervisor_id, id);
        self.resources.insert(id, 0);
        Ok(id)
    }

    fn layoff(&mut self, staff_id: Uuid, supervisor_id: Uuid) -> Result<Staff> {
        self.is_supervisor_for(supervisor_id, staff_id)?;

        let resource_transfer = self.get_resource_amount(staff_id)?;
        self.transfer_resources(staff_id, supervisor_id, resource_transfer)?;
        self.resources.remove(&staff_id);

        self.move_subordinates(supervisor_id, staff_id)?;
        self.subordinates.remove(&staff_id);

        Ok(self.staff.remove(&staff_id).unwrap().0)
    }

    fn transfer_resources(&mut self, from: Uuid, to: Uuid, amount: Resource) -> Result<Resource> {
        self.staff_exists(from)?;
        self.staff_exists(to)?;

        let transferred_amount: Resource = self.spend_resource(from, amount)?;

        match self.recieve_resource(to, transferred_amount) {
            Ok(amount) => Ok(amount),
            Err(e) => {
                self.recieve_resource(from, transferred_amount)?;
                Err(e)
            }
        }
    }
}
