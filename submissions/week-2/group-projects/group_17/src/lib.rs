use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ResourceAllocator {
    pub id: u32,
    pub resource_name: String,
    pub project_name: String,
    pub quantity: u32,
}

// pub struct Resources {
//     pub resource: Vec<ResourceAllocator>,
//     next_id: u32
// }

pub struct Resources {
    pub resource: HashMap<u32, ResourceAllocator>,
    next_id: u32
}

impl Resources {
    pub fn init() -> Self {
        Self {
            resource: HashMap::new(),
            // resource: Vec::new(),
            next_id: 1
        }
    }

    // pub fn add_resource(&mut self, resource_name: String, project_name: String, quantity: u32) -> u32 {
    //     let id_count = self.next_id;
    //     self.next_id += 1;
    //     let record = ResourceAllocator {
    //         id: id_count,
    //         resource_name,
    //         project_name,
    //         quantity,
    //     };
    //     self.resource.push(record);
    //     id_count
    // }

    pub fn add_resource(&mut self, resource_name: String, project_name: String, quantity: u32) -> u32 {
        let id_count = self.next_id;
        self.next_id += 1;
        let record = ResourceAllocator {
            id: id_count,
            resource_name,
            project_name,
            quantity,
        };
        self.resource.insert(id_count, record);
        id_count
    }

    // pub fn view_allocations(&self) -> &Vec<ResourceAllocator> {
    //     &self.resource
    // }

    pub fn view_allocations(&self) -> Vec<&ResourceAllocator> {
        self.resource.values().collect()
    }

    // pub fn remove_allocations(&mut self, id: u32) {
    //     self.resource.retain(|index| index.id != id);
    // }

    pub fn remove_allocations(&mut self, id: u32) {
        self.resource.remove(&id);
    }

    // pub fn edit_allocation(&mut self, id: u32, new_resource_name: String, new_project_name: String, new_quantity: u32) {
    //     if let Some(rec) = self.resource.iter_mut().find(|index| index.id == id) {
    //         rec.resource_name = new_resource_name;
    //         rec.project_name = new_project_name;
    //         rec.quantity = new_quantity;
    //     }
    // }

    pub fn edit_allocation(&mut self, id: u32, new_resource_name: String, new_project_name: String, new_quantity: u32) {
        if let Some(rec) = self.resource.get_mut(&id) {
            rec.resource_name = new_resource_name;
            rec.project_name = new_project_name;
            rec.quantity = new_quantity;
        }
    }
}

pub use Resources as ResourceManager;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Resources {
        let mut resources = Resources::init();

        // Add some test data
        resources.add_resource(
            String::from("Cement"),
            String::from("House"),
            25
        );

        resources.add_resource(
            String::from("Steel"),
            String::from("Bridge"),
            50
        );

        resources
    }

    #[test]
    fn test_init_resources() {
        let resources = Resources::init();
        assert_eq!(resources.resource.len(), 0);
        assert_eq!(resources.next_id, 1);
    }

    #[test]
    fn test_add_resource() {
        let mut resources = Resources::init();
        
        let id = resources.add_resource(
            String::from("Wood"),
            String::from("Cabin"),
            100
        );
        
        assert_eq!(id, 1);
        assert_eq!(resources.resource.len(), 1);
        
        let resource = resources.resource.get(&1).unwrap();
        assert_eq!(resource.resource_name, "Wood");
        assert_eq!(resource.project_name, "Cabin");
        assert_eq!(resource.quantity, 100);
    }

    #[test]
    fn test_add_multiple_resources() {
        let record = setup();
        assert_eq!(record.resource.len(), 2);
        assert_eq!(record.next_id, 3);
        assert!(record.resource.contains_key(&1));
        assert!(record.resource.contains_key(&2));
    }

    #[test]
    fn test_view_allocations() {
        let record = setup();
        let allocations = record.view_allocations();
        
        assert_eq!(allocations.len(), 2);
        
        // Check that both resources exist (order doesn't matter with HashMap)
        let has_cement = allocations.iter().any(|r| r.resource_name == "Cement");
        let has_steel = allocations.iter().any(|r| r.resource_name == "Steel");
        assert!(has_cement);
        assert!(has_steel);
    }

    #[test]
    fn test_remove_allocation() {
        let mut record = setup();
        assert_eq!(record.resource.len(), 2);
        
        record.remove_allocations(1);
        assert_eq!(record.resource.len(), 1);
        assert!(!record.resource.contains_key(&1));
        assert!(record.resource.contains_key(&2));
        
        let remaining = record.resource.get(&2).unwrap();
        assert_eq!(remaining.resource_name, "Steel");
    }

    #[test]
    fn test_remove_nonexistent_allocation() {
        let mut record = setup();
        let original_len = record.resource.len();
        
        record.remove_allocations(999);
        assert_eq!(record.resource.len(), original_len);
    }

    #[test]
    fn test_edit_allocation() {
        let mut record = setup();
        
        record.edit_allocation(
            1,
            String::from("Concrete"),
            String::from("Skyscraper"),
            75
        );
        
        let updated = record.resource.get(&1).unwrap();
        assert_eq!(updated.resource_name, "Concrete");
        assert_eq!(updated.project_name, "Skyscraper");
        assert_eq!(updated.quantity, 75);
        assert_eq!(updated.id, 1);
    }

    #[test]
    fn test_edit_nonexistent_allocation() {
        let mut record = setup();
        let original_first = record.resource.get(&1).unwrap().clone();
        
        record.edit_allocation(
            999,
            String::from("Glass"),
            String::from("Nowhere"),
            10
        );
        
        let unchanged = record.resource.get(&1).unwrap();
        assert_eq!(unchanged.resource_name, original_first.resource_name);
        assert_eq!(unchanged.project_name, original_first.project_name);
        assert_eq!(unchanged.quantity, original_first.quantity);
    }

    #[test]
    fn test_empty_allocations() {
        let resources = Resources::init();
        let allocations = resources.view_allocations();
        assert!(allocations.is_empty());
    }
}
