use std::collections::HashMap;
use uuid::Uuid;
use crate::web3bridge::StaffDepartment;

pub struct Staff {
    name: String,
    id: Uuid,
    dept: StaffDepartment,
    pub is_employed: bool,
}

impl Staff {
    pub(crate) fn new(name: String, dept: StaffDepartment) -> (Uuid, Self) {
        let id = Uuid::new_v4();
        (
            id,
            Self {
                name,
                id,
                dept,
                is_employed: true,
            },
        )
    }

    fn terminate_employment(&mut self) {
        self.is_employed = false
    }

    fn is_employed(&self) -> bool {
        self.is_employed
    }

    pub(crate) fn check_building_access(&self) -> Result<bool, &str> {
        if self.is_employed {
            match self.dept {
                StaffDepartment::MediaTeam => Ok(true),
                StaffDepartment::IT => Ok(true),
                StaffDepartment::Manager => Ok(true),
                _ => Err("No access to the building"),
            }
        } else {
            panic!("Staff has already been removed")
        }
    }
}


pub struct Web3Bridge {
    pub staffs: HashMap<Uuid, Staff>,
}
