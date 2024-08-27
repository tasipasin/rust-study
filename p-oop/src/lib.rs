// --- gui
pub mod gui {
    // === Draw

    pub trait Draw {
        fn draw(&self);
    }

    // === Screen

    pub struct Screen {
        // Wherever a trait object is used, Rust's type system will ensure
        // at compile time that any value used in that context will implement
        // the trait. Thus, there's no need to know every possible types at compile time.
        // Differently of using a generic type parameter <T>, when only one concrete time
        // can be used at once
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // === Screen (Using generic type parameter - won't be used, just to diff both ways)

    pub struct ScreenGenT<T: Draw> {
        pub components: Vec<T>,
    }

    impl<T> ScreenGenT<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    // === Button

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }

    impl Button {
        // Specific functions for button
        pub fn on_click(&self) {
            // Some code to perform an action when the button is clicked
        }
    }

    impl Draw for Button {
        // Inside this block, only exists methods defined at Draw trait
        fn draw(&self) {
            // code to actually draw a button
            println!("Drawing a Button");
        }
    }
}

// --- Blog
pub mod blog {
    // === Post
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    // Implementation
    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(&mut self) {
            // Here's where the old state is consumed.
            // The take method consumes it and leaves a None value instead.
            // This moves the value out of Post.
            if let Some(e) = self.state.take() {
                // Will request to the state determinate the next state
                self.state = Some(e.request_review());
            }
            // Equivalent to:
            // match self.state.take(){
            //     Some(e) => self.state = Some(e.request_review()),
            // }
        }

        pub fn approve(&mut self) {
            if let Some(e) = self.state.take() {
                self.state = Some(e.approve());
            }
        }

        pub fn content(&self) -> &str {
            // as_ref() is used to not consume Option value
            // If it wasn't called, the compiler will cause an error because
            // state can't be moved outside borrowed &self
            // unwrap() can be called safelly because the implementation
            // makes sure that state is never None
            self.state.as_ref().unwrap().content(self)
        }

        pub fn state(&self) -> String {
            self.state.as_ref().unwrap().state_string()
        }
    }

    // === State
    // An Enum could be used to determinate the States, but in every method
    // a match call would be required to handle the variations. If more states
    // were required, it would turn to be big matches.
    // Defines the behavior shared by different post states
    trait State {
        // All states will implement this method
        // The 'self: Box<Self>' means that the method is only valid when
        // called on a Box holding the concrete type
        // It always consumes old state, invalidating it
        fn request_review(self: Box<Self>) -> Box<dyn State>;

        fn approve(self: Box<Self>) -> Box<dyn State>;

        // The lifetime annotation is required because the method is taking
        // a reference to a post as argument and returning part of it,
        // so the lifetime of the return is the same as the post parameter
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            // Default return is empty string slice
            // Because a default implementation was defined,
            // only the Structs that have different behavior must
            // redefine the method
            ""
        }

        fn state_string(&self) -> String;
    }

    // [Draft]
    #[derive(Debug)]
    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            PendingReview::new()
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            // A post can't be approved when in Draft
            self
        }

        fn state_string(&self) -> String {
            String::from("Draft")
        }
    }

    // [PendingReview]
    #[derive(Debug)]
    struct PendingReview {
        approvals: u8,
    }

    impl PendingReview {
        fn new() -> Box<dyn State> {
            Box::new(PendingReview { approvals: 0 })
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            // A post in PendingReview will stay in PendingReview
            self
        }

        fn approve(mut self: Box<Self>) -> Box<dyn State> {
            self.approvals += 1;
            if self.approvals >= 2 {
                // A post can't be approved when in Draft
                Box::new(Published {})
            } else {
                self
            }
        }

        fn state_string(&self) -> String {
            String::from("PendingReview")
        }
    }

    // [Published]
    #[derive(Debug)]
    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            // A post in Published can't change to PendingReview
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            // A post in Published is already Published
            self
        }

        fn content<'a>(&self, post: &'a Post) -> &'a str {
            &post.content
        }

        fn state_string(&self) -> String {
            String::from("Published")
        }
    }
}
