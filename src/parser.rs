use crate::command::Command;

pub fn parse(command: &str) -> Result<Command, String> {
    let parts: Vec<&str> = if command.contains('"') {
        command
            .split('"')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        command.split_whitespace().collect()
    };

    if parts.is_empty() {
        return Err(String::from("empty command"))
    }

    match parts[0].to_lowercase().as_str() {
        "get" => { 
            
            if parts.len() != 2 {
               return Err(String::from("get command requires, one argument")) 
            }    
            
            Ok(Command::Get(parts[1].to_string())) 
        }
        "set" => { 
            if parts.len() != 3 {
                return Err(String::from("set command requires two arguments"))
            }
            Ok(Command::Set(parts[1].to_string(), parts[2].to_string()))
        }
        "del" => { 
            if parts.len() != 2 {
                return Err(String::from("del command require one argument"))
            }
            Ok(Command::Delete(parts[1].to_string()))
        }
        _ => Err(String::from("command not found"))
    }
}
