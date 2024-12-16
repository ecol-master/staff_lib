use staff_lib::traits::StaffEntity;
use staff_lib::Company;
use uuid::Uuid;

#[derive(Debug)]
pub struct Worker {
    id: Uuid,
}

impl Worker {
    fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

impl StaffEntity for Worker {
    type ID = Uuid;

    fn get_id(&self) -> Uuid {
        self.id
    }
}

fn main() {
    let ceo = Worker::new();
    let mut company = Company::<Worker, u64>::new(ceo);
    let start_resource = 100000;
    company.mint(start_resource);

    let ceo_id = company.ceo().unwrap().get_id();
    for _ in 0..10 {
        let manager = Worker::new();

        let id = company.hire(manager, &ceo_id).unwrap();
        for _ in 0..50 {
            company.hire(Worker::new(), &id).unwrap();
        }
    }

    for id in company.get_all_staff() {
        if id == ceo_id {
            continue;
        }

        company.fire(&id).unwrap();
    }

    assert_eq!(*company.resource(&ceo_id).unwrap(), start_resource);
}
