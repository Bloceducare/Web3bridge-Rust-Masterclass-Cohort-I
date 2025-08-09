#[derive(Debug)]
pub enum GarageError {
    StaffNotFound,
    StaffCanNotAccessGarage,
    StaffNotActive,
    StaffAlreadyExists,
    StaffTerminated
}

#[derive(Clone, Debug, PartialEq)]
pub enum StaffType {
    MediaTeam,
    SocialMediaTeam,
    Manager,
    ITTeam,
    MarketingTeam,
    SalesTeam,
    CustomerServiceTeam,
    TechinalSupervisor,
    KitchenStaff,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StaffStatus {
    Active,
    Inactive,
    Suspended,
    OnLeave,
    Terminated,
}

#[derive(Debug)]
pub struct StaffMember {
    pub id: u32,
    pub name: String,
    pub staff_type: StaffType,
    pub status: StaffStatus,
    pub still_employed: bool,
}

pub struct Garage {
    pub staff: Vec<StaffMember>,
    pub next_id: u32,
}

impl Garage {
    pub fn new() -> Self {
        Self {
            staff: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_staff(&mut self, name: String, staff_type: StaffType) -> bool {
        let staff_member = StaffMember {
            id: self.next_id,
            name,
            staff_type,
            status: StaffStatus::Active,
            still_employed: true,
        };
        self.staff.push(staff_member);
        self.next_id += 1;
        true
    }

    pub fn get_staff(&self, id: u32) -> Option<&StaffMember> {
        self.staff.iter().find(|s| s.id == id)
    }

    pub fn get_all_staff(&self) -> &Vec<StaffMember> {
        &self.staff
    }

    pub fn update_staff(&mut self, id: u32, name: String, staff_type: StaffType) -> bool {
        if let Some(staff_member) = self.staff.iter_mut().find(|s| s.id == id) {
            staff_member.name = name;
            staff_member.staff_type = staff_type;
            true
        } else {
            false
        }
    }

    pub fn delete_staff(&mut self, id: u32) -> bool {
        if let Some(staff_member) = self.staff.iter_mut().find(|s| s.id == id) {
            staff_member.still_employed = false;
            staff_member.status = StaffStatus::Inactive;
            true
        } else {
            false
        }
    }

    pub fn set_staff_status(&mut self, id: u32, status: StaffStatus) -> bool {
        if let Some(staff_member) = self.staff.iter_mut().find(|s| s.id == id) {
            staff_member.status = status;
            true
        } else {
            false
        }
    }

    //terminate staff
    pub fn terminate_staff(&mut self, id: u32) -> bool {
        if let Some(staff_member) = self.staff.iter_mut().find(|s| s.id == id) {
            staff_member.still_employed = false;
            staff_member.status = StaffStatus::Terminated;
            true
        } else {
            false
        }
    }

    pub fn can_staff_access_garage(&self, id: u32) -> Result<bool, GarageError> {
        if let Some(staff_member) = self.staff.iter().find(|s| s.id == id) {
            if staff_member.status == StaffStatus::Terminated {
                return Err(GarageError::StaffTerminated);
            } else if staff_member.status == StaffStatus::Inactive {
                return Err(GarageError::StaffNotActive);
            }
            match staff_member.staff_type {
                StaffType::Manager => Ok(true),
                StaffType::ITTeam => Ok(true),
                StaffType::MediaTeam => Ok(true),
                _ => Err(GarageError::StaffCanNotAccessGarage),
            }
        } else {
            Err(GarageError::StaffNotFound)
        }
    }
}
