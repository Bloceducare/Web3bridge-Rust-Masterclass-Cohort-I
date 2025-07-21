/*
 * Computer-Engineering Equipment Maintenance Scheduler
 * This program manages maintenance tasks for computer engineering equipment.
 */

use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MaintenanceType {
    Cleaning,
    FirmwareUpgrade,
    HardwareCheck,
    Calibration,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MaintenanceTask {
    equipment_name: String,
    date: DateTime<Local>,
    maintenance_type: MaintenanceType,
}

type TaskDbStage1 = Vec<MaintenanceTask>;

type TaskDbStage2 = HashMap<String, MaintenanceTask>;

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}

pub fn parse_date(date_str: &str) -> Option<DateTime<Local>> {
    date_str.parse::<DateTime<Local>>().ok()
}

pub fn pause() {
    println!("\nPress <Enter> to continue…");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
}

pub fn pick_maintenance_type() -> MaintenanceType {
    loop {
        println!("Select maintenance type:");
        println!("1) Cleaning");
        println!("2) Firmware Upgrade");
        println!("3) Hardware Check");
        println!("4) Calibration");
        let choice = read_line("Choice: ");
        match choice.as_str() {
            "1" => return MaintenanceType::Cleaning,
            "2" => return MaintenanceType::FirmwareUpgrade,
            "3" => return MaintenanceType::HardwareCheck,
            "4" => return MaintenanceType::Calibration,
            _ => println!("Invalid option! Try again."),
        }
    }
}

pub fn add_task_stage1(db: &mut TaskDbStage1) -> Result<(), String> {
    let equipment_name = read_line("Equipment name (e.g., FPGA Board): ");
    let date_str = read_line("Date (YYYY-MM-DD HH:MM:SS): ");
    let date = parse_date(&date_str).unwrap_or_else(|| {
        println!("Invalid date! Using current time.");
        Local::now()
    });
    let maintenance_type = pick_maintenance_type();
    db.push(MaintenanceTask {
        equipment_name: equipment_name.clone(),
        date,
        maintenance_type,
    });
    println!("Task for '{}' added.", equipment_name);
    Ok(())
}

pub fn view_tasks_stage1(db: &TaskDbStage1) {
    if db.is_empty() {
        println!("No tasks scheduled.");
    } else {
        for (i, task) in db.iter().enumerate() {
            println!(
                "{}. [{}] {} – {:?}",
                i + 1,
                task.date.format("%Y-%m-%d %H:%M:%S"),
                task.equipment_name,
                task.maintenance_type
            );
        }
    }
    pause();
}

pub fn migrate_stage1_to_stage2(stage1: &TaskDbStage1) -> TaskDbStage2 {
    let mut map = HashMap::new();
    for task in stage1 {
        map.insert(task.equipment_name.clone(), task.clone());
    }
    map
}

pub fn add_task_stage2(db: &mut TaskDbStage2) -> Result<(), String> {
    let equipment_name = read_line("Equipment name (unique key): ");
    if db.contains_key(&equipment_name) {
        return Err("Equipment already exists! Use edit instead.".to_string());
    }
    let date_str = read_line("Date (YYYY-MM-DD HH:MM:SS): ");
    let date = parse_date(&date_str).unwrap_or_else(|| {
        println!("Invalid date! Using current time.");
        Local::now()
    });
    let maintenance_type = pick_maintenance_type();
    db.insert(
        equipment_name.clone(),
        MaintenanceTask {
            equipment_name,
            date,
            maintenance_type,
        },
    );
    Ok(())
}

pub fn view_tasks_stage2(db: &TaskDbStage2) {
    if db.is_empty() {
        println!("No tasks.");
    } else {
        for (key, task) in db {
            println!(
                "{}: [{}] {:?}",
                key,
                task.date.format("%Y-%m-%d %H:%M:%S"),
                task.maintenance_type
            );
        }
    }
    pause();
}

pub fn remove_task_stage2(db: &mut TaskDbStage2) -> Result<(), String> {
    let name = read_line("Equipment name to remove: ");
    match db.remove(&name) {
        Some(_) => {
            println!("Removed '{}'.", name);
            Ok(())
        }
        None => Err("Equipment not found.".to_string()),
    }
}

