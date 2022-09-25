mod db;
mod utils;
mod commands;

use commands::Commands;
use db::{DB, db_item::DataType};

use crate::utils::get_input;

fn main() {
    let mut database = DB::new();
    database.load();
    loop {
        if let Ok(user_input) = get_input("> "){
            
            match parse_input(user_input){
                Ok(command) => {
                    if let Commands::Stop = command{
                        break;
                    }
                    command.run(&mut database);
                },
                Err(err) => {
                    eprintln!("Syntax error: {}", err);
                }
            }
        }else{
            eprintln!("Error reading user input.")
        }
    }


    database.save();
}

fn parse_input(input: String) -> Result<Commands, String>{
    let inputs: Vec<&str> = input.trim().split(" ").map(|x| x.trim()).collect();

    match inputs[0]{
        "get" => {
            match inputs[1]{
                "key" => {
                    if let Ok(val) = inputs[2].parse(){
                        Ok(Commands::GetFromKey(val))
                    }else{
                        Err("expected a positive integer value for key".to_owned())
                    }
                },
                "ident" => Ok(Commands::GetFromIdent(inputs[2].to_owned())),
                "len" => Ok(Commands::GetLen),
                _ => Err("should be `get key <key>` or `get ident <ident>`".to_owned())
            }
        },
        "add" => {
            let ident = inputs[1];
            match parse_data_type(inputs[2], inputs[3]){
                Ok(val) => Ok(Commands::AddEntry { ident: ident.to_owned(), data: val }),
                Err(err) => Err(err)
            }
        },
        "insert" => {
            let ident = inputs[1];
            if let Ok(key) = inputs[2].parse(){
                match parse_data_type(inputs[3], inputs[4]){
                    Ok(data) => Ok(Commands::AddEntryWithKey { key, ident: ident.to_string(), data }),
                    Err(e) => Err(e),
                }
            }else{
                Err("key needs to be a positive integer".to_owned())
            }
        },
        "modify" => {
            let ident = inputs[1];
            if let Ok(key) = inputs[2].parse(){
                match parse_data_type(inputs[3], inputs[4]){
                    Ok(data) => Ok(Commands::ModifyEntry { key, ident: ident.to_owned(), data }),
                    Err(e) => Err(e),
                }
            }else{
                Err("key needs to be a positive integer".to_owned())
            }

        },
        "save" => Ok(Commands::Save),
        "reload" => Ok(Commands::Reload),
        "stop" => Ok(Commands::Stop),
        _ =>{Err(format!("command {} is not defined", inputs[0]))}
    }
}

fn parse_data_type(data_type: &str, data_value: &str) -> Result<DataType, String>{
    match data_type{
        "Null" => {
            Ok(DataType::Null)
        }
        "Bool" => {
            if data_value == "true" {
                Ok(DataType::Bool(true))
            } else if data_value == "false" {
                Ok(DataType::Bool(false))
            } else {
                Err("type 'Bool' can only be 'true' or 'false'".to_owned())
            }
        }
        "Int" => {
            match data_value.parse() {
                Ok(val) => Ok(DataType::Int(val)),
                Err(err) => Err(err.to_string()),
            }
        }
        "Float" => match data_value.parse() {
            Ok(val) =>  Ok(DataType::Float(val)),
            Err(err) => Err(err.to_string()),
        },
        "Str" => {
            Ok(DataType::Str(data_value.to_string()))
        }
        // if it doesnt fall into any of the other branches, something's wrong, panic
        _ => {
            return Err(format!("{} is not a valid data type", {data_type}))
        }
    }
}