use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    file_path: String,
    query: String,
}

impl Config {
    // Function name is build because when a programmer uses the new function
    // they expects the function to never fail
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // The zero argument is the executable file
        args.next();
        // the first argument is the file path
        let file_path = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a file path"),
        };
        // the rest are string to be found
        let query = match args.next() {
            Some(value) => value,
            None => return Err("Didn't get a query"),
        };
        Ok(Config { file_path, query })
    }
}

// Box<dyn Error> means an implementation of Error
// dyn is a short for dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&content, &config.query) {
        println!("{line}");
    }
    Ok(())
}

fn search<'a>(content: &'a str, query: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "tasi";
        let content = "some text here\nmore tasi there\nand some writting here";
        assert_eq!(vec!["more tasi there"], search(content, query));
    }
}
