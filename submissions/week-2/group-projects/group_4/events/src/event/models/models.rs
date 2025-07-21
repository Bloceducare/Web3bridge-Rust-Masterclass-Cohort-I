#[derive(Debug)]
pub struct Attendee {
    pub name: String,
    pub email: String,
    pub rsvp_status: RsvpStatus,
}

#[derive(Debug)]
pub enum RsvpStatus {
    YES,
    NO,
    MAYBE,
}

#[derive(Debug)]
pub struct Event {
    pub attendees: Vec<Attendee>,
}
