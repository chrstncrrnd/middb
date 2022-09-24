#[derive(Debug, Clone)]
pub enum DataType{
    Null,
    Bool(bool),
    Int(i32),
    Float(f32),
    Str(String)
}

#[derive(Debug, Clone)]
pub struct DBItem{
    pub key: u64,
    pub ident: String,
    pub data: DataType
}
