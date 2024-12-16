# staff lib
## Project overview
`staff_lib` is a library which is created for making the simple hierarchy system for managing company's staff and resources.

## Library design
The design of this library follows a few key principles:
- Only `Company` owns all the data stored in it
- All staff entites act as view objects used for interaction with the company object, but do not hold data themselves.

## Library defined entities
**Traits**:
- [StaffEntity](./src/traits.rs), 

**Structs**:
- [Company](./src/company.rs)


## Code examples

In `examples` folder you can find writen code examples how using this library.

Run concrete example file:

```
cargo run --example google
```

The simpliest way of using stored in file [simple.rs](./examples/simple.rs):
```rust
use staff_lib::traits::StaffEntity;
use staff_lib::Company;
use uuid::Uuid;

/// Create our custom `Worker` type which implement `Staff Entity`.
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
```

However, you can add custom methods for your `Worker` struct and use them. File [mutable_methods.rs](./examples/mutable_methods.rs):
```rust
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
```

If you need to implement custom behaviour for your company you can find an example of embedding default library's struct `Company` into yours. See [google.rs](./examples/google.rs) 


## Create project documentation:
You can see detailed documentation to this project using command:
```bash
cargo doc --open
```


