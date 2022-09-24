use core::panic;
use std::{fs::File, io::{Read}};

use self::db_item::{DataType, DBItem};

mod db_item;

pub struct DB{
    items: Vec<DBItem>
}
const DEFAULT_DB_FILE_PATH: &str = "./database.shdb";


impl DB{
    pub fn new() -> Self{
        Self{
            items: Vec::new()
        }
    }

    pub fn load(&mut self){
        if let Ok(mut file) = File::open(DEFAULT_DB_FILE_PATH){
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            self.items = Self::parse_file_contents(&contents);

            dbg!(self.items.clone());

        }else{
            File::create(DEFAULT_DB_FILE_PATH).unwrap();
        }
    }

    pub fn add_entry(&mut self, key: u64, ident: String, data: DataType){
        self.items.push(
            DBItem{
                key,
                ident,
                data
            }
        )
    }

    pub fn save(&self){
        let mut output = String::new();
        for row in self.items.iter(){
            
        }
    }

    fn parse_file_contents(file_contents: &String) -> Vec<DBItem>{
        let mut database = Vec::<DBItem>::new();
        let file_contents = file_contents.trim().to_owned();

        for (row_number, row) in file_contents.split("\n").into_iter().enumerate(){
            let mut key: u64 = u64::default();
            let mut ident: String = String::default();
            let mut data: DataType = DataType::Null;
            
            for (column_number, mut column) in row.split(";").into_iter().enumerate(){
                column = column.trim();
                match column_number{
                    0 => {
                        // key
                        key = column.parse().unwrap();
                    },
                    1 => {
                        // ident
                        ident = column.to_owned();
                    },
                    2 => {
                        // data
                        // very very long way of doing this bruh
                        let mut split = column.split("::");
                        let [data_type, data_value]: [String; 2] = [split.next().unwrap().trim().to_owned(), split.next().unwrap().trim().to_owned()];
                        match data_type.as_str() {
                            "Null" => {
                                data = DataType::Null;
                            },
                            "Bool" => {
                                if data_value == "true"{
                                    data = DataType::Bool(true);
                                }else if data_value == "false"{
                                    data = DataType::Bool(false);
                                }else{
                                    panic!("Error whilst parsing bool on row: {}", row_number + 1)
                                }
                            },
                            "Int" => {
                                match data_value.parse(){
                                    Ok(val) => {data = DataType::Int(val)},
                                    Err(err) => {panic!("Error whilst parsing integer on row {}: {err}, {data_value}", row_number + 1)}
                                }
                            },
                            "Float" => {
                                match data_value.parse(){
                                    Ok(val) => {data = DataType::Float(val)},
                                    Err(err) => {panic!("Error whilst parsing float on row {}: {err}", row_number + 1)}
                                }
                            },
                            "Str" => {
                                data = DataType::Str(data_value);
                            }
                            _ => {panic!("Error whilst parsing column: {column}")}
                        }
                    }
                    _ => {
                        // there should not be more than 3 columns
                        panic!("Error whilst parsing database: row {} incorrectly formatted", row_number + 1)
                    }
                }

                
            }

            database.push(DBItem{
                key,
                ident,
                data
            })
        }

        return database;
    }
}