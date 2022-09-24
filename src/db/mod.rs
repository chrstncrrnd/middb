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
            
            // 1 mib buffer (should not rly exceed this lol) if so we have a big problem
            let buffer: &mut [u8; 1048576] = &mut [0_u8; 1048576];

            file.read(buffer).unwrap();
            
            let file_contents = String::from_utf8_lossy(buffer).to_string();
            
            self.items = Self::parse_file_contents(&file_contents);

            dbg!(self.items.clone());

        }else{
            File::create(DEFAULT_DB_FILE_PATH).unwrap();
        }
    }

    fn save(&self){

    }

    fn parse_file_contents(file_contents: &String) -> Vec<DBItem>{
        let mut database = Vec::<DBItem>::new();

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
                                    panic!("Syntax error on line {column_number}: {data_value} should be either true or false")
                                }
                            },
                            "Int" => {
                                match data_value.parse(){
                                    Ok(val) => {data = DataType::Int(val)},
                                    Err(_) => {panic!("Error whilst parsing integer on row: {row_number}")}
                                }
                            },
                            "Float" => {
                                match data_value.parse(){
                                    Ok(val) => {data = DataType::Float(val)},
                                    Err(_) => {panic!("Error whilst parsing float on row: {row_number}")}
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
                        panic!("Error whilst parsing database: row {row_number} incorrectly formatted")
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