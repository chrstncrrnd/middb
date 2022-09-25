use crate::db::{db_item::DataType, DB};

pub enum OrderedBy {
    KeyAsc,
    KeyDesc,
    IdentAsc,
    IdentDesc,
    ValAsc,
    ValDesc,
}

pub struct Cli<'a> {
    database: &'a DB,
}

impl Cli<'_> {
    pub fn add(&mut self, ident: String, val: DataType) {}
    pub fn remove(key: u64) {}
    pub fn get(key: u64) {}
    pub fn get_ident(ident: String) {}
    pub fn clear() {}
    pub fn get_all() {}
    pub fn get_all_ordered(by: OrderedBy) {}
}
