use crate::errors::StaffError;
use crate::traits::StaffEntity;
use crate::types::Result;
use num_traits::{Num, Zero};
use std::collections::HashMap;

pub struct Company<V, R>
where
    V: StaffEntity,
    R: Num + Copy + Zero,
{
    ceo_id: V::ID,
    staff: HashMap<V::ID, V>,
    resources: HashMap<V::ID, R>,
}

impl<V: StaffEntity, R: Num + Copy + Zero> Company<V, R> {
    pub fn new(ceo: V) -> Self {
        Self {
            ceo_id: ceo.get_id(),
            resources: HashMap::from([(ceo.get_id(), R::zero())]),
            staff: HashMap::from([(ceo.get_id(), ceo)]),
        }
    }

    pub fn get_ceo(&self) -> &V {
        self.staff.get(&self.ceo_id).unwrap()
    }

    pub fn mint_resource(&mut self, amount: R) {
        let _ = self.resources.get_mut(&self.ceo_id).unwrap().add(amount);
    }

    pub fn hire(&mut self, staff: V) -> Result<(), V::ID, R> {
        if self.staff.contains_key(&staff.get_id()) {
            return Err(StaffError::StaffAlreadyExists { id: staff.get_id() });
        }

        self.staff.insert(staff.get_id(), staff);
        Ok(())
    }
}
