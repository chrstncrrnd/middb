mod db;
mod utils;
mod commands;

use commands::Commands;
use db::DB;

use crate::utils::{get_input, parse_data_type};

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
                "all" => Ok(Commands::GetAll),
                _ => Err(format!("the command get does not have a '{}' subcommand", inputs[1]))
            }
        },
        "add" => {
            if inputs.len() <= 3{
                return Err("not enough arguments.".to_owned())
            }
            let ident = inputs[1];
            match parse_data_type(inputs){
                Ok(val) => Ok(Commands::AddEntry { ident: ident.to_owned(), data: val }),
                Err(err) => Err(err)
            }
        },
        "insert" => {
            let ident = inputs[1];
            if let Ok(key) = inputs[2].parse(){
                let data_type_w_value = inputs[2..3].to_vec();
                match parse_data_type(data_type_w_value){
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
                let data_type_w_value = inputs[2..3].to_vec();
                match parse_data_type(data_type_w_value){
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