// Description: Manage vendor contracts.
// Stage 1: Add contracts (vendor name, contract value, end date). View all contracts.
// Stage 2: Remove expired or canceled contracts.
// Stage 3: Edit contract details.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ContractStatus {
    Active,
    Expired,
    Canceled,
}

#[derive(Debug, Clone)]
pub struct Contract {
    vendor_name: String,
    contract_value: f64,
    end_date: String,
    status: ContractStatus,
    id: u32,
}

impl Contract {
    pub fn new(vendor_name: String, contract_value: f64, end_date: String, id: u32) -> Self {
        Contract {
            vendor_name,
            contract_value,
            end_date,
            status: ContractStatus::Active,
            id,
        }
    }
}

// ONE implementation that switches from Vec to HashMap
pub struct VendorManagement {
    contracts_vec: Option<Vec<Contract>>,      // Stage 1: Use Vec
    contracts_hashmap: Option<HashMap<u32, Contract>>,  // Stage 2+: Use HashMap 
    next_id: u32,
    using_hashmap: bool,
}

impl VendorManagement {
    // Start with Vec (Stage 1)
    pub fn new() -> Self {
        VendorManagement {
            contracts_vec: Some(Vec::new()),
            contracts_hashmap: None,
            next_id: 1,
            using_hashmap: false,
        }
    }

