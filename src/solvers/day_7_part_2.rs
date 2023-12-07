use std::str::FromStr;

struct Object {}
impl Object {}

struct StringParseError;
impl FromStr for Object {
    type Err = StringParseError;
    fn from_str(_data: &str) -> Result<Self, Self::Err> {
        return Ok(Object {});
    }
}

pub fn solve(_data: &String) -> String {
    return "sum".to_string();
}
