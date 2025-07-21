use group_17::*;
use std::io;

fn main() {
    let mut resources = Resources::init();

    println!("Resource Allocation\nAdd a record of the resources you need for your project");
    loop {
        println!("Enter your resource name");
        let mut resource_name = String::new();
        io::stdin().read_line(&mut resource_name).expect("Something is wrong with resource name");
        let resource_name: String = match resource_name.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your project name");
        let mut project_name = String::new();
        io::stdin().read_line(&mut project_name).expect("Something is wrong with resource name");
        let project_name: String = match project_name.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your quantity");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Something is wrong with resource name");
        let quantity: u32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        resources.add_resource(resource_name, project_name, quantity);

        let out = resources.view_allocations();
        println!("Out first add: {:#?}\n", out);
        
        println!("Enter your resource name");
        let mut resource_name2 = String::new();
        io::stdin().read_line(&mut resource_name2).expect("Something is wrong with resource name");
        let resource_name2: String = match resource_name2.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your project name");
        let mut project_name2 = String::new();
        io::stdin().read_line(&mut project_name2).expect("Something is wrong with resource name");
        let project_name2: String = match project_name2.trim().parse() {
            Ok(name) => name,
            Err(_) => continue,
        };
        println!("Enter your quantity");
        let mut quantity2 = String::new();
        io::stdin().read_line(&mut quantity2).expect("Something is wrong with resource name");
        let quantity2: u32 = match quantity2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        resources.add_resource(resource_name2, project_name2, quantity2);

        let out = resources.view_allocations();
        println!("Out Second add: {:#?}", out);

        resources.remove_allocations(1);

        let out = resources.view_allocations();
        println!("Out after remove 1: {:#?}", out);

        resources.edit_allocation(2, String::from("Wood"), String::from("Nails"), 16);

        let out = resources.view_allocations();
        println!("Out after edit: {:#?}", out);

        break;
    }
}
