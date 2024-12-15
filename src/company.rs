use crate::errors::StaffError;
use crate::traits::StaffEntity;
use num_traits::{FromPrimitive, Num, Zero};
use std::collections::{HashMap, HashSet};
use std::ops::Div;

/// Struct [`Company`] orginize the relations between company members and store its resources
pub struct Company<V, R>
where
    V: StaffEntity,
    R: Num + Copy + Zero + PartialOrd + Div<Output = R> + FromPrimitive,
{
    ceo_id: V::ID,

    // Mapping from staff ID to its Object
    staff: HashMap<V::ID, V>,

    // Mappin from staff ID to its balances
    resources: HashMap<V::ID, R>,

    // Mapping form subordinate ID to supervisor ID.
    // Mapping princip: only one supervisor for subordinate
    supervisors: HashMap<V::ID, V::ID>,

    // Mapping from superviso's ID to list of it's subordinates.
    subordinates: HashMap<V::ID, HashSet<V::ID>>,
}

impl<V: StaffEntity, R: Num + Copy + Zero + Div<Output = R> + PartialOrd + FromPrimitive>
    Company<V, R>
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

    pub fn get_ceo(&self) -> &V {
        self.staff.get(&self.ceo_id).unwrap()
    }

    pub fn get_ceo_mu(&mut self) -> &mut V {
        self.staff.get_mut(&self.ceo_id).unwrap()
    }

    pub fn get_staff(&self) -> Vec<V::ID> {
        self.staff.keys().cloned().collect()
    }

    pub fn get_resource(&self, staff_id: V::ID) -> Result<R, StaffError<V::ID, R>> {
        match self.resources.get(&staff_id) {
            Some(r) => Ok(*r),
            None => Err(StaffError::StaffNotFound { id: staff_id }),
        }
    }

    pub fn mint_resource(&mut self, amount: R) {
        if let Some(resource) = self.resources.get_mut(&self.ceo_id) {
            *resource = *resource + amount;
        };
    }

    /// Method add new staff into company, set for it a supervisor and give him a 10% of
    /// supervisors's resources
    pub fn hire(&mut self, staff: V, supervisor_id: V::ID) -> Result<(), StaffError<V::ID, R>> {
        self.staff_exists(supervisor_id.clone())?;

        if self.staff.contains_key(&staff.get_id()) {
            return Err(StaffError::StaffAlreadyExists { id: staff.get_id() });
        }

        let staff_id = staff.get_id();
        self.staff.insert(staff_id.clone(), staff);
        self.supervisors
            .insert(staff_id.clone(), supervisor_id.clone());

        if let Some(s) = self.subordinates.get_mut(&supervisor_id) {
            s.insert(staff_id.clone());
        } else {
            self.subordinates
                .insert(supervisor_id.clone(), HashSet::from([staff_id.clone()]));
        }

        let amount = self.get_resource(supervisor_id.clone())? / R::from_i16(10).unwrap();
        let _ = self.resources.get_mut(&supervisor_id).unwrap().sub(amount);
        self.resources.insert(staff_id, amount);
        Ok(())
    }

    pub fn fire(&mut self, staff_id: V::ID) -> Result<V, StaffError<V::ID, R>> {
        self.staff_exists(staff_id.clone())?;

        let supervisor: V::ID = self.get_supervisor(staff_id.clone())?;
        self.supervisors.remove(&staff_id);

        let resource = self.get_resource(staff_id.clone())?;
        self.transfer(staff_id.clone(), supervisor.clone(), resource)?;
        self.resources.remove(&staff_id);

        // transfer subordinates
        if let Some(subordinates) = self.subordinates.remove(&staff_id) {
            for subordinate_id in subordinates.iter() {
                // Обновляем начальника для каждого подчиненного
                self.supervisors
                    .insert(subordinate_id.clone(), supervisor.clone());
            }

            self.subordinates
                .entry(supervisor.clone())
                .or_insert_with(HashSet::new)
                .extend(subordinates);
        }

        Ok(self.staff.remove(&staff_id).unwrap())
    }

    pub fn transfer(
        &mut self,
        from: V::ID,
        to: V::ID,
        amount: R,
    ) -> Result<(), StaffError<V::ID, R>> {
        self.staff_exists(from.clone())?;
        self.staff_exists(to.clone())?;

        let resources = self.resources.get_mut(&from).unwrap();
        if *resources < amount {
            return Err(StaffError::InsufficientResourcesError {
                id: from.clone(),
                available: *resources,
                requied: amount,
            });
        }

        let _ = resources.sub(amount);
        let _ = self.resources.get_mut(&to).unwrap().add(amount);
        Ok(())
    }

    pub fn get_supervisor(&mut self, staff_id: V::ID) -> Result<V::ID, StaffError<V::ID, R>> {
        self.staff_exists(staff_id.clone())?;
        Ok(self.supervisors.get(&staff_id).unwrap().clone())
    }

    pub fn get_subordinates(
        &self,
        staff_id: V::ID,
    ) -> Result<Option<&HashSet<V::ID>>, StaffError<V::ID, R>> {
        self.staff_exists(staff_id.clone())?;
        Ok(self.subordinates.get(&staff_id))
    }

    /// Private function which is needed to check the existance of staff with concrete ID
    fn staff_exists(&self, staff_id: V::ID) -> Result<(), StaffError<V::ID, R>> {
        match self.staff.get(&staff_id) {
            Some(_) => Ok(()),
            None => Err(StaffError::StaffNotFound { id: staff_id }),
        }
    }
}
