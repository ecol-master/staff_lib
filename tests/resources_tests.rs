#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use staff_lib::companies::google::Google;
    use staff_lib::staff::{ceo::CEO, manager::Manager, worker::Worker};
    use staff_lib::traits::{CompanyBehaviour, StaffEntity, Supervisor};
    use staff_lib::types::{Company, Staff};
    use std::cell::RefCell;
    use std::rc::Rc;
    use uuid::Uuid;

    #[test]
    fn test_layoff_workers_balances() {
        let company = Rc::new(RefCell::new(Company::Google(Google::new())));

        let mut ceo = CEO::new(company.clone());
        company.borrow_mut().set_ceo(Staff::Ceo(ceo.clone()));

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

    #[test]
    fn test_resource_transferring() {
        let company = Rc::new(RefCell::new(Company::Google(Google::new())));

        let mut ceo = CEO::new(company.clone());
        company.borrow_mut().set_ceo(Staff::Ceo(ceo.clone()));
        company
            .borrow_mut()
            .recieve_resource(ceo.get_id(), 50000)
            .unwrap();

        let mut staff_id: Vec<Uuid> = vec![];

        for _ in 0..10 {
            let mut manager = Manager::new(company.clone());
            let manager_id = manager.get_id();
            staff_id.push(manager_id);

            ceo.hire(Staff::Manager(manager.clone())).unwrap();
            ceo.send_resource(manager_id, 3000).unwrap();

            for _ in 0..5 {
                let worker = Staff::Employee(Worker::new(company.clone()));
                let worker_id = worker.get_id();
                staff_id.push(worker_id);

                manager.hire(worker).unwrap();
                manager.send_resource(worker_id, 500).unwrap();
            }

            dbg!(manager.get_subordinates());
            assert_eq!(500, manager.get_resource_amount().unwrap());
        }

        assert_eq!(20000, ceo.get_resource_amount().unwrap());

        let mut rng = thread_rng();
        for _ in 0..200 {
            let pair: Vec<&Uuid> = staff_id.choose_multiple(&mut rng, 2).collect();
            company
                .borrow_mut()
                .transfer_resources(*pair[0], *pair[1], 1)
                .unwrap();
        }

        for id in staff_id {
            ceo.layoff(id).unwrap();
        }

        assert_eq!(50000, ceo.get_resource_amount().unwrap());
    }
}
