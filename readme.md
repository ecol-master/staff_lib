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

**Structs**:
- [Company](./src/company.rs)

## Code examples

In `examples` folder you can find write code examples using this library.

Run concrete example file:
```
cargo run --example google
```


## Create project documentation:
You can see detailed documentation to this project using command:
```bash
cargo doc --open
```
