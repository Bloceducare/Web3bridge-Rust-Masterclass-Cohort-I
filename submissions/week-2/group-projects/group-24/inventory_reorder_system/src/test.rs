use crate::Inventory;
use crate::inventory::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Inventory {
        let mut inventory = Inventory::new();

        let item_1 = Item {
            name: AllItems::Biro,
            quantity: 100,
            supplier: String::from("Eleganza"),
            status: OrderState::Pending(300),
        };

        inventory.create_order(item_1, RequestId::Femi);

        inventory
    }

    #[test]
    fn test_create_order() {
        let inventory = setup();

        let mut found = false;

        for order in &inventory.data {
            if let Some(item) = order.get(&RequestId::Femi) {
                assert_eq!(item.name, AllItems::Biro);
                assert_eq!(item.quantity, 100);
                found = true;
            }
        }

        assert!(found, "Femi's order was not found.");
    }

    #[test]
    fn test_view_order_by_id_output() {
        let inventory = setup();
        inventory.view_order_by_id(RequestId::Femi);
        inventory.view_order_by_id(RequestId::Nonso);
    }

    #[test]
    fn test_remove_fulfilled_order() {
        let mut inventory = setup();

        inventory.remove_fulfilled_order();

        let mut has_fulfilled = false;
        for map in &inventory.data {
            for item in map.values() {
                if item.status == OrderState::Fulfilled {
                    has_fulfilled = true;
                }
            }
        }

        assert!(!has_fulfilled, "Fulfilled items should have been removed.");
    }

    #[test]
    fn test_edit_order() {
        let mut inventory = setup();

        let updated_item = Item {
            name: AllItems::Stapler,
            quantity: 999,
            supplier: "New Supplier".to_string(),
            status: OrderState::Pending(200),
        };

        inventory.edit_order(RequestId::Femi, updated_item.clone());

        let mut found = false;

        for map in &inventory.data {
            if let Some(item) = map.get(&RequestId::Femi) {
                assert_eq!(item.name, AllItems::Stapler);
                assert_eq!(item.quantity, 999);
                assert_eq!(item.supplier, "New Supplier");
                assert_eq!(item.status, OrderState::Pending(200));
                found = true;
            }
        }

        assert!(found, "Edited item for Femi not found.");
    }
}
