# staff lib
## Project overview
`staff_lib` is a library which defines the traits and its implementations for making the simple hierarchy system for managing company's staff and resources:
- `Employee`: a company entity with basic functionality.
- `Supervisor`: a special company role that can manage subordinates.
- `CEO`: the company's owner, responsible for top-level management.
- `Company`: a concrete company implementation.

## Library design
The design of this library follows a few key principles:
- Only `Company` owns all the data stored in it
- All staff entites act as view objects used for interaction with the company object, but do not hold data themselves.

## Library defined entities
**Traits**:
- [StaffEntity](./src/traits.rs), 
- [Employee](./src/traits.rs), 
- [Supervisor](./src/traits.rs), 
- [CompanyBehaviour](./src/traits.rs), 

**Staff** implementations:
- [CEO](./src/staff/ceo.rs)  
- [Manager](./src/staff/manager.rs)
- [Worker](./src/staff/worker.rs)

**Companies** implementations:
- [Google](./src/companies/google.rs) 

**Enums**:
- [Staff](./src/types.rs)
- [Company](./src/types.rs)

## Create project documentation:
You can see detailed documentation to this project using command:
```bash
cargo doc --open
```