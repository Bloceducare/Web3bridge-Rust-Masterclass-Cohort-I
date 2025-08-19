pub mod lib;

use crate::lib::{Profiles, Profile, Status};


fn main() {
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

        println!("profiles: {:?}", all_profiles);

}
