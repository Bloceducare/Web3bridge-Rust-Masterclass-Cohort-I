use crate::event::models::models::{Attendee, Event, RsvpStatus};
impl RsvpStatus {
    pub fn check_variant(index: usize) -> Result<Self, String> {
        match index {
            1_usize => Ok(RsvpStatus::YES),
            2_usize => Ok(RsvpStatus::NO),
            0 => Ok(RsvpStatus::MAYBE),
            _ => Err("RsvpStatus Error::: Invalid index".to_string()),
        }
    }
}
impl Attendee {
    pub fn new(name: String, email: String, rsvp: usize) -> Result<Self, String> {
        if !email.contains("@") {
            return Err("Invalid email address".to_string());
        }

        let rsvp_status = RsvpStatus::check_variant(rsvp)?;

        Ok(Self {
            name,
            email,
            rsvp_status,
        })
    }

    pub fn change_details(&mut self, name: String, email: String, rsvp_status: RsvpStatus) -> bool {
        self.name = name;
        self.email = email;
        self.rsvp_status = rsvp_status;

        true
    }
}
impl Event {
    pub fn new() -> Self {
        Self {
            attendees: Vec::new(),
        }
    }
    pub fn add_attendee(
        &mut self,
        name: String,
        email: String,
        rsvp: usize,
    ) -> Result<bool, String> {
        self.attendees.push(Attendee::new(name, email, rsvp)?);
        Ok(true)
    }
    pub fn get_attendees(&self) -> Vec<&Attendee> {
        // self.attendees
        self.attendees.iter().collect()
    }
    pub fn remove_attendee(&mut self, index: usize) -> Result<bool, String> {
        if index >= self.attendees.len() {
            return Err("Invalid index".to_string());
        }
        self.attendees.remove(index);
        Ok(true)
    }
    pub fn update_attendee(
        &mut self,
        index: usize,
        name: String,
        email: String,
        rsvp_status: RsvpStatus,
    ) -> Result<bool, String> {
        if index >= self.attendees.len() {
            return Err("Invalid index".to_string());
        }
        let attendee = &mut self.attendees[index];
        attendee.change_details(name, email, rsvp_status);
        Ok(true)
    }
}
