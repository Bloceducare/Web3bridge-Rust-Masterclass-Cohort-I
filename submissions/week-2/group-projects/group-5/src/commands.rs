use crate::cli::Commands;
use crate::store::BudgetStore;

pub fn handle_command(command: Commands) {
    let mut store = BudgetStore::load();

    match command {
        Commands::Add { name, amount } => store.add(name, amount),
        Commands::View => store.view(),
        Commands::Remove { name } => store.remove(name),
        Commands::EditAmount { name, new_amount } => store.edit_amount(name, new_amount),
        Commands::EditName { old_name, new_name } => store.edit_name(old_name, new_name),
        Commands::ExportCsv => store.export_csv(), 
    }
}
