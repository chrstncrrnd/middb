use crate::db::{db_item::{DataType, DBItem}, DB};

pub enum Commands{
    GetFromKey(u64),
    GetFromIdent(String),
    AddEntry{ident: String, data: DataType},
    AddEntryWithKey{key: u64, ident: String, data: DataType},
    ModifyEntry{key: u64, ident: String, data: DataType},
    GetLen,
    GetAll,
    Save,
    Reload,
    Stop
}

impl Commands{
    fn print_column_header() {
        println!("{key:<6}|{ident:^15}|{value:>15}", key="Key", ident="Ident", value="Value");
    }
    fn print_column(key: u64, ident: String, data: DataType) {
        let value = data.to_string();
        println!("{key:<6}|{ident:^15}|{value:>15}")
    }
    pub fn run(&self, database: &mut DB) {
        match &self {
            Commands::GetFromKey(key) => {
                if let Some(i) = database.get_index_from_key(*key){
                    if let Some(row) = database.read_row(i){
                        Self::print_column_header();
                        Self::print_column(row.key, row.ident, row.data)
                    }else{
                        eprintln!("Error reading row from database.")
                    }
                }else{
                    eprintln!("Key: {key} does not exist")
                }
            },
            Commands::GetFromIdent(ident) => {
                if let Some(is) = database.get_index_from_ident(ident.clone()){
                    Self::print_column_header();
                    for i in is{
                        if let Some(DBItem{key, ident, data}) = database.read_row(i){
                            Self::print_column(key, ident, data);
                        }else{
                            eprintln!("Error reading database row: {i}");
                        }
                    }
                }
            },
            Commands::AddEntry { ident, data } => {
                let key = database.add_entry_last(ident.clone(), data.clone());
                println!("Added new column:");
                Self::print_column(key, ident.clone(), data.clone());
            },
            Commands::AddEntryWithKey { key, ident, data } => {
                database.add_entry(*key, ident.clone(), data.clone());
                println!("Added new column:");
                Self::print_column(*key, ident.clone(), data.clone());
                
            },
            Commands::ModifyEntry { key, ident, data } => {
                database.modify_entry_from_key(*key, ident.clone(), data.clone())
            },
            Commands::GetLen => {
                println!("Database size: {}", database.len())
            },
            Commands::GetAll => {
                Self::print_column_header();
                for row in database.get_all(){
                    Self::print_column(row.key, row.ident, row.data)
                }
            },
            Commands::Save => {
                database.save();
                println!("Database saved successfully")
            },
            Commands::Reload => {
                database.save();
                database.load();
                println!("Database reloaded successfully!");
            },
            Commands::Stop => {/* this should already be handled */},
        }
    }
}