#[cfg(test)]
mod tests {
    use staff_lib::company::Company;
    use staff_lib::staff::{ceo::CEO, manager::Manager, worker::Worker};
    use staff_lib::traits::{StaffEntity, Supervisor};
    use staff_lib::types::{Result, Staff};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn simple() {
        let company = Rc::new(RefCell::new(Company::new()));

        let mut ceo = CEO::new(company.clone());
        company.as_ref().borrow_mut().set_ceo(ceo.clone()).unwrap();

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
}
