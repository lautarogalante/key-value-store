use std::{collections::HashMap};
use crate::command::Command;

pub struct Store {
    value: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
           value: HashMap::new(),
        }
    }
    
    pub fn execute(&mut self, s: &str) -> Result<String, String> {
        
        match s.parse()? {
            Command::Get(key) => { 
                self.value.get(&key).map(|v| v.to_owned()).ok_or_else(|| format!("The value with the key: {} was deleted", key))
            }
            Command::Set(key,value ) => { 
                self.value.insert(key, value);
                Ok(String::from("Inserted value\n"))
            }
            Command::Delete(key) => { 
                let value = self.value.remove(&key).ok_or_else(|| format!("Error to remove value with key: {} ", key))?;
                Ok(format!("Value: {} with the Key: {} has been deleted", value, key))
            } 
        }

    }
}
