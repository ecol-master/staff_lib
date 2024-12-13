//! Module defines traits for making hierarhy for staff managment system.
//! Defined traits:
//! - [`crate::traits::StaffEntity`]
//! - [`crate::traits::Employee`]
//! - [`crate::traits::Supervisor`],
//! - [`crate::traits::CompanyBehaviour`]

#![cfg_attr(doc, allow(unused_variables, unused_imports))]
use crate::types::Staff;
use crate::types::{Resource, Result};
use std::collections::HashSet;
use uuid::Uuid;

/// [`StaffEntity`] provides basic methods for company staff entity
pub trait StaffEntity {
    /// Method return the unique identifier of staff entity in company.
    ///
    /// # Example:
    /// ```
    /// let id: Uuid = staff_entity.get_id();
    /// println!("Staff ID: {}", id);
    /// ```
    fn get_id(&self) -> Uuid;

    /// Method returns the amount of resources which staff entity has.
    ///
    /// # Example:
    /// ```
    /// let resource = staff_entity.get_resource_amount()?;
    /// println!("Resource amount: {}$", resource);
    /// ```
    ///
    /// # Error
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] if object does not register in Company.
    fn get_resource_amount(&self) -> Result<Resource>;

    /// Method tries to spend the amount of resources from staff entitry balance.
    ///
    /// # Parameters
    /// - `amount`: The amount of resources to spend.
    ///
    /// # Returns
    /// - On success, returns the spended amount of resources.
    ///
    /// # Errors
    /// - Returns [`crate::errors::StaffError::InsufficientResourcesError`] if there
    ///     are not enough resources available.
    ///
    /// # Example
    /// ```
    /// let remaining = staff_entity.spend(50)?;
    /// println!("Spended resources: {}$", remaining);
    /// ```
    fn spend(&mut self, amount: Resource) -> Result<Resource>;

    /// Method sends an amount of resources to another staff entity from current balance by their
    /// [`uuid::Uuid`].
    ///
    /// # Parameters
    /// - `to`: The identifier of  staff entity to send resources.
    /// - `amount`: The amount of resources to send.
    ///
    /// # Returns
    /// - On success, returns the amount of resources sent.
    ///
    /// # Errors
    /// - Returns [`crate::errors::StaffError::StaffNotFound`] if the recipient is not registered in the company.
    /// - Returns [`crate::errors::StaffError::InsufficientResourcesError`] if there are not enough resources to send.
    ///
    /// # Example
    /// ```
    /// let staff_id = "...";
    /// let sent = staff_entity.send_resource(staff_id, 30)?;
    /// println!("Resources sent: {}$", sent);
    /// ```
    fn send_resource(&mut self, to: Uuid, amount: Resource) -> Result<Resource>;

    /// Method increase the  amount of resources in current staff entity balance.
    ///
    /// # Parameters
    /// - `amount`: The amount of resources to add to the staff entity.
    ///
    /// # Returns
    /// - On success, returns the new total amount of resources after receiving.
    ///
    /// # Example
    /// ```
    /// let new_total = staff_entity.recieve_resource(25)?;
    /// println!("New total resources: {}$", new_total);
    /// ```
    fn recieve_resource(&mut self, amount: Resource) -> Result<Resource>;
}

/// Trait [`Employee`] defines the methods fo
pub trait Employee {
    /// Returns the unique identifier of the employee's supervisor.
    ///
    /// If the employee does not have a supervisor, it will return `None`.
    ///
    /// # Example
    /// ```rust
    /// let worker = Worker::new(...);
    /// match worker.get_supervisor_id() {
    ///     Some(id) => println!("Supervisor ID: {}", id),
    ///     None => println!("No supervisor assigned"),
    /// }
    /// ```
    fn get_supervisor_id(&self) -> Option<Uuid>;
}

/// [`Supervisor`]
pub trait Supervisor {
    /// Hires a new staff member and returns their unique identifier.
    ///
    /// Method adds a new employee to the supervisor's subordinates set.
    ///
    /// # Example
    /// ```rust
    /// let mut supervisor = Manager::new();
    /// let new_employee = Staff::new();
    /// let employee_id = supervisor.hire(new_employee).unwrap();
    /// println!("Hired new employee with ID: {}", employee_id);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffAlreadyExists`] if staff_entity with the
    /// same id already exists.
    fn hire(&mut self, staff_entity: Staff) -> Result<Uuid>;

    /// Layoff an employee by its id and returns the `Staff` object.
    ///
    /// This method removes an employee from the company.
    ///
    /// # Example
    /// ```rust
    /// let mut supervisor = Supervisod::new();
    /// let employee_id = "...";
    /// let staff_entity = supervisor.layoff(employee_id).unwrap();
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] is employee not found in
    /// company or error [`crate::errors::StaffError::StaffHasNoPermission`] is current supervisor
    /// can not lay off the employee
    fn layoff(&mut self, staff_id: Uuid) -> Result<Staff>;

    /// Returns the set of subordinates identifiers.
    ///
    /// If the supervisor has no subordinates, it will return [`None`].
    ///
    /// # Example
    /// ```rust
    /// let supervisor = Manager::new(...);
    /// let subordinates = supervisor.get_subordinates().unwrap();
    /// for subordinate_id in subordinates {
    ///     println!("Subordinate ID: {}", subordinate_id);
    /// }
    /// ```
    fn get_subordinates(&self) -> Option<HashSet<Uuid>>;
}

/// [`CompanyBehaviour`] defines necessary methods for company's staff management and resource
/// managment
pub trait CompanyBehaviour {
    /// Sets the CEO of the company.
    ///
    /// # Example
    /// ```rust
    /// let mut company = Company::new();
    /// let ceo = CEO::new();
    /// company.set_ceo(ceo);
    /// ```
    fn set_ceo(&mut self, ceo: Staff);

