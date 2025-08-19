### Group 1: Inventory Stock Manager

- Description: Manage inventory stock for a small business.
- Stage 1:
  - Add items (name, quantity, price).
  - View all items in stock.
- Stage 2:
  - Remove items from inventory.
- Stage 3:
  - Edit item details (name, quantity, price).
  - Cancel edits if the user changes their mind.
- Implementation Tips: Use a `Vec<Item>` in Stage 1, transition to a `HashMap<String, Item>` in Stages 2 and 3 for lookup by item name.