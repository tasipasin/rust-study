// [Mock Objects]

pub trait Messenger {
    fn send(&self, msg: &str);
}

// LimitTracker struct has the same lifetime as the messenger it holds ('a)
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// LimitTracker implementation when T type is derivated of Messenger trait
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod mock_tests {
    use std::cell::{Ref, RefCell};

    use super::*;

    struct MockMessenger {
        // To store all Messenger::send messages
        // Defined as RefCell<T>
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        // Constructor
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }

        // To make transparent to ones using, Ref<T> can be returned
        // from a method, that is a smart pointer
        fn sent_messages(&self) -> Ref<Vec<String>> {
            // .borrow() returns a imutable reference of sent_messages
            self.sent_messages.borrow()
        }
    }

    // Implements Messenger trait methods
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self is still immutable, but can be changed because of the
            // use of RefCell<T>
            // .borrow_mut() returns smart pointer RefMut<T>
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // Messenger trait object
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages().len(), 1);
    }
}
