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

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: AllItems,
    pub quantity: u32,
    pub supplier: String,
    pub status: OrderState,
}

type PendingAmount = u32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderState {
    Pending(PendingAmount),
    Fulfilled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllItems {
    Biro,
    Stapler,
    Books,
    Pencil,
    Eraser,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum RequestId {
    Ayoola,
    Nonso,
    Femi,
}

#[derive(Debug)]
pub struct Inventory {
    pub data: Vec<HashMap<RequestId, Item>>,
}
