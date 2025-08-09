# 24 Management-Related Rust Projects for Students

These are the projects for the 24 Groups we have, if you don't have a group send a message on discord and I'll add you an existing group,read instructions carefully, these projects builds on what we've learnt in the past few weeks. Each project should and must be designed as a command-line interactive application. The projects progress through three stages: Stage 1 (add and view), Stage 2 (remove), and Stage 3 (edit and cancel). To complete this project you must use features like the Vec, hashmap, enums and Option think about some of these things and use separate functions for each menu options.

### General Implementation Notes

- Structure: Use a `loop` for an interactive menu with options like `1. Add`, `2. View`, `3. Remove` (Stage 2), and `4. Edit` (Stage 3). Implement each option as a separate function for modularity.
- Data Storage: Start with a `Vec<Struct>` in Stage 1 to make your lives easier. Then in Stages 2 and 3, transition to a `HashMap` for efficient lookups, using a unique identifier as the key.
- Rust Libraries: Use `std::io` for input/output, `serde` for potential serialization, and `chrono` for date handling where relevant (e.g., contracts, bookings).
- Error Handling: Use Rust’s `Result` and `Option` types to handle invalid inputs and missing data.
- Cancel Functionality: In Stage 3, implement a confirmation step for edits (e.g., “Save changes? (y/n)”) to allow canceling.

You are not limited to these instructions, do as occasion serves you in your respective projects, This is just to somehow aid your ideation process and help you gain speed.

Enjoy!

---

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

### Group 2: Employee Directory

- Description: Manage employee contact information.
- Stage 1:
  - Add employees (name, department, phone).
  - View all employees.
- Stage 2:
  - Remove employees by name.
- Stage 3:
  - Edit employee details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with employee ID or name as the key.

### Group 3: Customer Support Queue

- Description: Manage customer support tickets in a queue.
- Stage 1:
  - Add tickets (customer name, issue description, priority).
  - View all tickets.
- Stage 2:
  - Remove resolved tickets.
- Stage 3:
  - Edit ticket details (e.g., update priority or description).
  - Cancel edits.
- Implementation Tips: Use a `Vec` for tickets in Stage 1, switch to a `HashMap` with ticket ID as the key.

### Group 4: Event RSVP Tracker

- Description: Manage RSVPs for an event.
- Stage 1:
  - Add attendees (name, email, RSVP status).
  - View all attendees.
- Stage 2:
  - Remove attendees.
- Stage 3:
  - Edit attendee details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, then a `HashMap` with email as the key.

### Group 5: Budget Allocator

- Description: Manage departmental budgets.
- Stage 1:
  - Add budget categories (name, allocated amount).
  - View all budgets.
- Stage 2:
  - Remove budget categories.
- Stage 3:
  - Edit budget amounts or names.
  - Cancel edits.
- Implementation Tips: Start with a `Vec`, then use a `HashMap` with category name as the key.

### Group 6: Vendor Contract Tracker

- Description: Manage vendor contracts.
- Stage 1:
  - Add contracts (vendor name, contract value, end date).
  - View all contracts.
- Stage 2:
  - Remove expired or canceled contracts.
- Stage 3:
  - Edit contract details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, switch to a `HashMap` with vendor name or contract ID as the key.

### Group 7: Asset Tracker

- Description: Manage company assets (e.g., laptops, furniture).
- Stage 1:
  - Add assets (name, serial number, value).
  - View all assets.
- Stage 2:
  - Remove disposed assets.
- Stage 3:
  - Edit asset details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with serial number as the key.

### Group 8: Event Volunteer Coordinator

- Description: Manage volunteers for events.
- Stage 1:
  - Add volunteers (name, role, contact).
  - View all volunteers.
- Stage 2:
  - Remove volunteers.
- Stage 3:
  - Edit volunteer details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, then a `HashMap` with volunteer name or ID as the key.

### Group 9: Meeting Room Booker

- Description: Manage meeting room reservations.
- Stage 1:
  - Add bookings (room name, date, time, capacity).
  - View all bookings.
- Stage 2:
  - Remove bookings.
- Stage 3:
  - Edit booking details.
  - Cancel edits.
- Implementation Tips: Store bookings in a `Vec`, then use a `HashMap` with a composite key (room + date).

### Group 10: Expense Report Manager

- Description: Manage employee expense reports.
- Stage 1:
  - Add expenses (employee name, amount, category).
  - View all expenses.
- Stage 2:
  - Remove approved or rejected expenses.
