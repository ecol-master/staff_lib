#[cfg(test)]
mod tests {
    use staff_lib::{errors::Error, Company, StaffEntity};
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

    #[test]
    fn test_change_supervisor() {
        let ceo = Manager::new();
        let ceo_id = ceo.get_id();

        let mut company: Company<Manager, u64> = Company::new(ceo);

        let first_manager_id = company.hire(Manager::new(), &ceo_id).unwrap();
        let second_manager_id = company.hire(Manager::new(), &first_manager_id).unwrap();

        if let Err(Error::HierarchyConflict {
            staff_id,
            supervisor_id,
        }) = company.change_supervisor(&first_manager_id, &second_manager_id)
        {
            assert_eq!(staff_id, first_manager_id);
            assert_eq!(supervisor_id, second_manager_id);
        } else {
            panic!("Expected HierarchyConflict error");
        }

        let third_manager_id = company.hire(Manager::new(), &second_manager_id).unwrap();

        if let Err(Error::HierarchyConflict {
            staff_id,
            supervisor_id,
        }) = company.change_supervisor(&first_manager_id, &third_manager_id)
        {
            assert_eq!(staff_id, first_manager_id);
            assert_eq!(supervisor_id, third_manager_id);
        } else {
            panic!("Expected HierarchyConflict error");
        }

        // Успешное изменение супервизора
        assert!(company
            .change_supervisor(&third_manager_id, &ceo_id)
            .is_ok());

        // Проверка общего количества сотрудников
        assert_eq!(company.get_all_staff().len(), 4); // CEO + 3 managers
    }
}
