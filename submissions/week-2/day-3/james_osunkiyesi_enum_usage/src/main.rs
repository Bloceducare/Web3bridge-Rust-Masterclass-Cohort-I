mod web3bridge;

use web3bridge::{Web3Bridge, StaffDepartment};

fn main() {
    let mut web3b = Web3Bridge::init();

    web3b.employ(String::from("James"), StaffDepartment::IT);
    web3b.employ(String::from("Joe"), StaffDepartment::Kitchen);

    let staff_list = web3b.get_active_staffs();
}
