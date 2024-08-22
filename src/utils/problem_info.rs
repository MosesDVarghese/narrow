pub struct ProblemInfo {
    pub number: u16,
    pub name: &'static str,
    pub variant: &'static str,
    pub takeaway: &'static str,
}

impl ProblemInfo {
    pub fn new(number: u16, name: &'static str, variant: &'static str, takeaway: &'static str) -> Self {
        Self {
            number,
            name,
            variant,
            takeaway,
        }
    }

    pub fn display(&self) {
        println!("\n<===================================>");
        println!("Problem #{}", self.number);
        println!("Name: {}", self.name);
        println!("Difficulty: {}", self.variant);
        println!("<===================================>");
    }

    pub fn end(&self) {
        println!("\nTake away: {}", self.takeaway)
    }
}
