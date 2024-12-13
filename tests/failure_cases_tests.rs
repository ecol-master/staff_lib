#[cfg(test)]
mod tests {
    use staff_lib::companies::google::Google;
    use staff_lib::errors::StaffError;
    use staff_lib::staff::{ceo::CEO, manager::Manager, worker::Worker};
    use staff_lib::traits::{CompanyBehaviour, StaffEntity, Supervisor};
    use staff_lib::types::{Company, Staff};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_hire_layoff_failure() {
        let company = Rc::new(RefCell::new(Company::Google(Google::new())));

        let mut ceo = CEO::new(company.clone());
        company.borrow_mut().set_ceo(Staff::Ceo(ceo.clone()));

        let mut manager = Manager::new(company.clone());
        ceo.hire(Staff::Manager(manager.clone())).unwrap();

        let mut staff_id: Vec<Staff> = vec![];

        for i in 0..100 {
            if i % 2 == 0 {
                let worker = Staff::Employee(Worker::new(company.clone()));
                staff_id.push(worker.clone());
                manager.hire(worker).unwrap();
            } else {
                let manager = Staff::Manager(Manager::new(company.clone()));
                staff_id.push(manager.clone());
                ceo.hire(manager).unwrap();
            }
        }

        for i in 0..100 {
            assert!(matches!(
                ceo.hire(staff_id[i].clone()),
                Err(StaffError::StaffAlreadyExists(_))
            ));
        }

        for i in 0..100 {
            assert_eq!(
                ceo.layoff(staff_id[i].get_id().clone()).unwrap().get_id(),
                staff_id[i].get_id()
            );
        }

        for i in 0..100 {
            assert!(matches!(
                ceo.layoff(staff_id[i].get_id()),
                Err(StaffError::StaffNotFound(_))
            ));
        }
    }
}
