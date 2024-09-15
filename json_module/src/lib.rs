pub trait Serialize {
    fn serialize(&self) -> String;
}

pub trait Deserialize: Sized {
    fn deserialize(json: &str) -> Result<Self, &'static str>;
}

impl Serialize for String {
    fn serialize(&self) -> String {
        format!("\"{}\"", self)
    }
}

impl Deserialize for String {
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        Ok(json.to_string())
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> String {
        format!("{}", self)
    }
}

impl Deserialize for i32 {
    fn deserialize(json: &str) -> Result<Self, &'static str> {
        json.parse().map_err(|_| "Invalid integer")
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
        let (key, value) = split_key_value(part)?;
        map.insert(key, value);
    }

    Ok(map)
}

fn split_key_value(input: &str) -> Result<(String, String), &'static str> {
    // Remove the surrounding quotes from the input
    let trimmed = input.trim().trim_matches('"').trim();

    // Find the position of the first colon and ensure it's not at the beginning or end
    if let Some(colon_pos) = trimmed.find(':') {
        // Split at the first colon
        let key = trimmed[..colon_pos].trim().trim_matches('"').to_string();
        let value = trimmed[colon_pos + 1..]
            .trim()
            .trim_matches('"')
            .to_string();

        Ok((key, value))
    } else {
        Err("Colon not found")
    }
}

#[cfg(test)]
mod test {
    use crate::Deserialize;

    #[test]
    fn string_test() {
        let string = String::deserialize("\"hello\"");
        assert_eq!(string, Ok("hello".to_string()));
    }
}
