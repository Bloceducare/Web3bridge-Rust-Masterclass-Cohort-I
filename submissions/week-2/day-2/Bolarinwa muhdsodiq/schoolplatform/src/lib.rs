#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub grade: i32,
    pub status: Status,
}

pub struct Profiles {
    pub profiles: Vec<Profile>,
}

impl Profiles {
    pub fn initialize() -> Profiles {
        Profiles {
            profiles: Vec::new(),
        }
    }

    pub fn create_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    pub fn update_profile(&mut self, index: usize) {
        if index < self.profiles.len() {
            self.profiles[index].status = match self.profiles[index].status {
                Status::ACTIVE => Status::INACTIVE,
                Status::INACTIVE => Status::ACTIVE,
            }
        } else {
            panic!("Out of index")
        }
    }

    pub fn get_profile(&self, index: usize) -> &Profile {
        self.profiles.get(index).unwrap()
    }

    pub fn get_all_profiles(&self) -> Vec<Profile> {
        self.profiles.to_vec()
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.profiles.len() {
            self.profiles.remove(index);
        } else {
            panic!("Out of index")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_profile() {
        let mut profiles = Profiles::initialize();

        assert!(profiles.profiles.len() == 0);

        let profile = Profile {
            name: String::from("John Doe"),
            grade: 10,
            status: Status::ACTIVE,
        };

        profiles.create_profile(profile);

        assert!(profiles.profiles.len() == 1);
        assert_eq!(profiles.get_profile(0).name, "John Doe");
    }

    #[test]
    fn test_update_profile() {
        let mut profiles = Profiles::initialize();

        let profile = Profile {
            name: String::from("Jane Doe"),
            grade: 12,
            status: Status::ACTIVE,
        };
        profiles.create_profile(profile);

        profiles.update_profile(0);

        assert_eq!(profiles.get_profile(0).status, Status::INACTIVE);
    }

    #[test]
    fn test_get_profile() {
        let mut profiles = Profiles::initialize();

        let profile = Profile {
            name: String::from("Alice Smith"),
            grade: 11,
            status: Status::ACTIVE,
        };
        profiles.create_profile(profile);

        let retrieved_profile = profiles.get_profile(0);
        assert_eq!(retrieved_profile.name, "Alice Smith");
        assert_eq!(retrieved_profile.grade, 11);
        assert_eq!(retrieved_profile.status, Status::ACTIVE);
    }

    #[test]
    fn test_get_all_profiles() {
        let mut profiles = Profiles::initialize();

        let profile1 = Profile {
            name: String::from("Bob Brown"),
            grade: 9,
            status: Status::ACTIVE,
        };
        let profile2 = Profile {
            name: String::from("Charlie Green"),
            grade: 10,
            status: Status::INACTIVE,
        };
        profiles.create_profile(profile1);
        profiles.create_profile(profile2);

        let all_profiles = profiles.get_all_profiles();
        assert_eq!(all_profiles.len(), 2);
        assert_eq!(all_profiles[0].name, "Bob Brown");
        assert_eq!(all_profiles[1].name, "Charlie Green");
    }
}
