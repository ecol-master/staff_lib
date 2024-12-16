use staff_lib::traits::StaffEntity;
use staff_lib::Company;
use uuid::Uuid;

#[derive(Debug)]
pub struct Worker {
    id: Uuid,
    grade: u8,
}

impl Worker {
    fn new(grade: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            grade,
        }
    }

    fn grade(&self) -> u8 {
        self.grade
    }

    fn upgrade(&mut self, new_grade: u8) {
        self.grade = new_grade;
    }
}

impl StaffEntity for Worker {
    type ID = Uuid;

    fn get_id(&self) -> Uuid {
        self.id
    }
}

fn main() {
    let ceo = Worker::new(15);
    let ceo_id = ceo.get_id();
    let mut company = Company::<Worker, u64>::new(ceo);

    let worker = Worker::new(10);
    let worker_id = company.hire(worker, &ceo_id).unwrap();

    println!(
        "Worker's grade: {}",
        company.get(&worker_id).unwrap().grade()
    );

    company.get_mut(&worker_id).unwrap().upgrade(20);

    println!(
        "After updgrade: {}",
        company.get(&worker_id).unwrap().grade()
    );
}
