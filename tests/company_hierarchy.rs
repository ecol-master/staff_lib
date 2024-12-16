#[cfg(test)]
mod tests {
    use staff_lib::{Company, StaffEntity};
    use uuid::Uuid;

    #[derive(Debug)]
    struct Manager {
        id: Uuid,
    }

    impl Manager {
        fn new() -> Self {
            Self { id: Uuid::new_v4() }
        }
    }

    impl StaffEntity for Manager {
        type ID = Uuid;

        fn get_id(&self) -> Self::ID {
            self.id
        }
    }

    #[test]
    fn test_company_hierarchy() {
        let ceo = Manager::new();
        let ceo_id = ceo.get_id();

        let mut company: Company<Manager, u64> = Company::new(ceo);
        let mint_amount: u64 = 1000;
        company.mint(mint_amount);

        let mut ceo_resource = mint_amount;
        for _ in 0..10 {
            let id = company.hire(Manager::new(), &ceo_id).unwrap();

            let manager_resource = ceo_resource / 10;
            ceo_resource = ceo_resource - manager_resource;

            assert_eq!(*company.resource(&id).unwrap(), manager_resource);
            assert_eq!(*company.resource(&ceo_id).unwrap(), ceo_resource);
        }

        assert_eq!(company.get_all_staff().len(), 11); // 10 managers and ceo
        assert_eq!(company.subordinates(&ceo_id).unwrap().len(), 10);

        for id in company.subordinates(&ceo_id).unwrap() {
            assert_eq!(company.supervisor(id).unwrap(), &ceo_id);
        }

        let ceo_subordinates = company.subordinates(&ceo_id).unwrap();
        for id in ceo_subordinates.clone() {
            company.fire(&id).unwrap();
        }

        assert_eq!(*company.resource(&ceo_id).unwrap(), mint_amount);
    }
}
