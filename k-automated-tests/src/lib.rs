#[derive(Debug)]
// Define as pub because the compiler was complaining about never be constructed
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    pub fn new(width: u32, height: u32) -> Rectangle {
        if width / height == 1 {
            panic!("It's a square");
        }
        Rectangle { width, height }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn hello(name: &str) -> String {
    // Name is not used
    format!("{name}");
    format!("Hello!")
}

// Each test is run in a new thread. If any thread fails, the main thread will
// see that and the tests will finish as failed
#[cfg(test)]
mod tests {
    // The tests module is a inner module, this line brings the code under test,
    // in the outer scope
    use super::*;

    // This annotation indicates this function is a test function
    // Not all the functions inside the test module must be a test function
    // Some might be a commom preps for every test and should be not annotated
    // by the #[test] annotation
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // Macro to indicates the result must be equals to an expected
        // value, or will panic
        assert_eq!(result, 4);
    }

    #[test]
    fn can_hold() {
        let rec1 = Rectangle {
            width: 5,
            height: 4,
        };
        let rec2 = Rectangle {
            width: 2,
            height: 1,
        };
        // The assert! macro expects to receive a true boolean value
        // If it receives a false value, it will panic and the test will fail
        assert!(rec1.can_hold(&rec2));
        // This assert! will fail
        // assert!(rec2.can_hold(&rec1));
    }

    #[test]
    fn cannot_hold() {
        let rec1 = Rectangle {
            width: 5,
            height: 4,
        };
        let rec2 = Rectangle {
            width: 2,
            height: 1,
        };
        assert!(!rec2.can_hold(&rec1));
    }

    #[test]
    fn greetings() {
        let expected = "Tasi";
        let result = hello(expected);
        // Assert can also work as a format!
        assert!(
            result.contains(expected),
            "Did not contain the expected. Value was {}",
            result
        );
    }

    #[test]
    // It means the test is expecting a panic to successfully pass
    // It does not specifie the panic reason, so if the test panics for any
    // reason, the test will pass and can cause a false positive
    #[should_panic]
    fn should_panic() {
        Rectangle::new(50, 50);
    }

    #[test]
    #[should_panic]
    fn not_panic() {
        // This won't cause a panic, which will cause a test failure
        Rectangle::new(50, 51);
    }

    #[test]
    #[should_panic]
    fn not_should_panic() {
        // Expecting this to panic
        Rectangle::new(50, 51);
        // But this will panic instead, so will cause a false positive
        panic!("I'm panicking");
    }

    #[test]
    // The expected value is the a substring from the panic that considered for this case
    // If the panic message doesn't have this substring, the panic will be disconsidered
    // and test will fail
    #[should_panic(expected = "square")]
    fn not_should_panic_pattern() {
        // Expecting this to panic
        Rectangle::new(50, 51);
        // But this will panic instead, so will cause a false positive
        panic!("I'm panicking");
    }

    #[test]
    // The expected value is the a substring from the panic that considered for this case
    // If the panic message doesn't have this substring, the panic will be disconsidered
    // and test will fail
    #[should_panic(expected = "square")]
    fn not_should_panic_pattern_pass() {
        // Expecting this to panic
        Rectangle::new(50, 50);
        // But this will panic instead, so will cause a false positive
        panic!("I'm panicking");
    }

    #[test]
    fn it_fails() {
        let result = add(2, 2);
        // This test is supposed to fail
        // When a test fails, it will display which tests failed and which
        // ones failed and why
        // assert_eq!(result, 5);
        // The macro assert_ne! let the test pass if the values passed as
        // parameters are different
        assert_ne!(result, 5);
        // When a test fails, a message can be defined to be shown after the main
        // parameters required for the macro
        // This text won't remove the details about what went wrong
        assert_eq!(result, 5, "Should display this text");
    }

    #[test]
    fn it_works_result() -> Result<(), String>{
        // Because Result return type is being used, there's no need to
        // user the asserts macros
        // This method allows to use the question mark ? for convenience
        // In the other hand, the should_panic annotation can't be used.
        if add(3,2) == 5 {
            Ok(())
        }else{
            Err(String::from("It's not 5"))
        }
    }
}
