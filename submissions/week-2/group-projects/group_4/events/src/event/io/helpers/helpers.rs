use crate::event::models::models::{Event, RsvpStatus};

pub fn start_app() {
    let mut event = Event::new();
    menu(&mut event);
}

fn menu(event: &mut Event) {
    loop {
        console_log(
            "What would you like to do? \n\
                 1. Add an attendee \n\
                 2. View all attendees \n\
                 3. Remove an attendee \n\
                 4. Edit an attendee \n\
                 5. Exit \n\
                 "
                .to_string(),
        );

        let input: Option<String> = collect_input();
        let input = input.unwrap().parse::<u8>().ok().unwrap();
        match input {
            1 => add_option(event),
            2 => {
                console_log("View all attendees".to_string());

                println!("{:?}", event.get_attendees());
            }
            3 => remove_option(event),
            4 => edit_option(event),
            5 => break,
            _ => {
                println!("invalid input, please try again");
                start_app();
            }
        }
    }
}

fn add_option(event: &mut Event) {
    console_log("Add an attendee".to_string());

    console_log("input name :: ".to_string());
    let name = collect_input();
    console_log("input email :: ".to_string());
    let email = collect_input();
    console_log("input decision :: ".to_string());

    let rsvp = collect_input().unwrap().parse::<usize>().ok().unwrap();

    let rsvp_status = RsvpStatus::check_variant(rsvp);
    if rsvp_status.is_err() {
        console_log(rsvp_status.unwrap_err().to_string());
        add_option(event);
    }

    let result = event.add_attendee(name.unwrap(), email.unwrap(), rsvp);

    if result.is_err() {
        console_log(result.unwrap_err().to_string());
        add_option(event);
    }
    console_log("attendee added successfully ...".to_string())
}

fn remove_option(event: &mut Event) {
    console_log("Remove an attendee with the index".to_string());
    console_log("input index :: ".to_string());
    let index = collect_input().unwrap().parse::<usize>().ok().unwrap();
    let result = event.remove_attendee(index);
    if result.is_err() {
        console_log(result.unwrap_err().to_string());
        remove_option(event);
    }
    console_log("successfully removed::: ".to_string());
}

fn edit_option(event: &mut Event) {
    console_log("Edit an attendee".to_string());

    console_log("input the index ".to_string());
    let index: usize = collect_input().unwrap().parse::<usize>().ok().unwrap();

    console_log("input the name:: ".to_string());
    let new_name = collect_input().unwrap();

    console_log("input the email:: ".to_string());
    let new_email = collect_input().unwrap();

    console_log("new decision (0 || 1 || 2)".to_string());
    let rsvp = collect_input().unwrap().parse::<usize>().ok().unwrap();
    let rsvp_status = RsvpStatus::check_variant(rsvp).unwrap();

    console_log("save changes(y/n)".to_string());

    save_changes(event, index, new_name, new_email, rsvp_status);
}
fn save_changes(event: &mut Event, index: usize, new_name: String, new_email: String, rsvp_status: RsvpStatus) {
    let input = collect_input().unwrap();

    match input.as_str() {
        "y" => {
            let result = event
                .update_attendee(index, new_name, new_email, rsvp_status);
            // .unwrap();
            if result.is_err() {
                console_log(result.unwrap_err().to_string());
                menu(event);
                // save_changes(event, index, new_name.clone(), new_email, rsvp_status);
            }
            console_log("saved successfully...".to_string());
        }
        "n" => {
            println!("changes not saved");
            menu(event);
        }
        _ => {
            console_log("invalid input ".to_string());
            edit_option(event);
        }
    }
}
fn collect_input() -> Option<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_string())
}
fn console_log(out_put: String) {
    println!("{}", out_put);
}
