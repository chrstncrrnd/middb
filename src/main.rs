// REMOVE THIS!!!!
mod cli;
#[allow(dead_code)]
mod db;
mod utils;

use db::db_item::DataType;
use db::DB;

fn main() {
    let mut database = DB::new();
    database.load();
    database.add_entry_last("number".to_owned(), DataType::Int(19));
    database.add_entry_last("number".to_owned(), DataType::Int(10));
    database.add_entry_last("number".to_owned(), DataType::Float(100.1));
    database.save();
}
