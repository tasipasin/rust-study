use std::vec;

use p_oop::{blog::Post, gui};

// === SelectBox
// User defined type

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl gui::Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing a SelectBox");
    }
}

fn main() {
    // Rust isn't a OOP language, but it is influenced by many of it's paradigms.

    // === Encapsulation ===
    // Rust has the pub keyword to allow external code to access some functions,
    // attributes and even a full struct of some structure/module

    // === Inheritance ===
    // Rust does not support inheritance directly.
    // In Rust, there's other solutions to that, like default implementations
    // of a trait.

    // === Polymorphism ===
    // Rust instead uses generics to abstract over different possible types and
    // trait bounds to impose constraints on what those types must provide

    // === Trait for Values of Different Types ===
    {
        println!("=== Trait for Values of Different Types ===");
        let select_box = SelectBox {
            width: 25,
            height: 5,
            options: vec![
                String::from("hi"),
                String::from("i'm"),
                String::from("optimus"),
                String::from("prime"),
            ],
        };
        let button = gui::Button {
            height: 10,
            width: 10,
            label: String::from("Do not click here"),
        };
        // Encapsulates the itens in Boxes for the screen struct accept them,
        // since they implemented the Draw trait required
        let screen = gui::Screen {
            components: vec![Box::new(select_box), Box::new(button)],
        };

        // Will run the screen, which will draw every element
        // Draw, here, is print something on the console
        screen.run();

        // This piece of code won't compile because String does not implement
        // the Draw trait, so Screen can't accept it
        // let screen = gui::Screen {components: vec![Box::new(String::from("Not running"))]};
    }

    println!();

    // State Pattern
    {
        println!("=== State Pattern ===");
        // It's a design pattern defined by a set of states. The value's behavior
        // changes based on its state.
        // Each state is responsible for its own behavior and to determinate when
        // it should change into another state.

        // Example rules:
        // 1. A post starts as an empty draft
        // 2. When the draft is done, a review is required
        // 3. When the review is approved, it gets published
        // 4. Only when published, the post can return content to print
        // 5. An unapproved review can't be published

        // State: Draft
        let mut post = Post::new();

        post.add_text("I really want to play 7 Days to Die");
        // Doesn't have content to print because it's not published yet
        assert_eq!("", post.content());
        println!("Post [{}]", post.state());

        // Won't do anything because the post is still in Draft
        post.approve();
        assert_eq!("", post.content());
        println!("Tried to Approve: Post [{}]", post.state());

        // State: Pendente de Revisão
        post.request_review();
        // Doesn't have content to print because it's not published yet
        assert_eq!("", post.content());
        println!("Review Requested: Post [{}]", post.state());

        // State: posted
        // Require 2 approvals
        post.approve();
        println!("After 1 approval: Post [{}]", post.state());
        post.approve();
        println!("After 2 approvals: Post [{}]", post.state());
        // Now that's approved, the content must be able to print
        assert_eq!("I really want to play 7 Days to Die", post.content());
        println!("Post content: {}", post.content());
    }
}
