use crate::inventory::*;

use std::collections::HashMap;

impl Inventory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn create_order(&mut self, item: Item, request_id: RequestId) {
        let new_item = Item {
            name: item.name,
            quantity: item.quantity,
            supplier: item.supplier,
            status: item.status,
        };

        let mut hashed_data = HashMap::new();
        hashed_data.insert(request_id, new_item);

        self.data.push(hashed_data);
    }

    pub fn view_order_by_id(&self, request_id: RequestId) {
        let item = &self.data;

        for i in item {
            match i.get(&request_id) {
                Some(data) => println!("Printed {:#?} requests: {:#?}", request_id, data),
                _ => println!(""), // println!("{:#?}", item);
            }
        }
    }

    pub fn remove_fulfilled_order(&mut self) {
        self.data.retain(|order| {
            order.values().any(|item| match item.status {
                OrderState::Fulfilled => false,
                _ => true,
            })
        });
    }


    

    pub fn edit_order(&mut self, request_id: RequestId, new_item: Item) {
        for order in &mut self.data {
            if let Some(item) = order.get_mut(&request_id) {
                item.name = new_item.name;
                item.quantity = new_item.quantity;
                item.supplier = new_item.supplier.clone();
                item.status = new_item.status;
            }
        }
    }
}
