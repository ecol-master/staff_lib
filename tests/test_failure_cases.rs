#[cfg(test)]
mod tests {
    use staff_lib::{errors::Error, Company, StaffEntity};
    use uuid::Uuid;

    #[derive(Debug, Clone)]
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
    fn test_failure_hire() {
        let mut company: Company<Manager, u64> = Company::new(Manager::new());
        let ceo_id = company.ceo().unwrap().get_id();

        let manager = Manager::new();
        let cloned_manager = manager.clone();

        let manager_id = company.hire(manager, &ceo_id).unwrap();

        if let Err(Error::StaffAlreadyExists { staff_id }) = company.hire(cloned_manager, &ceo_id) {
            assert_eq!(staff_id, manager_id);
        } else {
            panic!("Expected HierarchyConflict error");
        }
    }

    #[test]
    fn test_failure_fire() {
        let mut company: Company<Manager, u64> = Company::new(Manager::new());
        let ceo_id = company.ceo().unwrap().get_id();

        assert!(matches!(company.fire(&ceo_id), Err(Error::CannotFireCeo)));

        let manager_id = company.hire(Manager::new(), &ceo_id).unwrap();
        company.fire(&manager_id).unwrap();

        if let Err(Error::StaffNotFound { staff_id }) = company.fire(&manager_id) {
            assert_eq!(staff_id, manager_id);
        } else {
            panic!("Expected HierarchyConflict error");
        }
    }
}
