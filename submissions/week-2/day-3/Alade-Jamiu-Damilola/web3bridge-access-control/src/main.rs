use web3bridge_garage::{Employee, EmployeeType, print_access};

fn main() {
    let alice = Employee::new(EmployeeType::IT, true);
    let bob   = Employee::new(EmployeeType::SocialMediaTeam, true);
    let charlie = Employee::new(EmployeeType::Manager, false);

    for (name, emp) in [("Alice", &alice), ("Bob", &bob), ("Charlie", &charlie)] {
        match print_access(emp) {
            Ok(_)  => println!("{} entered the garage.", name),
            Err(e) => println!("{} was denied: {:?}", name, e),
        }
    }
}