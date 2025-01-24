use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonifyValue {
    value: Value,
    value_type: String,
}

pub struct Jsonify {
    values: HashMap<String, JsonifyValue>,
}

impl Jsonify {
    pub fn new(json: &str) -> Result<Self, String> {
        let mut values: HashMap<String, JsonifyValue> = HashMap::new();
        let parsed_value: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        Self::parse_json(parsed_value, String::new(), &mut values);
        Ok(Jsonify { values })
    }

    fn parse_json(value: Value, prefix: String, map: &mut HashMap<String, JsonifyValue>) {
        match value {
            Value::Object(map_values) => {
                for (key, sub_value) in map_values {
                    let new_prefix = if prefix.is_empty() {
                        key
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    Self::parse_json(sub_value, new_prefix, map);
                }
            }
            Value::Array(array_values) => {
                for (index, sub_value) in array_values.into_iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, index);
                    Self::parse_json(sub_value, new_prefix, map);
                }
            }
            _ => {
                let value_type = match &value {
                    Value::String(_) => "String",
                    Value::Number(_) => "Number",
                    Value::Bool(_) => "Bool",
                    Value::Null => "Null",
                    _ => "Unsupported",
                }
                .to_string();

                map.insert(
                    prefix,
                    JsonifyValue {
                        value: value.clone(),
                        value_type,
                    },
                );
            }
        }
    }

    pub fn to_flat_json(&self) -> String {
        let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new();
        for (key, value) in &self.values {
            json_map.insert(key.clone(), value.value.clone());
        }
        serde_json::to_string(&json_map).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn to_nested_json(&self) -> String {
        let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new();
        for (key, value) in &self.values {
            json_map.insert(key.clone(), value.value.clone());
        }
        serde_json::to_string(&json_map).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn add_to_json(&mut self, key: &str, value: Value) {
        let value_type = match &value {
            Value::String(_) => "String".to_string(),
            Value::Number(_) => "Number".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Null => "Null".to_string(),
            _ => "Unsupported".to_string(),
        };
        self.values.insert(
            key.to_string(),
            JsonifyValue {
                value: value.clone(),
                value_type,
            },
        );
    }

    pub fn remove_from_json(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }

    pub fn merge_json(&mut self, other_json: &str) -> Result<(), String> {
        let parsed_value: Value = serde_json::from_str(other_json).map_err(|e| e.to_string())?;
        Self::parse_json(parsed_value, String::new(), &mut self.values);
        Ok(())
    }
}

fn main() {
    let json_string = r#"{
        "name": "John Doe",
        "age": 30,
        "address": {
            "city": "New York",
            "zip": "10001"
        }
    }"#;

    let mut hashset = Jsonify::new(json_string).expect("Valid JSON");

    println!("Initial JSON: {}", hashset.to_flat_json());
    println!();

    hashset.remove_from_json("name");
    println!("After removing 'name': {}", hashset.to_flat_json());
    println!();

    hashset.add_to_json("address.country", Value::String("USA".to_string()));
    println!("After adding 'country': {}", hashset.to_flat_json());
    println!();

    let v = json!({ "token": "abcdef123456" });
    hashset.add_to_json("auth", v);
    println!("After adding 'auth': {}", hashset.to_flat_json());
    println!();

    let l = json!(["1", "2", "3", "4"]);
    hashset.add_to_json("list", l);
    println!("After adding 'list': {}", hashset.to_flat_json());
    println!();

    println!("Does key 'age' exist? {}", hashset.has_key("age"));
    println!();

    println!("All keys: {:?}", hashset.get_keys());
    println!();

    hashset
        .merge_json(r#"{"IsMarried": false, "HasChild": true}"#)
        .expect("Valid merge JSON");
    println!("After merging JSON: {}", hashset.to_flat_json());
}
