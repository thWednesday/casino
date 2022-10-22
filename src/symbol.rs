pub struct Symbol {
    pub name: String,
    pub ascii: String,
}

impl Symbol {
    pub fn new(name: &str) -> Self {
        let ascii: String = match name {
            "spade" => "♠".to_string(),
            "heart" => "♥".to_string(),
            "club" => "♣".to_string(),
            "diamond" => "♦".to_string(),
            &_ => panic!("Not a suit"),
        }
        .to_string();

        let name = name.to_string();

        Self { name, ascii }
    }
}
