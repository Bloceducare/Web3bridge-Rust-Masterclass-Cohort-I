#[derive(Debug, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: u32,
    pub customer_name: String,
    pub issue_description: String,
    pub priority: Priority,
    pub status: String,
}

pub fn string_to_priority(s: &str) -> Option<Priority> {
    match s.to_lowercase().as_str() {
        "low" => Some(Priority::Low),
        "medium" => Some(Priority::Medium),
        "high" => Some(Priority::High),
        "critical" => Some(Priority::Critical),
        _ => None,
    }
}

pub fn priority_to_string(priority: &Priority) -> &str {
    match priority {
        Priority::Low => "Low",
        Priority::Medium => "Medium",
        Priority::High => "High",
        Priority::Critical => "Critical",
    }
}

pub fn create_ticket(
    id: u32,
    customer_name: String,
    issue_description: String,
    priority: Priority,
) -> Ticket {
    Ticket {
        id,
        customer_name,
        issue_description,
        priority,
        status: "Open".to_string(),
    }
}
