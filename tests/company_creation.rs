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
    fn test_create_company() {
        let manager = Manager::new();
        let manager_id = manager.get_id();

        let mut company: Company<Manager, u64> = Company::new(manager);

        assert_eq!(company.get_ceo().unwrap().get_id(), manager_id);
        assert_eq!(*company.get_resource(&manager_id).unwrap(), 0);
        assert_eq!(company.get_supervisor(&manager_id), None);
        assert_eq!(company.get_subordinates(&manager_id), None);
        assert_eq!(company.get_all_staff().len(), 1);

        let mint_amount: u64 = 1000;
        company.mint(mint_amount);

        assert_eq!(*company.get_resource(&manager_id).unwrap(), mint_amount);
    }
}