pub fn edit_task_stage3(db: &mut TaskDbStage2) -> Result<(), String> {
    let name = read_line("Equipment name to edit: ");
    let Some(mut task) = db.remove(&name) else {
        return Err("Equipment not found.".to_string());
    };

    println!(
        "Editing task for '{}'. Leave blank to keep current value.",
        name
    );

    let new_name = read_line(&format!("New name [{}]: ", task.equipment_name));
    if !new_name.is_empty() {
        task.equipment_name = new_name;
    }

    let new_date_str = read_line(&format!(
        "New date [{}]: ",
        task.date.format("%Y-%m-%d %H:%M:%S")
    ));
    if !new_date_str.is_empty() {
        task.date = parse_date(&new_date_str).ok_or("Invalid date")?;
    }

    let new_type_input = read_line(&format!(
        "Change type? (y/n) [Current: {:?}]: ",
        task.maintenance_type
    ));
    if new_type_input.eq_ignore_ascii_case("y") {
        task.maintenance_type = pick_maintenance_type();
    }

    let confirm = read_line("Save changes? (y/n): ");
    if confirm.eq_ignore_ascii_case("y") {
        db.insert(task.equipment_name.clone(), task);
        println!("Changes saved.");
    } else {
        db.insert(name, task);
        println!("Edit cancelled.");
    }
    Ok(())
}

pub fn main() {
    let mut stage1_db: TaskDbStage1 = Vec::new();
    let mut stage2_db: TaskDbStage2 = HashMap::new();
    let mut migrated = false;

    loop {
        println!("\n==== COMPUTER-ENGINEERING MAINTENANCE SCHEDULER ====");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Remove task (Stage 2)");
        println!("4. Edit task (Stage 3)");
        println!("5. Migrate to HashMap (Stage 2)");
        println!("0. Exit");
        let choice = read_line("Choice: ");

        match choice.as_str() {
            "1" => {
                let result = if !migrated {
                    add_task_stage1(&mut stage1_db)
                } else {
                    add_task_stage2(&mut stage2_db)
                };
                if let Err(e) = result {
                    println!("Error: {}", e);
                } else {
                    if !migrated {
                        stage2_db = migrate_stage1_to_stage2(&stage1_db);
                    } else {
                        stage1_db = stage2_db.values().cloned().collect();
                    }
                }
            }
            "2" => {
                if !migrated {
                    view_tasks_stage1(&stage1_db);
                } else {
                    view_tasks_stage2(&stage2_db);
                }
            }
            "3" => {
                if !migrated {
                    println!("Migrate first (option 5) to enable removal.");
                } else if let Err(e) = remove_task_stage2(&mut stage2_db) {
                    println!("Error: {}", e);
                } else {
                    stage1_db = stage2_db.values().cloned().collect();
                }
            }
            "4" => {
                if !migrated {
                    println!("Migrate first (option 5) to enable editing.");
                } else if let Err(e) = edit_task_stage3(&mut stage2_db) {
                    println!("Error: {}", e);
                } else {
                    stage1_db = stage2_db.values().cloned().collect();
                }
            }
            "5" => {
                if !migrated {
                    stage2_db = migrate_stage1_to_stage2(&stage1_db);
                    migrated = true;
                    println!("Migrated to HashMap storage.");
                } else {
                    println!("Already migrated.");
                }
            }
            "0" => {
                println!("\n");
                println!("╔══════════════════════════════════════════════════════════════╗");
                println!("║                                                              ║");
                println!("║     A J T E C H   S A Y S   T H A N K   Y O U !            ║");
                println!("║                                                              ║");
                println!("║  To every Computer Engineer, Hardware Technician,            ║");
                println!("║  FPGA Designer, Embedded-Systems Developer, IoT Innovator,   ║");
                println!("║  Network Architect, Cyber-Security Specialist,               ║");
                println!("║  AI/ML Accelerator Builder, and Robotics Integrator—         ║");
                println!("║                                                              ║");
                println!("║  Your trust in AJTECH keeps critical equipment running       ║");
                println!("║  at peak performance today and into the future.              ║");
                println!("║                                                              ║");
                println!("║                        AJTECH!                               ║");
                println!("╚══════════════════════════════════════════════════════════════╝");
                println!("\n");
                break;
            }
            _ => println!("Invalid menu choice."),
        }
    }
}