- Stage 3:
  - Edit expense details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, switch to a `HashMap` with expense ID as the key.

### Group 11: Customer Database

- Description: Manage customer information for a business.
- Stage 1:
  - Add customers (name, email, phone).
  - View all customers.
- Stage 2:
  - Remove customers.
- Stage 3:
  - Edit customer details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with email or customer ID as the key.

### Group 12: Marketing Campaign Tracker

- Description: Manage marketing campaigns.
- Stage 1:
  - Add campaigns (name, budget, start date).
  - View all campaigns.
- Stage 2:
  - Remove completed campaigns.
- Stage 3:
  - Edit campaign details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, then a `HashMap` with campaign name as the key.

### Group 13: Training Program Manager

- Description: Manage employee training programs.
- Stage 1:
  - Add training sessions (course name, date, trainer).
  - View all sessions.
- Stage 2:
  - Remove completed sessions.
- Stage 3:
  - Edit session details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, switch to a `HashMap` with course ID as the key.

### Group 14: Supply Order Tracker

- Description: Manage supply orders for a business.
- Stage 1:
  - Add orders (item name, quantity, supplier).
  - View all orders.
- Stage 2:
  - Remove fulfilled orders.
- Stage 3:
  - Edit order details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with order ID as the key.

### Group 15: Fleet Vehicle Manager

- Description: Manage a company’s vehicle fleet.
- Stage 1:
  - Add vehicles (model, license plate, status).
  - View all vehicles.
- Stage 2:
  - Remove retired vehicles.
- Stage 3:
  - Edit vehicle details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, then a `HashMap` with license plate as the key.

### Group 16: Donation Tracker

- Description: Manage donations for a nonprofit.
- Stage 1:
  - Add donations (donor name, amount, date).
  - View all donations.
- Stage 2:
  - Remove incorrect entries.
- Stage 3:
  - Edit donation details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with donor name or donation ID as the key.

### Group 17: Project Resource Allocator

- Description: Manage resource allocation for projects.
- Stage 1:
  - Add resources (resource name, project, quantity).
  - View all allocations.
- Stage 2:
  - Remove allocations.
- Stage 3:
  - Edit allocation details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, then a `HashMap` with resource name as the key.

### Group 18: Customer Feedback Logger

- Description: Manage customer feedback.
- Stage 1:
  - Add feedback (customer name, comment, rating).
  - View all feedback.
- Stage 2:
  - Remove feedback entries.
- Stage 3:
  - Edit feedback details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, switch to a `HashMap` with feedback ID as the key.

### Group 19: Maintenance Scheduler

- Description: Manage maintenance schedules for equipment.
- Stage 1:
  - Add maintenance tasks (equipment name, date, type).
  - View all tasks.
- Stage 2:
  - Remove completed tasks.
- Stage 3:
  - Edit task details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with equipment name as the key.

### Group 20: Hiring Candidate Tracker

- Description: Manage job candidates during hiring.
- Stage 1:
  - Add candidates (name, position, contact).
  - View all candidates.
- Stage 2:
  - Remove candidates.
- Stage 3:
  - Edit candidate details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, then a `HashMap` with candidate name or ID as the key.

### Group 21: Subscription Manager

- Description: Manage business subscriptions (e.g., software licenses).
- Stage 1:
  - Add subscriptions (service name, cost, renewal date).
  - View all subscriptions.
- Stage 2:
  - Remove expired subscriptions.
- Stage 3:
  - Edit subscription details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, then a `HashMap` with service name as the key.

### Group 22: Facility Space Manager

- Description: Manage office space allocations.
- Stage 1:
  - Add space assignments (space name, occupant, purpose).
  - View all assignments.
- Stage 2:
  - Remove assignments.
- Stage 3:
  - Edit assignment details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` initially, then a `HashMap` with space name as the key.

### Group 23: Sales Lead Tracker

- Description: Manage sales leads for a business.
- Stage 1:
  - Add leads (name, contact, potential value).
  - View all leads.
- Stage 2:
  - Remove converted or lost leads.
- Stage 3:
  - Edit lead details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` for Stage 1, then a `HashMap` with lead ID as the key.

### Group 24: Inventory Reorder System

- Description: Manage inventory reorder requests.
- Stage 1:
  - Add reorder requests (item name, quantity, supplier).
  - View all requests.
- Stage 2:
  - Remove fulfilled requests.
- Stage 3:
  - Edit request details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, then a `HashMap` with request ID as the key.
