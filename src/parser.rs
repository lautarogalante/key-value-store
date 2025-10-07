
use crate::command::Command;

pub fn parse_args(s: &str) -> Vec<String> {
    if s.contains('"'){
        let args = s
            .replace('"', "").trim_end()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        return args;
    }
    return s.trim_end().split(' ').map(|s| s.to_string()).collect();
}

pub fn parse_key_value(s: Vec<String>) -> (String, String, String) { 
    return (s[0].clone(), s[1].clone(), s[2].clone());
}

pub fn parse_key(s: Vec<String>) -> (String, String){
    return (s[0].clone(), s[1].clone());
}

pub fn parse(command: &str) -> Result<Command, String> {
    let args: Vec<String> = parse_args(command);
    let (command, key) = parse_key(args.clone());
    
    match command.to_lowercase().as_str() {
        "get" =>  { 
            return Ok(Command::Get(key))
        }
        "set" => { 
            let (_, key, value) = parse_key_value(args);
            return Ok(Command::Set(key, value))
        }
        "del" => {
            return Ok(Command::Delete(key))
        }
        _ => return Err(String::from("Invalid argument"))
    } 

}
