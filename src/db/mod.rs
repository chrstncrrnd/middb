use core::panic;
use std::io::Write;
use std::{fs::File, io::Read};

use self::db_item::{DBItem, DataType};

use crate::utils;

pub mod db_item;

/// # Db
/// Saved with 3 columns seperated by ';'
/// eg: `0; ident; Int:: 10`
/// - First item is the key (unique and should be incremental although not necessary)
/// - Second is the identifier, human friendly identifier. Doesn't need to be unique
/// - Third item is the data that is stored formatted as: <DataType>:: <Value>
///
/// Note: you can't store semi-colons or colons because icba to make it work lol

#[derive(Clone, Debug)]
pub struct DB {
    items: Vec<DBItem>,
}
const DEFAULT_DB_FILE_PATH: &str = "./database.shdb";

impl DB {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    // load the database from disk
    pub fn load(&mut self) {
        // open file if exists
        if let Ok(mut file) = File::open(DEFAULT_DB_FILE_PATH) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            self.items = Self::parse_file_contents(&contents);
        } else {
            File::create(DEFAULT_DB_FILE_PATH).unwrap();
        }
    }

    pub fn add_entry(&mut self, key: u64, ident: String, data: DataType) {
        self.items.push(DBItem { key, ident, data })
    }

    pub fn get_index_from_key(&self, key: u64) -> Option<usize> {
        for (i, item) in self.items.iter().enumerate() {
            if item.key == key {
                return Some(i);
            }
        }
        None
    }

    // returns a vector of all of the indexes of all of the items which match the ident
    pub fn get_index_from_ident(&self, ident: String) -> Option<Vec<usize>> {
        let mut indexes = Vec::<usize>::new();
        for (i, item) in self.items.iter().enumerate() {
            if item.ident == ident {
                indexes.push(i);
            }
        }
        Some(indexes)
    }

    pub fn modify_entry_from_key(&mut self, key: u64, ident: String, data: DataType) {
        if let Some(index) = self.get_index_from_key(key) {
            if let Some(entry) = self.items.get_mut(index) {
                *entry = DBItem { key, ident, data }
            }
        }
    }

    pub fn add_entry_last(&mut self, ident: String, data: DataType) -> u64{
        let mut key = 0;
        if let Some(last) = self.items.last() {
            key = last.key + 1;
        }
        self.add_entry(key, ident, data);
        key
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn read_row(&self, i: usize) -> Option<DBItem> {
        if let Some(val) = self.items.get(i) {
            Some(val.clone())
        } else {
            None
        }
    }
    pub fn get_all(&self) -> Vec<DBItem> {
        return self.items.clone();
    }

    pub fn save(&self) {
        let mut output = String::new();
        for row in self.items.iter() {
            let data = row.data.clone();
            output.push_str(format!("{}; ", row.key).as_str());
            output.push_str(format!("{}; ", row.ident).as_str());
            output.push_str(
                match data {
                    DataType::Null => String::from("Null:: ") + data.to_string().as_str(),
                    DataType::Bool(_) => String::from("Bool:: ") + data.to_string().as_str(),
                    DataType::Int(_) => String::from("Int:: ") + data.to_string().as_str(),
                    DataType::Float(_) => String::from("Float:: ") + data.to_string().as_str(),
                    DataType::Str(_) => String::from("Str:: ") + "\"" + data.to_string().as_str() + "\"",
                }
                .as_str(),
            );
            output.push('\n')
        }
        let mut file = File::options()
            .write(true)
            .open(DEFAULT_DB_FILE_PATH)
            .unwrap();
        let bytes_ammount = file.write(output.as_bytes()).unwrap();
        if bytes_ammount != output.len(){
            panic!("Couldn't save all of the file!")
        }
    }

    // parses the file contents
    fn parse_file_contents(file_contents: &str) -> Vec<DBItem> {
        let file_contents = file_contents.trim().to_owned();

        // if the file is empty, you don't need to parse anything
        if file_contents.is_empty() {
            return Vec::new();
        }

        let mut database = Vec::<DBItem>::new();

        
        for (row_number, row) in file_contents.split("\n").into_iter().enumerate(){
            if let Some(item) = Self::parse_row(row, row_number){
                database.push(item);
            }
        }
        

        database
    }

    fn parse_row(mut row: &str, row_number: usize) -> Option<DBItem>{
        row = row.trim();
        
        if row.is_empty() {return None};
        
        let mut item = DBItem{
            key: 0,
            ident: String::new(),
            data: DataType::Null
        };
        
        
        let row: Vec<&str> = row.splitn(3, ";").collect();

        // parse key
        if let Ok(k) = row[0].trim().parse::<u64>(){
            item.key = k;
        }else{
            eprintln!("Error parsing key on row: {row_number}");
            return None;
        }

        // parse ident
        item.ident = row[1].trim().to_owned();


        let data_type_w_value = row[2].trim().splitn(2, "::").collect::<Vec<&str>>();
        

        match utils::parse_data_type(data_type_w_value){
            Ok(val) => item.data = val,
            Err(err) => eprintln!("Error: {err} while parsing data on line: {row_number}")
        }

        Some(item)
    }

}
