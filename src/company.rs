use crate::errors::Error;
use crate::traits::StaffEntity;
use core::fmt::Debug;
use num_traits::{FromPrimitive, Num, Zero};
use std::collections::{HashMap, HashSet};
use std::ops::Div;

/// The `Company` struct organizes relations between company members and manages resources.
///
/// # Generics
/// - `V`: A type that implements the [`StaffEntity`] trait, representing a company staff member.
/// - `R`: A numeric type that implements [`Num`], [`Copy`], [`Zero`], [`PartialOrd`], [`Div`], [`FromPrimitive`],
///        and [`Debug`]. It is used to represent company funds.
///
/// # Fields
/// - `ceo_id`: ID of the company's CEO.
/// - `staff`: A mapping from staff IDs to their staff.
/// - `resources`: A mapping of staff IDs to their resources.
/// - `supervisors`: A mapping from subordinate IDs to their supervisor IDs.
/// - `subordinates`: A mapping from supervisor IDs to sets of their subordinates' IDs.
///
/// # Usage
/// The `Company` struct provides methods to hire and fire employees, manage their resources,
/// and handle hierarchical relationships (supervisors and subordinates).
pub struct Company<V, R>
where
    V: StaffEntity,
    R: Num + Copy + Zero + PartialOrd + Div<Output = R> + FromPrimitive + Debug,
{
    ceo_id: V::ID,
    staff: HashMap<V::ID, V>,
    resources: HashMap<V::ID, R>,
    supervisors: HashMap<V::ID, V::ID>,
    subordinates: HashMap<V::ID, HashSet<V::ID>>,
}

impl<
        V: StaffEntity,
        R: Num + Copy + Zero + PartialOrd + FromPrimitive + Div<Output = R> + Debug,
    > Company<V, R>
{
    /// Creates a new company with the given CEO.
    pub fn new(ceo: V) -> Self {
        Self {
            ceo_id: ceo.get_id(),
            resources: HashMap::from([(ceo.get_id(), R::zero())]),
            staff: HashMap::from([(ceo.get_id(), ceo)]),
            supervisors: HashMap::new(),
            subordinates: HashMap::new(),
        }
    }

    /// Returns a reference to the CEO of the company.
    pub fn ceo(&self) -> Option<&V> {
        self.staff.get(&self.ceo_id)
    }

    /// Returns a mutable reference to the CEO of the company.
    pub fn ceo_mut(&mut self) -> Option<&mut V> {
        self.staff.get_mut(&self.ceo_id)
    }

    /// Returns a list of IDs of all staff members in the company.
    pub fn get_all_staff(&self) -> Vec<V::ID> {
        self.staff.keys().cloned().collect()
    }

    /// Returns a reference to a staff object by ID.
    pub fn get(&self, staff_id: &V::ID) -> Option<&V> {
        self.staff.get(staff_id)
    }

    /// Returns a reference to a staff member by ID.
    ///
    /// # Returns
    /// - `Some(&V)` if the staff member exists.
    /// - `None` otherwise.
    pub fn get_mut(&mut self, staff_id: &V::ID) -> Option<&mut V> {
        self.staff.get_mut(staff_id)
    }

    /// Returns the resource (balance) of a staff member.
    ///
    /// # Returns
    /// - `Some(&R)` if the staff member exists.
    /// - `None` otherwise.
    pub fn resource(&self, staff_id: &V::ID) -> Option<&R> {
        self.resources.get(staff_id)
    }

    /// Returns the supervisor's ID of the given staff member.
    ///
    /// # Returns
    /// - `Some(&V::ID)`
    /// - `None` - for CEO ot if staff member dosn't exists
    pub fn supervisor(&self, staff_id: &V::ID) -> Option<&V::ID> {
        self.supervisors.get(staff_id)
    }

    pub fn subordinates(&self, staff_id: &V::ID) -> Option<&HashSet<V::ID>> {
        self.subordinates.get(staff_id)
    }

    /// Mints (creates) resources and add them to the CEO's balance.
    ///
    /// # Arguments
    /// - `amount`: The amount of resources to mint.
    pub fn mint(&mut self, amount: R) {
        self.resources
            .entry(self.ceo_id.clone())
            .and_modify(|res| *res = *res + amount)
            .or_insert(amount);
    }

    /// Withdraws resources from a staff member's balance.
    ///
    /// # Arguments
    /// - `staff_id`: The ID of the staff member.
    /// - `amount`: The amount to withdraw.
    ///
    /// # Errors
    /// - [`Error::StaffNotFound`] if the staff member does not exist.
    /// - [`Error::InsufficientResourcesError`] if the staff member has insufficient resources.
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

    /// Hires a new staff member under a supervisor and give it a `10%` of the supervisor's resources.
    ///
    /// # Arguments
    /// - `staff`: The new staff member.
    /// - `supervisor_id`: The ID of the supervisor.
    ///
    /// # Returns
    /// - The ID of the newly hired staff member.
    ///
    /// # Errors
    /// - [`Error::StaffNotFound`] if the supervisor does not exist.
    /// - [`Error::StaffAlreadyExists`] if the new staff member already exists.
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

        let amount = *self.resource(supervisor_id).unwrap() / R::from_i16(10).unwrap();
        self.withdraw(supervisor_id, amount)?;
        self.resources.insert(staff_id.clone(), amount);
        Ok(staff_id)
    }

    /// Fires a staff member, transferring their resources to the supervisor and removes all data
    /// associated with current member.
    ///
    /// # Arguments
    /// - `staff_id`: The ID of the staff member to fire.
    ///
    /// # Returns
    /// - The fired staff member.
    ///
    /// # Errors
    /// - [`Error::StaffNotFound`] if the staff member does not exist.
    /// - [`Error::CannotFireCeo`] if attempting to fire the CEO.
    pub fn fire(&mut self, staff_id: &V::ID) -> Result<V, Error<V::ID, R>> {
        self.staff_exists(staff_id)?;

        if *staff_id == self.ceo_id {
            return Err(Error::CannotFireCeo);
        }

        let supervisor_id = self.supervisor(staff_id).unwrap().clone();
        self.supervisors.remove(staff_id);

        let resource = self.resource(staff_id).unwrap();
        self.transfer(staff_id, &supervisor_id, *resource)?;
        self.resources.remove(staff_id);

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

    /// Transfers resources from one staff member to another.
    ///
    /// # Arguments
    /// - `from`: The ID of the sender.
    /// - `to`: The ID of the receiver.
    /// - `amount`: The amount of resources to transfer.
    ///
    /// # Errors
    /// - [`Error::StaffNotFound`] if either staff member does not exist.
    /// - [`Error::InsufficientResourcesError`] if the sender has insufficient resources.
    pub fn transfer(&mut self, from: &V::ID, to: &V::ID, amount: R) -> Result<(), Error<V::ID, R>> {
        if !self.staff.contains_key(from) {
            return Err(Error::StaffNotFound { id: from.clone() });
        }

        if !self.staff.contains_key(to) {
            return Err(Error::StaffNotFound { id: to.clone() });
        }

        self.withdraw(from, amount)?;
        self.resources
            .entry(to.clone())
            .and_modify(|res| *res = *res + amount)
            .or_insert(amount);
        Ok(())
    }

    /// private methods
    fn staff_exists(&self, staff_id: &V::ID) -> Result<(), Error<V::ID, R>> {
        self.get(staff_id).ok_or(Error::StaffNotFound {
            id: staff_id.clone(),
        })?;
        Ok(())
    }
}
