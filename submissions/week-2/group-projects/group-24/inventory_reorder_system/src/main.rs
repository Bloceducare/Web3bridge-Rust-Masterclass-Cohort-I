// // Group 24: Inventory Reorder System
// // Description: Manage inventory reorder requests.
// // Stage 1:
// // Add reorder requests (item name, quantity, supplier).
// // View all requests.
// // Stage 2:
// // Remove fulfilled requests.
// // Stage 3:
// // Edit request details.
// // Cancel edits.
// // Implementation Tips: Use a Vec in Stage 1, then a HashMap with request ID as the key.
use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct Item {
        name: AllItems,
        quantity: u32,
        supplier: String,
        status: OrderState,
    }

    type pending_amount = u32;

    #[derive(Debug, Clone, Copy)]
    enum OrderState {
        Pending(pending_amount),
        Fulfilled,
    }

    #[derive(Debug, Clone, Copy)]
    enum AllItems {
        Biro,
        Stapler,
        Books,
        Pencil,
        Eraser,
    }

    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    enum RequestId {
        Ayoola,
        Nonso,
        Femi,
    }

    #[derive(Debug)]
    struct Inventory {
        data: Vec<HashMap<RequestId, Item>>,
    }

    impl Inventory {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn create_order(&mut self, item: Item, request_id: RequestId) {
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

        fn view_order_by_id(&self, request_id: RequestId) {
            let item = &self.data;

            for i in item {
                match i.get(&request_id) {
                    Some(data) => println!("Printed {:#?} requests: {:#?}", request_id, data),
                    _ => println!(""), // println!("{:#?}", item);
                }
            }
        }

        fn remove_fulfilled_order(&mut self) {
            self.data.retain(|order| {
                order.values().any(|item| match item.status {
                    OrderState::Fulfilled => false,
                    _ => true,
                })
            });
        }

        fn edit_order(&mut self, request_id: RequestId, new_item: Item) {
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

    let mut inventory = Inventory::new();

    let item_1 = Item {
        name: AllItems::Biro,
        quantity: 100,
        supplier: String::from("Eleganza"),
        status: OrderState::Pending(300),
    };
    let item_2 = Item {
        name: AllItems::Books,
        quantity: 520,
        supplier: String::from("MacMillian"),
        status: OrderState::Fulfilled,
    };
    let item_3 = Item {
        name: AllItems::Eraser,
        quantity: 20,
        supplier: String::from("Erasure"),
        status: OrderState::Pending(40),
    };

    inventory.create_order(item_1, RequestId::Femi);
    inventory.create_order(item_2, RequestId::Nonso);
    inventory.create_order(item_3, RequestId::Ayoola);

    inventory.view_order_by_id(RequestId::Femi);
    inventory.view_order_by_id(RequestId::Ayoola);
    inventory.view_order_by_id(RequestId::Nonso);

    inventory.remove_fulfilled_order();

    println!("Inventory: {:#?}", &inventory)
}
