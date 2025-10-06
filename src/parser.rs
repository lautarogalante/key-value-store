
use crate::command::Command;

pub fn parse_key(s: &str) -> Vec<&str> {
    let key: Vec<&str> = s.split_whitespace().collect();
    // key.remove(0);
    return key;
}

pub fn parse_key_value(_s: &str) { 

}

pub fn parse_args() {
}

pub fn parse(command: &str) -> Result<Command, String> {
    println!("{:?}", parse_key(command));


    return Ok(Command::Get(command.to_string()));
}
