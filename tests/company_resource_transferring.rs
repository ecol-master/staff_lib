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
    fn test_resource_transferring() {
        let ceo = Manager::new();
        let ceo_id = ceo.get_id();

        let mut company: Company<Manager, u64> = Company::new(ceo);
        let mint_amount: u64 = 1000;
        company.mint(mint_amount);

        let target_manager = Manager::new();
        let target_id = company.hire(target_manager, &ceo_id).unwrap();

        let mut ceo_resource = (mint_amount / 10) * 9;
        for _ in 0..10 {
            let id = company.hire(Manager::new(), &ceo_id).unwrap();

            let manager_resource = ceo_resource / 10;
            ceo_resource = ceo_resource - manager_resource;

            assert_eq!(*company.get_resource(&id).unwrap(), manager_resource);
            assert_eq!(*company.get_resource(&ceo_id).unwrap(), ceo_resource);
        }
        dbg!(ceo_resource);

        assert_eq!(company.get_all_staff().len(), 12); // 11 managers and ceo
        assert_eq!(company.get_subordinates(&ceo_id).unwrap().len(), 11);

        let ceo_subordinates = company.get_subordinates(&ceo_id).unwrap().clone();
        for id in ceo_subordinates.clone() {
            let amount = *company.get_resource(&id).unwrap();
            dbg!(amount);
            company.transfer(&id, &target_id, amount).unwrap();
        }

        for id in ceo_subordinates.clone() {
            if id != target_id {
                assert_eq!(*company.get_resource(&id).unwrap(), 0);
                company.fire(&id).unwrap();
            }
        }

        dbg!(ceo_resource);
        assert_eq!(
            *company.get_resource(&target_id).unwrap(),
            mint_amount - ceo_resource
        );

        company.fire(&target_id).unwrap();
        assert_eq!(*company.get_resource(&ceo_id).unwrap(), mint_amount);
    }
}
