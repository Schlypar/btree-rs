use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Fixed {
    str: [char; 25],
}

impl Fixed {
    pub fn new(str: [char; 25]) -> Self {
        Self { str }
    }

    pub fn from(str: &str) -> Self {
        let mut result: [char; 25] = ['\0'; 25];
        for (i, character) in str.chars().enumerate().take(std::cmp::min(str.len(), 25)) {
            result[i] = character;
        }
        Self { str: result }
    }
}

impl std::convert::From<String> for Fixed {
    fn from(value: String) -> Self {
        let mut result: [char; 25] = ['\0'; 25];
        for (i, character) in value
            .chars()
            .enumerate()
            .take(std::cmp::min(value.len(), 25))
        {
            result[i] = character;
        }
        Self { str: result }
    }
}

impl std::convert::From<&str> for Fixed {
    fn from(value: &str) -> Self {
        let mut result: [char; 25] = ['\0'; 25];
        for (i, character) in value
            .chars()
            .enumerate()
            .take(std::cmp::min(value.len(), 25))
        {
            result[i] = character;
        }
        Self { str: result }
    }
}
