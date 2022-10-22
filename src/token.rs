#![allow(non_snake_case)]

#[derive(Debug)]
pub struct Token {
    pub value: i32,
    pub ascii: Vec<String>,
}

impl Token {
    pub fn new(value: i32) -> Token {
        Self {
            value,
            ascii: tokenTemplate(value),
        }
    }
}

fn tokenTemplate(value: i32) -> Vec<String> {
    let mut template: Vec<String> = Vec::new();
    template.push("  /~~~~~~\\".to_string());
    template.push(" /        \\".to_string());

    if value > 9 {
        template.push(format!("|    {}    |", value).to_string());
    } else {
        template.push(format!("|    {}     |", value).to_string());
    }

    template.push(" \\        /".to_string());
    template.push("  \\~~~~~~/".to_string());

    return template;
}
