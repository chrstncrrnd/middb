// REMOVE THIS!!!!
#[allow(dead_code)]

mod db;
mod utils;
mod cli;

use db::db_item::DataType;
use db::DB;

fn main() {
    let mut database = DB::new();
    database.load();
    database.add_entry(3, "sussy baka 03".to_owned(), DataType::Str("ther is an imposter among us".to_owned()));
    database.save();

}

