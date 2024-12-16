use staff_lib::errors::Error;
use staff_lib::traits::StaffEntity;
use staff_lib::Company;
use std::collections::HashMap;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Debug)]
pub struct Manager {
    id: Uuid,
    _name: String,
}

impl Manager {
    fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            _name: name,
        }
    }
}

impl StaffEntity for Manager {
    type ID = Uuid;

    fn get_id(&self) -> Uuid {
        self.id
    }
}

///////////////////////////////////
struct GoogleService {
    id: Uuid,
    name: String,
    manager_id: Uuid,
    budget: u64,
}

impl Display for GoogleService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Service: \"{}\"", self.name)?;
        writeln!(f, "\tID: {}", self.id)?;
        writeln!(f, "\tManager ID: {}", self.manager_id)?;
        writeln!(f, "\tBudget: {}", self.budget)?;
        writeln!(f, "--------------------------")
    }
}

struct Google {
    company: Company<Manager, u64>,
    services: HashMap<Uuid, GoogleService>,
}

impl Google {
    fn new(ceo: Manager) -> Self {
        Self {
            company: Company::new(ceo),
            services: HashMap::new(),
        }
    }

    fn get_ceo(&self) -> &Manager {
        self.company.ceo().unwrap()
    }

    fn mint(&mut self, amount: u64) {
        self.company.mint(amount)
    }

    fn hire(&mut self, manager: Manager, supervisor_id: &Uuid) -> Result<Uuid, Error<Uuid, u64>> {
        self.company.hire(manager, supervisor_id)
    }

    fn add_service(&mut self, name: String, manager_id: &Uuid) -> Uuid {
        let budget = self.company.resource(manager_id).unwrap() / 2;
        self.company.withdraw(manager_id, budget).unwrap();

        let service_id = Uuid::new_v4();

        self.services.insert(
            service_id,
            GoogleService {
                id: service_id,
                name,
                manager_id: manager_id.clone(),
                budget,
            },
        );
        service_id
    }

    fn get_services(&self) -> Vec<&GoogleService> {
        self.services.iter().map(|(_, s)| s).collect()
    }
}

fn main() {
    let ceo = Manager::new(String::from("Sundar Pichai"));
    let mut google_company = Google::new(ceo);
    google_company.mint(100);
    let ceo_id = &google_company.get_ceo().get_id();

    let mut services: Vec<String> = vec![
        String::from("Google Drive"),
        String::from("Pixel"),
        String::from("Gmail"),
    ];

    let manager_names: Vec<String> = vec![String::from(""), String::from(""), String::from("")];

    for name in manager_names {
        let manager = Manager::new(name);
        let id = google_company.hire(manager, ceo_id).unwrap();
        google_company.add_service(services.pop().unwrap(), &id);
    }

    for service in google_company.get_services() {
        println!("{}", service);
    }
}
