use std::io::{self, Write};

use crate::db::db_item::DataType;

pub fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error reading from stdin");
    Ok(buffer.trim().to_owned())
}


pub fn parse_data_type(mut inputs: Vec<&str>) -> Result<DataType, String>{
    inputs = inputs.iter().map(|i| i.trim()).collect();

    let data_type = inputs[0];
    let data_value = inputs[1];
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
            let data = inputs.join(" ");
            let mut found_start = false;
            let mut start = 0;
            let mut end = 0;
            let mut prev_char = '\0';

            for (i, ch) in data.chars().into_iter().enumerate(){
                if ch == '\"' && prev_char != '\\'{
                    if !found_start{
                        start = i;
                        found_start = true;
                    }else{
                        end = i;
                        break;
                    }
                }
                prev_char = ch;
            }
            Ok(DataType::Str(data[start + 1..end].to_owned().replace("\\\"", "\"")))
        }
        // if it doesnt fall into any of the other branches, something's wrong, panic
        _ => {
            return Err(format!("{} is not a valid data type", {data_type}))
        }
    }
}
