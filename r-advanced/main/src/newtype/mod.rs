use std::fmt;

// Newtype Wrapper tuple struct holds a vector of String.
pub struct Wrapper(pub Vec<String>);

// The newtype allows to change implementation of insides.
// On the other side, the Newtype Wrapper doesn't have all the implementations
// from Vec<>.
// To solve this, the Newtype should implement the Deref trait.
// The newtype also allow the programmer to limit some of the inner value
// ops.
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(" -- "))
    }
}
