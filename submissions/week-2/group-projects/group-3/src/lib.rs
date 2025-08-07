pub mod queue;
pub mod ticket;
pub mod ui;

pub use queue::{
    CustomerSupportQueue, add_ticket_to_queue, create_queue, get_ticket_from_queue,
    remove_ticket_from_queue, switch_to_hashmap, update_ticket_in_queue,
};
pub use ticket::{Priority, Ticket, create_ticket, priority_to_string, string_to_priority};
pub use ui::{add_ticket, display_menu, edit_ticket, get_user_input, remove_ticket, view_tickets};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_priority() {
        assert_eq!(string_to_priority("low"), Some(Priority::Low));
        assert_eq!(string_to_priority("MEDIUM"), Some(Priority::Medium));
        assert_eq!(string_to_priority("High"), Some(Priority::High));
        assert_eq!(string_to_priority("CRITICAL"), Some(Priority::Critical));
        assert_eq!(string_to_priority("invalid"), None);
    }

    #[test]
    fn test_priority_to_string() {
        assert_eq!(priority_to_string(&Priority::Low), "Low");
        assert_eq!(priority_to_string(&Priority::Medium), "Medium");
        assert_eq!(priority_to_string(&Priority::High), "High");
        assert_eq!(priority_to_string(&Priority::Critical), "Critical");
    }

    #[test]
    fn test_create_ticket() {
        let ticket = create_ticket(
            1,
            "John Doe".to_string(),
            "Cannot login to system".to_string(),
            Priority::High,
        );

        assert_eq!(ticket.id, 1);
        assert_eq!(ticket.customer_name, "John Doe");
        assert_eq!(ticket.issue_description, "Cannot login to system");
        assert_eq!(ticket.priority, Priority::High);
        assert_eq!(ticket.status, "Open");
    }

    #[test]
    fn test_create_queue() {
        let queue = create_queue();
        assert_eq!(queue.next_id, 1);
        assert_eq!(queue.use_hashmap, false);
        assert_eq!(queue.tickets.len(), 0);
        assert_eq!(queue.ticket_map.len(), 0);
    }

    #[test]
    fn test_add_ticket_stage1() {
        let mut queue = create_queue();

        let ticket_id = add_ticket_to_queue(
            &mut queue,
            "Alice".to_string(),
            "Password reset needed".to_string(),
            Priority::Medium,
        );

        assert_eq!(ticket_id, 1);
        assert_eq!(queue.next_id, 2);
        assert_eq!(queue.tickets.len(), 1);
        assert_eq!(queue.ticket_map.len(), 0);
        assert_eq!(queue.tickets[0].customer_name, "Alice");
        assert_eq!(queue.tickets[0].issue_description, "Password reset needed");
        assert_eq!(queue.tickets[0].priority, Priority::Medium);
    }

    #[test]
    fn test_view_tickets_stage1() {
        let mut queue = create_queue();

        add_ticket_to_queue(
            &mut queue,
            "Bob".to_string(),
            "Email not working".to_string(),
            Priority::Low,
        );
        add_ticket_to_queue(
            &mut queue,
            "Carol".to_string(),
            "System crash".to_string(),
            Priority::Critical,
        );

        let tickets = queue.tickets.iter().collect::<Vec<_>>();
        assert_eq!(tickets.len(), 2);
        assert_eq!(tickets[0].customer_name, "Bob");
        assert_eq!(tickets[1].customer_name, "Carol");
    }

    #[test]
    fn test_remove_ticket_stage1() {
        let mut queue = create_queue();

        add_ticket_to_queue(
            &mut queue,
            "Dave".to_string(),
            "Slow performance".to_string(),
            Priority::Medium,
        );
        add_ticket_to_queue(
            &mut queue,
            "Eve".to_string(),
            "Login issues".to_string(),
            Priority::High,
        );

        assert_eq!(queue.tickets.len(), 2);

        assert!(remove_ticket_from_queue(&mut queue, 1));
        assert_eq!(queue.tickets.len(), 1);
        assert_eq!(queue.tickets[0].id, 2);

        assert!(!remove_ticket_from_queue(&mut queue, 999));
        assert_eq!(queue.tickets.len(), 1);
    }

    #[test]
    fn test_switch_to_hashmap() {
        let mut queue = create_queue();

        add_ticket_to_queue(
            &mut queue,
            "Frank".to_string(),
            "Database error".to_string(),
            Priority::High,
        );
        add_ticket_to_queue(
            &mut queue,
            "Grace".to_string(),
            "UI bug".to_string(),
            Priority::Low,
        );

        assert_eq!(queue.tickets.len(), 2);
        assert_eq!(queue.ticket_map.len(), 0);
        assert_eq!(queue.use_hashmap, false);

        switch_to_hashmap(&mut queue);

        assert_eq!(queue.tickets.len(), 0);
        assert_eq!(queue.ticket_map.len(), 2);
        assert_eq!(queue.use_hashmap, true);
        assert_eq!(queue.ticket_map.get(&1).unwrap().customer_name, "Frank");
        assert_eq!(queue.ticket_map.get(&2).unwrap().customer_name, "Grace");
    }

    #[test]
    fn test_add_ticket_stage2() {
        let mut queue = create_queue();
        switch_to_hashmap(&mut queue);

        let ticket_id = add_ticket_to_queue(
            &mut queue,
            "Henry".to_string(),
            "Network timeout".to_string(),
            Priority::Critical,
        );

        assert_eq!(ticket_id, 1);
        assert_eq!(queue.ticket_map.len(), 1);
        assert_eq!(queue.tickets.len(), 0);
        assert_eq!(queue.ticket_map.get(&1).unwrap().customer_name, "Henry");
    }

    #[test]
    fn test_remove_ticket_stage2() {
        let mut queue = create_queue();
        switch_to_hashmap(&mut queue);

        add_ticket_to_queue(
            &mut queue,
            "Ivy".to_string(),
            "Memory leak".to_string(),
            Priority::High,
        );
        add_ticket_to_queue(
            &mut queue,
            "Jack".to_string(),
            "API timeout".to_string(),
            Priority::Medium,
        );

        assert_eq!(queue.ticket_map.len(), 2);

        assert!(remove_ticket_from_queue(&mut queue, 1));
        assert_eq!(queue.ticket_map.len(), 1);
        assert!(queue.ticket_map.get(&1).is_none());
        assert!(queue.ticket_map.get(&2).is_some());

        assert!(!remove_ticket_from_queue(&mut queue, 1));
        assert_eq!(queue.ticket_map.len(), 1);
    }

    #[test]
    fn test_get_ticket() {
        let mut queue = create_queue();

        add_ticket_to_queue(
            &mut queue,
            "Kate".to_string(),
            "File upload failed".to_string(),
            Priority::Medium,
        );

        let ticket = get_ticket_from_queue(&queue, 1);
        assert!(ticket.is_some());
        assert_eq!(ticket.unwrap().customer_name, "Kate");

        let ticket = get_ticket_from_queue(&queue, 999);
        assert!(ticket.is_none());
    }

    #[test]
    fn test_update_ticket_stage1() {
        let mut queue = create_queue();

        add_ticket_to_queue(
            &mut queue,
            "Liam".to_string(),
            "Old description".to_string(),
            Priority::Low,
        );

        assert!(update_ticket_in_queue(
            &mut queue,
            1,
            Some("Liam Updated".to_string()),
            None,
            Some(Priority::High)
        ));

        let ticket = get_ticket_from_queue(&queue, 1).unwrap();
        assert_eq!(ticket.customer_name, "Liam Updated");
        assert_eq!(ticket.issue_description, "Old description");
        assert_eq!(ticket.priority, Priority::High);
    }

    #[test]
    fn test_update_ticket_stage2() {
        let mut queue = create_queue();
        switch_to_hashmap(&mut queue);

        add_ticket_to_queue(
            &mut queue,
            "Mia".to_string(),
            "Old description".to_string(),
            Priority::Low,
        );

        assert!(update_ticket_in_queue(
            &mut queue,
            1,
            None,
            Some("New description".to_string()),
            None
        ));

        let ticket = get_ticket_from_queue(&queue, 1).unwrap();
        assert_eq!(ticket.customer_name, "Mia");
        assert_eq!(ticket.issue_description, "New description");
        assert_eq!(ticket.priority, Priority::Low);
    }

    #[test]
    fn test_update_nonexistent_ticket() {
        let mut queue = create_queue();

        assert!(!update_ticket_in_queue(
            &mut queue,
            999,
            Some("New name".to_string()),
            None,
            None
        ));
    }

    #[test]
    fn test_stage_progression_simulation() {
        let mut queue = create_queue();

        add_ticket_to_queue(
            &mut queue,
            "Peter".to_string(),
            "Issue 1".to_string(),
            Priority::Low,
        );
        add_ticket_to_queue(
            &mut queue,
            "Quinn".to_string(),
            "Issue 2".to_string(),
            Priority::Medium,
        );
        add_ticket_to_queue(
            &mut queue,
            "Rachel".to_string(),
            "Issue 3".to_string(),
            Priority::High,
        );

        assert_eq!(queue.tickets.len(), 3);
        assert_eq!(queue.use_hashmap, false);

        switch_to_hashmap(&mut queue);
        assert_eq!(queue.ticket_map.len(), 3);
        assert_eq!(queue.use_hashmap, true);

        add_ticket_to_queue(
            &mut queue,
            "Sam".to_string(),
            "Issue 4".to_string(),
            Priority::Critical,
        );
        add_ticket_to_queue(
            &mut queue,
            "Tina".to_string(),
            "Issue 5".to_string(),
            Priority::Low,
        );

        assert_eq!(queue.ticket_map.len(), 5);

        assert!(remove_ticket_from_queue(&mut queue, 1));
        assert_eq!(queue.ticket_map.len(), 4);

        assert!(update_ticket_in_queue(
            &mut queue,
            2,
            Some("Quinn Updated".to_string()),
            None,
            None
        ));
        let ticket = get_ticket_from_queue(&queue, 2).unwrap();
        assert_eq!(ticket.customer_name, "Quinn Updated");
    }

    #[test]
    fn test_empty_queue_operations() {
        let mut queue = create_queue();

        let tickets = queue.tickets.iter().collect::<Vec<_>>();
        assert_eq!(tickets.len(), 0);

        assert!(!remove_ticket_from_queue(&mut queue, 1));

        assert!(get_ticket_from_queue(&queue, 1).is_none());

        assert!(!update_ticket_in_queue(&mut queue, 1, None, None, None));
    }
}
