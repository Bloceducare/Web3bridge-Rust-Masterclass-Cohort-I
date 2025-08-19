#[derive(Debug)]
pub enum Department {
  MEDIA,
  IT,
  MANAGER,
  SOCIAL_MEDIA,
  TECHNICIAN_SUPERVISOR,
  KITCHEN_STAFF,
}

#[derive(Debug, PartialEq)]
pub enum Status {
  ACTIVE,
  TERMINATED,
}
