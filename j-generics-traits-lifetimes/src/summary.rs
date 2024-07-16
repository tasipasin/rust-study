// Traits

use std::fmt::{Display, Formatter, Result};

// To declare a trait, a keyword with the same name is used, followed by the trait's name
// The rest of the structure are the same as impl
// The difference is that the function may not be implemented at this point, only it's signature
// This forces each data type that has Summary trait to have it's own summarize() implementation
// If an implementation is made here, it will become the main implementation for the method and no
// longer will be required to be implemented
// This means that if some structure forgets to implement, the implementation here will be used
pub trait Summary {
    fn summarize(&self) -> String;

    fn never_implemented(&self) -> String {
        String::from("Not implemented")
    }
}

// A struct to a news article
#[derive(Debug)]
pub struct NewsArticle {
    headline: String,
    content: String,
    author: String,
}

impl NewsArticle {
    pub fn new(headline: &str, content: &str, author: &str) -> Self {
        NewsArticle {
            headline: String::from(headline),
            content: String::from(content),
            author: String::from(author),
        }
    }
}

// The implementation of the trait Summary for NewsArticle syntax is exaclty how it's written:
// the keyword impl, followed by the trait being implemented, the keyword for to indicate which
// struct will be for, and the struct name.
// The implementation must only contain the methods defined in the trait being implemented
impl Summary for NewsArticle {
    // The method signature is copied and the implementation is done to match the struct ending.
    fn summarize(&self) -> String {
        // format by default gets the value by reference, so the value is passed as reference
        format!("{}\n by {}\n{}", self.headline, self.author, self.content)
    }
}

impl Display for NewsArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.content)
    }
}

// Tweet

// Tweet can also take vantage of Summary
pub struct Tweet {
    username: String,
    date: i32,
    content: String,
}

impl Tweet {
    pub fn new(username: &str, date: i32, content: &str) -> Tweet {
        Tweet {
            username: String::from(username),
            date,
            content: String::from(content),
        }
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        // format by default gets the value by reference, so the value is passed as reference
        format!("@{} on {:?}: \n{}", self.username, self.date, self.content)
    }
}
