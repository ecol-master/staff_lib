use crate::errors::Error;
use crate::traits::StaffEntity;
use core::fmt::Debug;
use num_traits::{FromPrimitive, Num, Zero};
use std::collections::{HashMap, HashSet};
use std::ops::Div;

/// Struct [`Company`] orginize the relations between company members and store its resources
pub struct Company<V, R>
where
    V: StaffEntity,
    R: Num + Copy + Zero + PartialOrd + Div<Output = R> + FromPrimitive + Debug,
{
    ceo_id: V::ID,

    // Mapping from staff ID to its Object
    staff: HashMap<V::ID, V>,

    // Mapping from staff ID to its balances
    resources: HashMap<V::ID, R>,

    // Mapping form subordinate ID to supervisor ID.
    // Follow princip: only one supervisor for subordinate
    supervisors: HashMap<V::ID, V::ID>,

    // Mapping from superviso's ID to list of it's subordinates.
    subordinates: HashMap<V::ID, HashSet<V::ID>>,
}

impl<
        V: StaffEntity,
        R: Num + Copy + Zero + PartialOrd + FromPrimitive + Div<Output = R> + Debug,
    > Company<V, R>
{
    pub fn new(ceo: V) -> Self {
        Self {
            ceo_id: ceo.get_id(),
            resources: HashMap::from([(ceo.get_id(), R::zero())]),
            staff: HashMap::from([(ceo.get_id(), ceo)]),
            supervisors: HashMap::new(),
            subordinates: HashMap::new(),
        }
    }

    pub fn get_ceo(&self) -> Option<&V> {
        self.staff.get(&self.ceo_id)
    }

    pub fn get_ceo_mut(&mut self) -> Option<&mut V> {
        self.staff.get_mut(&self.ceo_id)
    }

    pub fn get_all_staff(&self) -> Vec<V::ID> {
        self.staff.keys().cloned().collect()
    }

    pub fn get_staff(&self, staff_id: &V::ID) -> Option<&V> {
        self.staff.get(staff_id)
    }

    pub fn get_staff_mut(&mut self, staff_id: &V::ID) -> Option<&mut V> {
        self.staff.get_mut(staff_id)
    }

    pub fn get_resource(&self, staff_id: &V::ID) -> Option<&R> {
        self.resources.get(staff_id)
    }

    pub fn get_supervisor(&self, staff_id: &V::ID) -> Option<&V::ID> {
        self.supervisors.get(staff_id)
    }

    pub fn get_subordinates(&self, staff_id: &V::ID) -> Option<&HashSet<V::ID>> {
        self.subordinates.get(staff_id)
    }

    pub fn mint(&mut self, amount: R) {
        self.resources
            .entry(self.ceo_id.clone())
            .and_modify(|res| *res = *res + amount)
            .or_insert(amount);
    }

    pub fn withdraw(&mut self, staff_id: &V::ID, amount: R) -> Result<(), Error<V::ID, R>> {
        let resource = self
            .resources
            .get_mut(staff_id)
            .ok_or(Error::StaffNotFound {
                id: staff_id.clone(),
            })?;

        if *resource < amount {
            return Err(Error::InsufficientResourcesError {
                id: staff_id.clone(),
                available: *resource,
                required: amount,
            });
        }

        *resource = *resource - amount;
        Ok(())
    }

    /// Method add new staff into company, set for it a supervisor and give him a 10% of
    /// supervisors's resources
    pub fn hire(&mut self, staff: V, supervisor_id: &V::ID) -> Result<V::ID, Error<V::ID, R>> {
        self.staff_exists(supervisor_id)?;

        if self.staff.contains_key(&staff.get_id()) {
            return Err(Error::StaffAlreadyExists { id: staff.get_id() });
        }

        let staff_id = staff.get_id();
        self.staff.insert(staff_id.clone(), staff);
        self.supervisors
            .insert(staff_id.clone(), supervisor_id.clone());

        if let Some(s) = self.subordinates.get_mut(supervisor_id) {
            s.insert(staff_id.clone());
        } else {
            self.subordinates
                .insert(supervisor_id.clone(), HashSet::from([staff_id.clone()]));
        }

        let amount = *self.get_resource(supervisor_id).unwrap() / R::from_i16(10).unwrap();
        self.withdraw(supervisor_id, amount)?;
        self.resources.insert(staff_id.clone(), amount);
        Ok(staff_id)
    }

    pub fn fire(&mut self, staff_id: &V::ID) -> Result<V, Error<V::ID, R>> {
        self.staff_exists(staff_id)?;

        if *staff_id == self.ceo_id {
            return Err(Error::CannotFireCeo);
        }

        let supervisor_id = self.get_supervisor(staff_id).unwrap().clone();
        self.supervisors.remove(staff_id);

        let resource = self.get_resource(staff_id).unwrap();
        self.transfer(staff_id, &supervisor_id, *resource)?;
        self.resources.remove(staff_id);

        // transfer subordinates
        if let Some(subordinates) = self.subordinates.remove(staff_id) {
            for id in subordinates.iter() {
                self.supervisors.insert(id.clone(), supervisor_id.clone());
            }

            self.subordinates
                .entry(supervisor_id.clone())
                .or_default()
                .extend(subordinates);
        }

        Ok(self.staff.remove(staff_id).unwrap())
    }

    pub fn transfer(&mut self, from: &V::ID, to: &V::ID, amount: R) -> Result<(), Error<V::ID, R>> {
        if !self.staff.contains_key(from) {
            return Err(Error::StaffNotFound { id: from.clone() });
        }

        if !self.staff.contains_key(to) {
            return Err(Error::StaffNotFound { id: to.clone() });
        }

        self.withdraw(from, amount)?;
        self.resources
            .entry(self.ceo_id.clone())
            .and_modify(|res| *res = *res + amount)
            .or_insert(amount);
        Ok(())
    }

    /// private methods
    fn staff_exists(&self, staff_id: &V::ID) -> Result<(), Error<V::ID, R>> {
        self.get_staff(staff_id).ok_or(Error::StaffNotFound {
            id: staff_id.clone(),
        })?;
        Ok(())
    }
}
