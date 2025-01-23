use serde_json::Value;
use std::collections::HashSet;

pub struct Jsonify {
    values: HashSet<(String, JsonifyValue)>,
}

pub struct JsonifyValue{
    value:dyn,
    value_type:String,
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
            Value::String(s) => {
                let json_value = JsonifyValue {
                    value: Box::new(s),
                    value_type: "String".to_string(),
                };
                hashset.insert((prefix, json_value));
            }
            Value::Number(n) => {
                let json_value = JsonifyValue {
                    value: Box::new(n),
                    value_type: "Number".to_string(),
                };
                hashset.insert((prefix, json_value));
            }
            Value::Bool(b) => {
                let json_value = JsonifyValue {
                    value: Box::new(b),
                    value_type: "Bool".to_string(),
                };
                hashset.insert((prefix, json_value));
            }
            Value::Null => {
                let json_value = JsonifyValue {
                    value: Box::new(()),  
                    value_type: "Null".to_string(),
                };
                hashset.insert((prefix, json_value));
            }
        }
    }


    pub fn get_value<T>(&self, key: &str) -> Option<String> {
        let value = self.values
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone());
    }

    pub fn replace(&mut self, key: &str, new_value: &str) -> bool {
        if let Some((existing_key, existing_value)) = self.values.iter().find(|(k, _)| k == key).cloned() {
            self.values.remove(&(existing_key, existing_value.to_string()));
            self.values.insert((key.to_string(), new_value.to_string()));
            true
        } else {
            false
        }
    }

    pub fn to_json(&self) -> String {
        let mut json_map: std::collections::HashMap<String, Value> = std::collections::HashMap::new();

        for (key, value) in &self.values {
            let parsed_value = self.parse_value(value.value);
            json_map.insert(key.to_string(), parsed_value);
        }

        let json_string = serde_json::to_string(&json_map).unwrap_or_else(|_| "{}".to_string());
        json_string
    }

    fn parse_value(&self, value: &str) -> Value {
        if let Ok(n) = value.parse::<f64>() {
            Value::Number(serde_json::Number::from_f64(n).unwrap())
        } else if let Ok(b) = value.parse::<bool>() {
            Value::Bool(b)
        } else if value == "null" {
            Value::Null
        } else {
            Value::String(value.to_string())
        }
    }


    pub fn try_get_value(&self, key: &str) -> bool {
        self.values.iter().any(|(k, _)| k == key)
    }


    pub fn add_to_json<T>(&mut self, key: &str, value: T) 
    where
        T: serde::Serialize,  
    {
        let value_str = serde_json::to_string(&value).unwrap_or_else(|_| "null".to_string());
        self.values.insert((key.to_string(), value_str));
    }

}

fn main() {
    let json_string = r#"{
        "name": "John Doe",
        "age": 30,
        "address": {
            "city": "New York",
            "zip": "10001",
            "coordinates": {
                "latitude": 40.7128,
                "longitude": -74.0060
            }
        },
        "phones": ["123-456-7890", "987-654-3210"]
    }"#;

    let mut hashset = Jsonify::new(json_string);

    if hashset.try_get_value("name") {
        println!("Found key");
    } else {
        println!("Key 'name' not found.");
    }

    if hashset.replace("name", "Saeed Safi") {
        println!("Key 'name' replaced successfully.");
    } else {
        println!("Failed to replace key 'name'.");
    }

    hashset.add_to_json("is_active", true);
    hashset.add_to_json("rating", 4.5);
    hashset.add_to_json("nickname", "Saeed");

    println!("JSON representation of HashSet: {}", hashset.to_json());
    println!();
    println!();
    println!();
    let test = Jsonify::new(hashset.to_json().as_str());
    let r = test.to_json();
    println!("this is again =>{}",r);
    let key2 = test.get_value("is_active");
    println!("{:?}",key2.unwrap());
}
