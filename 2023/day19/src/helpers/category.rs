pub enum Category {
    X,
    M,
    A,
    S,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            x => panic!("unknown category: {}", x),
        }
    }
}