    pub fn switch_to_hashmap(&mut self) {
        if !self.using_hashmap {
            println!("----switching from Vec to HashMap😁😁😁😁---------");
            
            if let Some(vec) = self.contracts_vec.take() {
                let mut hashmap = HashMap::new();
                for contract in vec {
                    hashmap.insert(contract.id, contract);
                }
                self.contracts_hashmap = Some(hashmap);
            }
            
            self.using_hashmap = true;
            println!("--------switched to hashmap 😊--------");
        }
    }

    
    pub fn add_contract(&mut self, vendor_name: String, contract_value: f64, end_date: String) -> u32 {
        let contract_new = Contract::new(vendor_name, contract_value, end_date, self.next_id);
        
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                hashmap.insert(self.next_id, contract_new);
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                vec.push(contract_new);
            }
        }
        
        let assigned_id = self.next_id;
        self.next_id += 1;
        assigned_id
    }

    
    pub fn view_contract(&self) {
        if self.using_hashmap {
            if let Some(ref hashmap) = self.contracts_hashmap {
                if hashmap.is_empty() {
                    println!("------oops we cant find you 😢😢--------");
                    return;
                }
                for (id, contract) in hashmap {
                    println!("ID: {}, {:?}", id, contract);
                }
            }
        } else {
            if let Some(ref vec) = self.contracts_vec {
                if vec.is_empty() {
                    println!("------oops we cant find you 😢😢--------");
                    return;
                }
                for contract in vec {
                    println!("{:?}", contract);
                }
            }
        }
    }

    pub fn mark_contract_expired(&mut self, id: u32) -> bool {
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                for (_id, contract) in hashmap {
                    if contract.id == id {
                        contract.status = ContractStatus::Expired;
                        return true;
                    }
                }
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                for contract in vec {
                    if contract.id == id {
                        contract.status = ContractStatus::Expired;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn mark_contract_canceled(&mut self, id: u32) -> bool {
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                for (_, contract) in hashmap {
                    if contract.id == id {
                        contract.status = ContractStatus::Canceled;
                        return true;
                    }
                }
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                for contract in vec {
                    if contract.id == id {
                        contract.status = ContractStatus::Canceled;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn mark_contracts_expired_by_date(&mut self, current_date: &str) -> usize {
        let mut count = 0;
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                for (_, contract) in hashmap {
                    if contract.end_date < current_date.to_string() {
                        contract.status = ContractStatus::Expired;
                        count += 1;
                    }
                }
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                for contract in vec {
                    if contract.end_date < current_date.to_string() {
                        contract.status = ContractStatus::Expired;
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn mark_contracts_canceled_by_date(&mut self, current_date: &str) -> usize {
        let mut count = 0;
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                for (_, contract) in hashmap {
                    if contract.end_date < current_date.to_string() {
                        contract.status = ContractStatus::Canceled;
                        count += 1;
                    }
                }
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                for contract in vec {
                    if contract.end_date < current_date.to_string() {
                        contract.status = ContractStatus::Canceled;
                        count += 1;
                    }
                }
            }
        }
        count
    }

    // Your exact return type (usize, bool)
    pub fn remove_contract_expired(&mut self) -> (usize, bool) {
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                let initial_len = hashmap.len();
                hashmap.retain(|_, contract| contract.status != ContractStatus::Expired);
                let removed_count = initial_len - hashmap.len();
                (removed_count, removed_count > 0)
            } else {
                (0, false)
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                let initial_len = vec.len();
                vec.retain(|contract| contract.status != ContractStatus::Expired);
                let removed_count = initial_len - vec.len();
                (removed_count, removed_count > 0)
            } else {
                (0, false)
            }
        }
    }

   
    pub fn remove_contract_canceled(&mut self) -> (usize, bool) {
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                let initial_len = hashmap.len();
                hashmap.retain(|_, contract| contract.status != ContractStatus::Canceled);
                let removed_count = initial_len - hashmap.len();
                (removed_count, removed_count > 0)
            } else {
                (0, false)
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                let initial_len = vec.len();
                vec.retain(|contract| contract.status != ContractStatus::Canceled);
                let removed_count = initial_len - vec.len();
                (removed_count, removed_count > 0)
            } else {
                (0, false)
            }
        }
    }

    pub fn edit_contract(
        &mut self,
        contract_id: u32,
        new_vendor_name: Option<String>,
        new_contract_value: Option<f64>,
        new_end_date: Option<String>,
    ) -> bool {
        if self.using_hashmap {
            if let Some(ref mut hashmap) = self.contracts_hashmap {
                for (_id, contract) in hashmap {
                    if contract.id == contract_id {
                        if let Some(name) = new_vendor_name {
                            contract.vendor_name = name;
                        }
                        if let Some(value) = new_contract_value {
                            contract.contract_value = value;
                        }
                        if let Some(date) = new_end_date {
                            contract.end_date = date;
                        }
                        return true;
                    }
                }
            }
        } else {
            if let Some(ref mut vec) = self.contracts_vec {
                for contract in vec {
                    if contract.id == contract_id {
                        if let Some(name) = new_vendor_name {
                            contract.vendor_name = name;
                        }
                        if let Some(value) = new_contract_value {
                            contract.contract_value = value;
                        }
                        if let Some(date) = new_end_date {
                            contract.end_date = date;
                        }
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn main() {
    let mut vendor_management = VendorManagement::new();

    println!("========= STAGE 1: Using Vec =========");
    
    let id1 = vendor_management.add_contract(String::from("Akpolo"), 500.23, String::from("2025-02-02"));
    let id2 = vendor_management.add_contract(String::from("josh"), 500.23, String::from("2024-08-30"));
    let id3 = vendor_management.add_contract(String::from("prince"), 300.23, String::from("2023-09-21"));
     vendor_management.add_contract(String::from("uche"), 200.23, String::from("2026-10-02"));

    println!("--------added all the vector contract--------");
    vendor_management.view_contract();
    vendor_management.mark_contract_canceled(id2);
    vendor_management.mark_contract_expired(id3);

    println!("----------marked either expired or canceled--------------");
    vendor_management.view_contract();
    vendor_management.remove_contract_canceled();
    vendor_management.remove_contract_expired();

    println!("----------removing expired / canceled--------------");
    vendor_management.view_contract();

     vendor_management.add_contract(String::from("jtmax"), 300.23, String::from("2023-09-21"));
     vendor_management.add_contract(String::from("ridwaan"), 200.23, String::from("2026-10-02"));

    vendor_management.mark_contracts_canceled_by_date("2024-09-21");

    println!("----------viewing the contract--------------");
    vendor_management.view_contract();

    println!("\n========= STAGE 2: Switching to HashMap =========");
    vendor_management.switch_to_hashmap();

   
    vendor_management.add_contract(String::from("NewVendor1"), 800.50, String::from("2025-03-15"));
    let hash_id2 = vendor_management.add_contract(String::from("NewVendor2"), 950.75, String::from("2024-11-20"));

    println!("--------added contracts to hashmap--------");


    vendor_management.mark_contract_expired(hash_id2);
        vendor_management.view_contract();
    vendor_management.remove_contract_expired();

    println!("----------after removing expired from hashmap--------------");
    vendor_management.view_contract();

    println!("\n========= STAGE 3: Testing Edit =========");
    
    vendor_management.edit_contract(
        id1,
        Some(String::from("Akpolo EDITED")),
        Some(999.99),
        Some(String::from("2025-12-31")),
    );

    println!("After editing contract {}:", id1);
    vendor_management.view_contract();
}