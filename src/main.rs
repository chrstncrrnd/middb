// REMOVE THIS!!!!
#[allow(dead_code)]

mod db;
mod utils;

use db::DB;

fn main() {
    let mut data_base = DB::new();
    data_base.load()
}

