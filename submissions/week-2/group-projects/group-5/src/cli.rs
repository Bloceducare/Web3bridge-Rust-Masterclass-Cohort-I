use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Budget Allocator", about = "Manage departmental budgets")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { name: String, amount: f64 },
    View,
    Remove { name: String },
    EditAmount { name: String, new_amount: f64 },
    EditName { old_name: String, new_name: String },
    ExportCsv,
}
