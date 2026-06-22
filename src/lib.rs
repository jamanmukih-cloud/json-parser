pub struct JsonParser;

impl JsonParser {
    pub fn parse(input: &[u8]) -> Result<Document, Box<dyn std::error::Error>> {
        Ok(Document { data: input })
    }
}

pub struct Document<'a> {
    data: &'a [u8],
}

impl<'a> Document<'a> {
    pub fn get(&self, _path: &str) -> Result<Value, Box<dyn std::error::Error>> {
        Ok(Value::Null)
    }
}

pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
}
