use core::panic;
use std::io::Write;
use std::{fs::File, io::Read};

use self::db_item::{DBItem, DataType};

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
                    DataType::Str(_) => String::from("Str:: ") + data.to_string().as_str(),
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

        for (row_number, row) in file_contents.split('\n').into_iter().enumerate() {
            let mut key: u64 = u64::default();
            let mut ident: String = String::default();
            let mut data: DataType = DataType::Null;

            for (column_number, mut column) in row.split(';').into_iter().enumerate() {
                column = column.trim();
                // goes through each column
                match column_number {
                    0 => {
                        // key
                        key = column.parse().unwrap();
                    }
                    1 => {
                        // ident
                        ident = column.to_owned();
                    }
                    2 => {
                        // data
                        // very very long way of doing this bruh
                        // parses the data
                        // split string into 2 pieces: its datatype and the actual value
                        let mut split = column.split("::");
                        // idk how to make this better but it works
                        let [data_type, data_value]: [String; 2] = [
                            split.next().unwrap().trim().to_owned(),
                            split.next().unwrap().trim().to_owned(),
                        ];
                        // check what datatype it is
                        match data_type.as_str() {
                            "Null" => {
                                data = DataType::Null;
                            }
                            "Bool" => {
                                if data_value == "true" {
                                    data = DataType::Bool(true);
                                } else if data_value == "false" {
                                    data = DataType::Bool(false);
                                } else {
                                    panic!("Error whilst parsing bool on row: {row_number}")
                                }
                            }
                            "Int" => {
                                match data_value.parse() {
                                    Ok(val) => data = DataType::Int(val),
                                    Err(err) => {
                                        panic!("Error whilst parsing integer on row {row_number}: {err}")
                                    }
                                }
                            }
                            "Float" => match data_value.parse() {
                                Ok(val) => data = DataType::Float(val),
                                Err(err) => {
                                    panic!("Error whilst parsing float on row {row_number}: {err}")
                                }
                            },
                            "Str" => {
                                data = DataType::Str(data_value);
                            }
                            // if it doesnt fall into any of the other branches, something's wrong, panic
                            _ => {
                                panic!("Syntax error on row: {row_number}, {data_type}' is not a valid data type. ")
                            }
                        }
                    }
                    _ => {
                        // there should not be more than 3 columns
                        panic!(
                            "Error whilst parsing database: row {row_number} incorrectly formatted"
                        )
                    }
                }
            }

            database.push(DBItem { key, ident, data })
        }

        database
    }
}
