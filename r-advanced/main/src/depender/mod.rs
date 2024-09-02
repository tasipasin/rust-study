use core::fmt;

// This syntax indicates that PrintBox will only work for types
// that also implements Display and provides the functionality
// PrintBox requires
pub trait PrintBox: fmt::Display {
    fn outline_print(&self) {
        // The to_string() is automatically implemented for
        // any type that implements Display
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

pub struct Item {
    name: String,
    rarity: String,
}

impl Item {
    pub fn new(name: &str, rarity: &str) -> Self {
        Item {
            name: String::from(name),
            rarity: String::from(rarity),
        }
    }
}

impl PrintBox for Item {}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.rarity)
    }
}
