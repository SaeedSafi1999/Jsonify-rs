use serde_json::Value;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JsonifyValue {
    value: Value,
    value_type: String,
}

pub struct Jsonify {
    values: HashSet<(String, JsonifyValue)>,
}

impl Jsonify {
    pub fn new(json: &str) -> Self {
        let mut values: HashSet<(String, JsonifyValue)> = HashSet::new();
        Self::parse_json(json, String::new(), &mut values);
        Jsonify { values }
    }

    fn parse_json(json: &str, prefix: String, hashset: &mut HashSet<(String, JsonifyValue)>) {
        let json: Value = serde_json::from_str(json).unwrap_or(Value::Null);

        match json {
            Value::Object(map) => {
                for (key, value) in map {
                    let new_prefix = if prefix.is_empty() {
                        key
                    } else {
                        format!("{}.{}", prefix, key)
                    };
                    Self::parse_json(&value.to_string(), new_prefix, hashset);
                }
            }
            Value::Array(arr) => {
                for (index, value) in arr.iter().enumerate() {
                    let new_prefix = format!("{}[{}]", prefix, index);
                    Self::parse_json(&value.to_string(), new_prefix, hashset);
                }
            }
            _ => {
                let value_type = match &json {
                    Value::String(_) => "String",
                    Value::Number(_) => "Number",
                    Value::Bool(_) => "Bool",
                    Value::Null => "Null",
                    _ => "Unknown",
                }
                .to_string();

                let json_value = JsonifyValue {
                    value: json.clone(),
                    value_type,
                };
                hashset.insert((prefix, json_value));
            }
        }
    }

    pub fn get_value(&self, key: &str) -> Option<Value> {
        self.values
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.value.clone())
    }

    pub fn replace(&mut self, key: &str, new_value: Value) -> bool {
        if let Some((existing_key, existing_value)) =
            self.values.iter().find(|(k, _)| k == key).cloned()
        {
            self.values.remove(&(existing_key, existing_value));
            let new_value_entry = JsonifyValue {
                value: new_value.clone(),
                value_type: match &new_value {
                    Value::String(_) => "String".to_string(),
                    Value::Number(_) => "Number".to_string(),
                    Value::Bool(_) => "Bool".to_string(),
                    Value::Null => "Null".to_string(),
                    _ => "Unknown".to_string(),
                },
            };
            self.values.insert((key.to_string(), new_value_entry));
            true
        } else {
            false
        }
    }

    pub fn to_json(&self) -> String {
        let mut json_map: serde_json::Map<String, Value> = serde_json::Map::new();

        for (key, value) in &self.values {
            json_map.insert(key.clone(), value.value.clone());
        }

        serde_json::to_string(&json_map).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn add_to_json(&mut self, key: &str, value: Value) {
        let new_value = JsonifyValue {
            value: value.clone(),
            value_type: match &value {
                Value::String(_) => "String".to_string(),
                Value::Number(_) => "Number".to_string(),
                Value::Bool(_) => "Bool".to_string(),
                Value::Null => "Null".to_string(),
                _ => "Unknown".to_string(),
            },
        };
        self.values.insert((key.to_string(), new_value));
    }

    pub fn remove_from_json(&mut self, key: &str) -> bool {
        if let Some(entry) = self.values.iter().find(|(k, _)| k == key).cloned() {
            self.values.remove(&entry);
            true
        } else {
            false
        }
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.values.iter().any(|(k, _)| k == key)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.values.iter().map(|(k, _)| k.clone()).collect()
    }

    pub fn merge_json(&mut self, other_json: &str) {
        let mut new_values: HashSet<(String, JsonifyValue)> = HashSet::new();
        Self::parse_json(other_json, String::new(), &mut new_values);
        for (key, value) in new_values {
            self.values.insert((key, value));
        }
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

    let mut hashset = Jsonify::new(json_string);

    println!("Initial JSON: {}", hashset.to_json());

    hashset.remove_from_json("name");
    println!("After removing 'name': {}", hashset.to_json());

    hashset.add_to_json("country", Value::String("USA".to_string()));
    println!("After adding 'country': {}", hashset.to_json());

    println!("Does key 'age' exist? {}", hashset.has_key("age"));

    println!("All keys: {:?}", hashset.get_keys());

    hashset.merge_json(r#"{"state": "NY", "city": "Albany"}"#);
    println!("After merging JSON: {}", hashset.to_json());

   
}
