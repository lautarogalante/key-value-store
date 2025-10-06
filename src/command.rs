use std::str::FromStr;
use crate::parser::parse;
pub enum Command {
    Get(String),
    Set(String, String),
    Delete(String)
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }

}