#[derive(Debug)]
// This defines this structure inside a submodule as public, so anywhere
// submodule's included, will be allowed to access the struct
pub struct Newmodule {
    pub name: String,
    year: u16,
}

// Not all implementations must be public, then which must, are individually
// marked with the keyword pub
impl Newmodule {
    pub fn init(value: &str, year: u16) -> Self {
        Newmodule {
            name: String::from(value),
            year,
        }
    }

    pub fn get_age(&self) -> u16 {
        2024 - self.get_year()
    }

    fn get_year(&self) -> u16 {
        self.year
    }
}
