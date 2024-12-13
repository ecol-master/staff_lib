#[cfg(test)]
mod tests {
    use staff_lib::companies::google::Google;
    use staff_lib::staff::{ceo::CEO, manager::Manager, worker::Worker};
    use staff_lib::traits::{CompanyBehaviour, StaffEntity, Supervisor};
    use staff_lib::types::{Company, Staff};
    use std::cell::RefCell;
    use std::rc::Rc;
    use uuid::Uuid;

    #[test]
    fn test_simple() {
        let company = Rc::new(RefCell::new(Company::Google(Google::new())));

        let mut ceo = CEO::new(company.clone());
        company
            .as_ref()
            .borrow_mut()
            .set_ceo(Staff::Ceo(ceo.clone()))
            .unwrap();

        let director = Manager::new(company.clone());
        ceo.hire(Staff::Manager(director.clone())).unwrap();

        assert_eq!(
            company
                .borrow_mut()
                .recieve_resource(ceo.get_id(), 10000)
                .unwrap(),
            10000
        );

        ceo.send_resource(director.get_id(), 100).unwrap();
        assert_eq!(100, director.get_resource_amount().unwrap());
    }

    #[test]
    fn test_layoff_workers_balances() {
        let company = Rc::new(RefCell::new(Company::Google(Google::new())));

        let mut ceo = CEO::new(company.clone());
        company
            .as_ref()
            .borrow_mut()
            .set_ceo(Staff::Ceo(ceo.clone()))
            .unwrap();
        company
            .borrow_mut()
            .recieve_resource(ceo.get_id(), 10000)
            .unwrap();

        let mut workers: Vec<Uuid> = vec![];
        for _ in 0..5 {
            let worker = Worker::new(company.clone());
            let worker_id = worker.get_id();
            workers.push(worker.get_id());

            ceo.hire(Staff::Employee(worker)).unwrap();
            ceo.send_resource(worker_id, 100).unwrap();
        }

        assert_eq!(9500, ceo.get_resource_amount().unwrap());

        for id in workers.clone() {
            assert_eq!(100, company.borrow().get_resource_amount(id).unwrap());
        }

        for id in workers {
            ceo.layoff(id).unwrap();
        }

        assert_eq!(10000, ceo.get_resource_amount().unwrap());
    }
}
