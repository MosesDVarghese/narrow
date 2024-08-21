pub struct ProblemInfo {
    pub number: u16,
    pub name: &'static str,
    pub variant: &'static str,
}

impl ProblemInfo {
    pub fn new(number: u16, name: &'static str, variant: &'static str) -> Self {
        Self {
            number,
            name,
            variant,
        }
    }

    pub fn display(&self) {
        println!("\n\n\n");
        println!("<===================================>");
        println!("Problem #{}", self.number);
        println!("Name: {}", self.name);
        println!("Difficulty: {}", self.variant);
        println!("<===================================>");
    }
}
