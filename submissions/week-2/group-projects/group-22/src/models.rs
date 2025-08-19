use std::collections::HashMap;

pub struct Space {
    pub name: String,
    pub occupant: String,
    pub purpose: String,
}

pub struct SpaceManager {
    // spaces: Vec<Space>,
    pub spaces: HashMap<String, Space>,
}

impl SpaceManager {
    pub fn new() -> Self {
        SpaceManager {
            spaces: HashMap::new(),
        }
    }
}
