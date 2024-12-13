#[cfg(test)]
mod tests {
    use staff_lib::companies::google::Google;
    use staff_lib::errors::StaffError;
    use staff_lib::staff::{ceo::CEO, manager::Manager, worker::Worker};
    use staff_lib::traits::{CompanyBehaviour, StaffEntity, Supervisor};
    use staff_lib::types::{Company, Resource, Staff};
    use std::cell::RefCell;
    use std::rc::Rc;
    use uuid::Uuid;

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

    #[test]
    fn test_resource_transferring_failures() {
        let company = Rc::new(RefCell::new(Company::Google(Google::new())));

        let mut ceo = CEO::new(company.clone());
        company.borrow_mut().set_ceo(Staff::Ceo(ceo.clone()));

        company
            .borrow_mut()
            .recieve_resource(ceo.get_id(), 100000)
            .unwrap();

        let mut staff_id: Vec<Uuid> = vec![];
        for _ in 0..500 {
            let worker = Worker::new(company.clone());
            staff_id.push(worker.get_id());

            ceo.hire(Staff::Employee(worker)).unwrap();
        }

        assert_eq!(100000, ceo.get_resource_amount().unwrap());

        for id in staff_id.clone() {
            ceo.send_resource(id, 200).unwrap();
        }

        assert_eq!(0, ceo.get_resource_amount().unwrap());

        for id in staff_id.clone() {
            assert_eq!(200, company.borrow().get_resource_amount(id).unwrap());
        }

        let mut ceo_balance: Resource = 0;
        for id in staff_id.clone() {
            ceo_balance += 100;
            assert_eq!(
                ceo_balance,
                company
                    .borrow_mut()
                    .transfer_resources(id, ceo.get_id(), 100)
                    .unwrap()
            );

            assert_eq!(100, company.borrow().get_resource_amount(id).unwrap());

            ceo_balance += 100;
            assert_eq!(
                ceo_balance,
                company
                    .borrow_mut()
                    .transfer_resources(id, ceo.get_id(), 100)
                    .unwrap()
            );

            assert_eq!(0, company.borrow().get_resource_amount(id).unwrap());

            assert!(matches!(
                company
                    .borrow_mut()
                    .transfer_resources(id, ceo.get_id(), 100),
                Err(StaffError::InsufficientResourcesError(_))
            ));
        }

        assert_eq!(100000, ceo.get_resource_amount().unwrap());
        assert_eq!(100000, ceo_balance);
    }
}
