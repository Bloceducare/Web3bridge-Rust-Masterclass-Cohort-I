pub mod functions;
pub mod inventory;
pub mod test;

use crate::inventory::*;
// use crate::functions;

fn main() {
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
