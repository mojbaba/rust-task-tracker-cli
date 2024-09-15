pub trait Serialize {
    fn serialize(&self) -> String;
}

pub trait Deserialize: Sized {
    fn deserialize(json: &str) -> Result<Self, &'static str>;
}

impl Serialize for i32 {
    fn serialize(&self) -> String {
        format!("{}", self)
    }
}

impl Serialize for String {
    fn serialize(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl Serialize for bool{
    fn serialize(&self) -> String {
        format!("{}", self)
    }
}

impl Deserialize for String{
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        if json.starts_with('"') && json.ends_with('"'){
            Ok(json[1..json.len()-1].to_string())
        }else{
            Err("Invalid string")
        }
    }
}

impl Deserialize for i32{
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        json.parse().map_err(|_| "Invalid integer")
    }
}

impl Deserialize for bool {
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        match json {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err("Invalid bool")
        }
    }
}

use std::collections::HashMap;

// Helper function to parse JSON into a key-value map
pub fn parse_json_to_map(json: &str) -> Result<HashMap<String, String>, &'static str> {
    let json = json.trim();
    if !json.starts_with('{') || !json.ends_with('}') {
        return Err("Invalid JSON object");
    }

    let json_content = &json[1..json.len() - 1]; // strip braces
    let mut map = HashMap::new();

    for part in json_content.split(',') {
        let pair: Vec<&str> = part.split(':').map(|s| s.trim()).collect();
        if pair.len() != 2 {
            return Err("Invalid key-value pair");
        }

        let key = pair[0].trim_matches('"').to_string();
        let value = pair[1].to_string();
        map.insert(key, value);
    }

    Ok(map)
}