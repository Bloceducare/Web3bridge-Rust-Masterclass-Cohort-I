pub mod states;

use std::collections::HashMap;
use uuid::Uuid;
pub(crate) use states::{Web3Bridge};
use crate::web3bridge::states::Staff;

pub enum StaffDepartment {
    MediaTeam,
    IT,
    Manager,
    SocialTeam,
    Kitchen,
    TechnicianSuperVisor,
}

impl Web3Bridge {
    pub fn init() -> Self {
        Self {
            staffs: HashMap::new(),
        }
    }
    pub fn employ(&mut self, name: String, staff_department: StaffDepartment) {
        let (id, new_staff) = Staff::new(name, staff_department);
        self.staffs.insert(id, new_staff);
    }

    pub fn terminate_employment(&mut self, id: Uuid) {
        let _staff = self.staffs.get(&id);
        match _staff {
            Some(staff) => {
                if staff.is_employed {
                    self.staffs.entry(id).and_modify(|e| e.is_employed = false);
                } else {
                    panic!("Staff has already been removed")
                }
            }
            None => panic!("Staff does not exist"),
        }
    }

    pub fn enter_building(&self, id: Uuid) {
        let staff = self.staffs.get(&id).unwrap();

        let has_access = staff.check_building_access().unwrap();
        if has_access {
            println!("You shall Enter")
        }
    }

    pub fn get_active_staffs(&self) -> Vec<Uuid> {
        self.staffs
            .iter()
            .filter_map(
                |(id, staff)| {
                    if staff.is_employed { Some(*id) } else { None }
                },
            )
            .collect()
    }
}
