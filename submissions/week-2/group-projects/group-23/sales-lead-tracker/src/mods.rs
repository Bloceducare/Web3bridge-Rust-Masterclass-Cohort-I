#[derive(Debug, Clone)]
pub struct Leads {
    pub id: u32,
    pub name: String,
    pub contact: String,
    pub value: f64,
    pub status: LeadStatus,
}

#[derive(Debug, Clone)]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Lost,
    Converted,
}