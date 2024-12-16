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
    let company = Company::<Worker, u64>::new(ceo);
    dbg!(company.ceo());
}
