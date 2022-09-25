#[derive(Debug, Clone)]
pub enum DataType {
    Null,
    Bool(bool),
    Int(i32),
    Float(f32),
    Str(String),
}

impl ToString for DataType {
    fn to_string(&self) -> String {
        return match self {
            DataType::Null => String::from("Null"),
            DataType::Bool(val) => val.to_string(),
            DataType::Int(val) => val.to_string(),
            DataType::Float(val) => val.to_string(),
            DataType::Str(val) => val.to_owned(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct DBItem {
    pub key: u64,
    pub ident: String,
    pub data: DataType,
}
