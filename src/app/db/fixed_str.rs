use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Fixed {
    str: [char; 25],
}

#[allow(dead_code)]
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

impl std::fmt::Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self
            .str
            .iter()
            .cloned()
            .position(|ch| ch as u8 == b'\0')
            .unwrap_or(25);

        let mut string = String::with_capacity(len);

        for i in 0..len {
            string.push(self.str[i]);
        }
        write!(f, "{}", string)
    }
}
