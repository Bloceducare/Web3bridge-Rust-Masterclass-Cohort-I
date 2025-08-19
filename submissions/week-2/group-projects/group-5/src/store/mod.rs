use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Write},
    path::Path,
};
mod tests;

use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "budgets.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct BudgetStore {
    pub budgets: HashMap<String, f64>,
}

impl BudgetStore {
    pub fn load() -> Self {
        if Path::new(FILE_PATH).exists() {
            let file = File::open(FILE_PATH).expect("Failed to open file");
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap_or_else(|_| Self {
                budgets: HashMap::new(),
            })
        } else {
            Self {
                budgets: HashMap::new(),
            }
        }
    }

    pub fn save(&self) {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(FILE_PATH)
            .expect("Failed to open file for writing");

        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self).expect("Failed to write JSON");
    }

    pub fn add(&mut self, name: String, amount: f64) {
        self.budgets.insert(name.clone(), amount);
        self.save();
        println!("Added budget: {} = ${}", name, amount);
    }

    pub fn view(&self) {
        if self.budgets.is_empty() {
            println!("No budgets found.");
        } else {
            for (name, amount) in &self.budgets {
                println!("{}: ${}", name, amount);
            }
        }
    }

    pub fn remove(&mut self, name: String) {
        if self.budgets.remove(&name).is_some() {
            self.save();
            println!("Removed budget '{}'", name);
        } else {
            println!("Budget '{}' not found", name);
        }
    }

    pub fn edit_amount(&mut self, name: String, new_amount: f64) {
        if let Some(budget) = self.budgets.get_mut(&name) {
            *budget = new_amount;
            self.save();
            println!("Updated '{}' to ${}", name, new_amount);
        } else {
            println!("Budget '{}' not found", name);
        }
    }

    pub fn edit_name(&mut self, old_name: String, new_name: String) {
        if let Some(amount) = self.budgets.remove(&old_name) {
            self.budgets.insert(new_name.clone(), amount);
            self.save();
            println!("Renamed '{}' to '{}'", old_name, new_name);
        } else {
            println!("Budget '{}' not found", old_name);
        }
    }

    pub fn export_to_csv(&self, path: &str) {
    let mut file = File::create(path).expect("Failed to create CSV file");
    writeln!(file, "Category,Amount").expect("Failed to write header");

    for (category, amount) in &self.budgets {
        writeln!(file, "{},{}", category, amount).expect("Failed to write record");
    }

    println!("Exported to {}", path);
}

    pub fn export_csv(&self) {
        self.export_to_csv("budgets.csv");
    }


}