    /// Hires a new employee and adds to a supervisor's subordinatess set.
    ///
    /// Returns the unique identifier of  hired employee.
    ///
    /// # Example
    /// ```rust
    /// let mut company = Company::new();
    /// let worker = Worker::new();
    /// let manager = Manager::new();
    /// let staff_id = company.hire(worker, manager.get_id()).unwrap();
    /// println!("Hired employee with ID: {}", employee_id);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] if the supervisor not found
    /// or staff entity with the same id already exists.
    fn hire(&mut self, staff_entity: Staff, supervisor_id: Uuid) -> Result<Uuid>;

    /// Lays off an employee and returns the employee object.
    ///
    /// The layoff operation is allowed only if the `supervisor_id` matches the CEO's ID
    /// or if the `staff_id` is found in the supervisor's set of subordinates.
    ///
    /// # Example
    /// ```rust
    /// let mut company = Company::new();
    /// let staff_id = Uuid::new_v4();
    /// let manager = Manager::new();
    /// let laid_off_staff = company.layoff(staff_id, manager.get_id()).unwrap();
    /// println!("Laid off employee: {:?}", laid_off_staff);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffHasNoPermission`] if the employee cannot be
    /// laid off by this supervisor or an error [`crate::errors::StaffError::StaffNotFound`] if the staff
    /// entity not found.
    fn layoff(&mut self, staff_id: Uuid, supervisor_id: Uuid) -> Result<Staff>;

    /// Transfers resources between two staff members in the company.
    ///
    /// The method works for any staff member in the company.
    ///
    /// # Example
    /// ```rust
    /// let mut company = Company::new();
    /// let from_id = "...";
    /// let to_id = "...";
    /// let amount = 100;
    /// let transferred_amount = company.transfer_resources(from_id, to_id, amount).unwrap();
    /// println!("Transferred amount: {}", transferred_amount);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] if one of the staff members does not exist.
    fn transfer_resources(&mut self, from: Uuid, to: Uuid, amount: Resource) -> Result<Resource>;

    /// Returns a option [`Staff`] object by its unique identifier.
    ///
    /// If the staff member is not registered in the company, this returns [`None`].
    ///
    /// # Example
    /// ```rust
    /// let company = Company::new();
    /// let staff_id = "...";
    /// let staff = company.get_staff_by_id(staff_id);
    /// match staff {
    ///     Some(staff_member) => println!("Found staff: {:?}", staff_member),
    ///     None => println!("Staff not found"),
    /// }
    /// ```
    fn get_staff_by_id(&mut self, staff_id: Uuid) -> Option<Staff>;

    /// Returns the supervisor's if for a staff entity.
    ///
    /// Returns `None` if the staff member is the CEO, as they have no supervisor.
    ///
    /// # Example
    /// ```rust
    /// let company = Company::new();
    /// let staff_id = "...";
    /// let supervisor_id = company.get_supervisor_id(staff_id).unwrap();
    /// println!("Supervisor ID: {}", supervisor_id);
    /// ```
    fn get_supervisor_id(&self, staff_id: Uuid) -> Option<Uuid>;

    /// Returns the [`std::collections::HashSet`] of subordinates's id for a given supervisor.
    ///
    /// Returns [`None`] if the supervisor has no subordinates.
    ///
    /// # Example
    /// ```rust
    /// let company = Company::new();
    /// let supervisor_id = "...";
    /// let subordinates = company.get_subordinates(supervisor_id).unwrap();
    /// for subordinate in subordinates {
    ///     println!("Subordinate ID: {}", subordinate);
    /// }
    /// ```
    fn get_subordinates(&self, supervisor_id: Uuid) -> Option<HashSet<Uuid>>;

    /// Retrieves the amount of resources for a specific staff member.
    ///
    /// Returns an error if the staff member is not found in the company.
    ///
    /// # Example
    /// ```rust
    /// let company = Company::new();
    /// let staff_id = "...";
    /// let resource_amount = company.get_resource_amount(staff_id).unwrap();
    /// println!("Resource amount: {}$", resource_amount);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] if staff entity is not found in company.
    fn get_resource_amount(&self, staff_id: Uuid) -> Result<Resource>;

    /// Spends a specific amount of resource from a staff member's balance.
    ///
    /// The operation will apply if the staff member is registered in the company and it have
    /// enougth resources in balance.
    ///
    /// # Example
    /// ```rust
    /// let mut company = Company::new();
    /// let staff_id = "...";
    /// let amount_to_spend = 50;
    /// let spent_amount = company.spend_resource(staff_id, amount_to_spend).unwrap();
    /// println!("Spent amount: {}$", spent_amount);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] if the staff member is not found
    /// or and error [`crate::errors::StaffError::InsufficientResourcesError`] if staff member's
    /// resources not enought.
    fn spend_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource>;

    /// Increase the balance of a specific staff member by amount.
    ///
    /// This method will be applied if the staff member is registered in the company.
    ///
    /// # Example
    /// ```rust
    /// let mut company = Company::new();
    /// let staff_id = "...";
    /// let amount_to_receive = 100;
    /// let new_balance = company.recieve_resource(staff_id, amount_to_receive).unwrap();
    /// println!("New balance: {}$", new_balance);
    /// ```
    ///
    /// # Errors
    /// Returns an error [`crate::errors::StaffError::StaffNotFound`] if staff entity is not found in company.
    fn recieve_resource(&mut self, staff_id: Uuid, amount: Resource) -> Result<Resource>;
}
