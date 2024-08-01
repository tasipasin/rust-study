use std::cell::RefCell;
use std::rc::Weak;
use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RefCellList::{RefCellCons, RefCellNil};

// A implementation of cons list
// Uses Box<> because the compiler requires to known how much space
// it needs for a variable. Box is a pointer, so Rust always knows it's size.
// Box will point to the next List item and the value will be stored in the heap.
// Now the compiler known how much space List is required
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// A similar implementation of Box<T>
// It declares a generic parameter so it can hold any value type
// And is a tuple with only one value
struct MyBox<T>(T);

impl<T> MyBox<T> {
    // The new method to represent the Box::new()
    // Receives a value of any data type and returns an instance of itself
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // Defines the type used for the entire Deref to use
    type Target = T;

    // Receives itself borrowed and returns the reference to the data inside self
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct CustomPointer {
    data: String,
}

impl Drop for CustomPointer {
    // This defines what happens with a CustomPointer when it gets
    // out of scope
    fn drop(&mut self) {
        println!("Dropping Custom Pointer: {}", self.data);
    }
}

// An implementation of cons list using Rc instead of Box
#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

// An implementation of cons list using Rc instead of Box
#[derive(Debug)]
enum RefCellList {
    RefCellCons(Rc<RefCell<i32>>, Rc<RefCellList>),
    RefCellNil,
}

#[derive(Debug)]
struct Node {
    value: i32,
    // RefCell allows to change which node is child of another node
    // Rc allows to share ownership (immutable) to access the children directly
    children: RefCell<Vec<Rc<Node>>>,
    // RefCell allows to change which node is child of another node
    // Weak is a weak reference to the node evaluated at runtime, returning Option
    parent: RefCell<Weak<Node>>
}

fn main() {
    // Smart pointer have metadata and capabilities along with a reference
    // to a address in memory.
    {
        println!("=== Box<T> ===");
        // Box stores data in the heap instead in the stack
        // In the stack only stays the pointer to the Box data in heap
        // Boxes are used in:
        // 1. When a data type size can't be defined at compile time
        // 2. Transfer ownership of a large amount of data without copying
        // 3. Assuring the data using is an implementation of a certain Trait

        // Syntax to encapsulate the data inside the box
        // The data will stored in the heap while the reference will be in the stack
        // When it gets out of scope, both Box and value are deallocated
        let boqs = Box::new(5);
        println!("boqs: {boqs}");

        println!("[Cons list]");
        // Cons list works like a linked list. It contains a pair of values.
        // Each pair contains the value of the current item and the next item, which is another pair
        // When it comes to the last pair, the second value is the Nil, indicating the end
        let conslist = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", conslist);
        if let List::Cons(val1, val2) = conslist {
            println!("{:?}, {:?}", val1, val2);
        };
    }

    println!();

    // Deref Trait
    {
        println!("=== Deref Trait ===");
        let x = 5;
        // y holds the address of x
        let y = &x;
        println!("y: {y}");

        assert_eq!(5, x);
        // This line don't work because y is pointing to the address of x
        // assert_ne!(5, y);
        // y need to be dereferenced so the compile can use the value
        // that it points to and can compare values
        assert_eq!(5, *y);

        println!("[Using Box<T> like a Reference]");

        let mut x = 5;
        // The main difference here to the next operation is that y
        // is now a copy from x and not borrowed
        let y = Box::new(x);
        x = 6;
        assert_eq!(6, x);
        assert_eq!(5, *y);

        println!("[Defining a Smart Pointer]");

        let mut x = 5;
        let y = MyBox::new(x);
        x = 6;
        assert_eq!(6, x);
        // Before MyBox implemented the Deref trait this line won't work
        // The Deref trait makes Rust run the code: *(y.deref())
        assert_eq!(5, *y);

        println!("[Implicit Deref Coercions]");
        // Deref coersion works only in types that implements Deref trait
        hello("Tasi");
        // Rust will derref until gets to &str, which fits the required parameter
        hello(&MyBox::new(String::from("MyBox")));
    }

    println!();

    // Drop
    {
        println!("=== Drop ===");
        // Drop trait allows customize what happens when a value is going out of scope.
        // The Drop trait is automatically called by Rust.

        // A new scope to force Rust calling the Drop trait
        {
            let cp = CustomPointer {
                data: String::from("A value to be dropped"),
            };
            println!("Created CustomPointer: {:?}", cp);
        } // Here cp will go out of scope, so drop() should be called
        println!("What's cp?");
    }

    println!();

    // Rc<T>
    {
        println!("=== Rc<T> ===");
        // Rc, of Reference Counting, allows to have multiple ownsership.
        // It keeps track of references to a value.
        // It should be used to allocate data on the heap for multiple parts
        // of the program to read.
        // This should be used in single-thread scenarios only

        let rclist_shared = Rc::new(RcCons(1, Rc::new(RcCons(2, Rc::new(RcNil)))));

        // Rc::strong_count reveals the reference count
        println!(
            "Counting after creating rclist_shared: {}",
            Rc::strong_count(&rclist_shared)
        );
        // Clone call only increments the reference count
        let rclist_1 = RcCons(100, Rc::clone(&rclist_shared));
        println!(
            "Counting after creating rclist_1: {}",
            Rc::strong_count(&rclist_shared)
        );
        let rclist_2 = RcCons(200, Rc::clone(&rclist_shared));
        println!(
            "Counting after creating rclist_2: {}",
            Rc::strong_count(&rclist_shared)
        );
        println!("rclist_1: {:?}", rclist_1);
        println!("rclist_2: {:?}", rclist_2);
        // Creates a new scope
        {
            let rclist_3 = RcCons(300, Rc::clone(&rclist_shared));
            println!("rclist_3: {:?}", rclist_3);
            println!(
                "Counting after creating rclist_3: {}",
                Rc::strong_count(&rclist_shared)
            )
        }
        println!(
            "Counting after rclist_3 went out of scope: {}",
            Rc::strong_count(&rclist_shared)
        );
        if let RcList::RcCons(val1, val2) = rclist_2 {
            println!("{:?}, {:?}", val1, val2);
        };
    }

    println!();

    // RefCell<T>
    {
        println!("=== RefCell<T> ===");
        // RefCell<T> represents single ownership, unlike Rc<T>
        // Unlike Box<T>, the borrowing rules of RefCell<T> are enforced at runtime.
        // It means that, if any of the borrowing rules are broken, the program
        // will panic and exit.
        // So, RefCell<T> is usefull when the compile can't guarantee if the borrowing
        // rules are broken, but the developer can

        // Rc<T> enables multiple owner of the same data, while Box<T> and RefCell<T> doesn't

        // Box<T> allow immutable and mutable borrows checked at compile time
        // Rc<T> allows only immutable borrow checked at compile time
        // RefCell<T> are a Box<T>, but at runtime

        // An immutable data can suffer mutation with RefCell<T> at runtime

        // This piece of code doesn't work because borrow rules
        // let imut = 10;
        // let nmut = &mut imut;

        // RefCell<T> allows the value to have interior mutability

        // RefCell<T>, like Rc, keeps track of how many Ref<T> or RefMut<T> are
        // active distinguishing the two types
        // The borrowing rule that must be only one mutable borrow in the same scope
        // is still guaranteed by RefCell<T>. The code will compile if this rule
        // gets violated, but will panic at runtime.

        println!();
        println!("[RefCell and Rc combo]");
        // Using Rc<RefCell<T>> allows to have multiple owner that can mutate data

        // Visible value: 5
        let refcell_shared = Rc::new(RefCell::new(5));
        println!("refcell_shared: {:?}", refcell_shared);

        // Visible value: (5, RefCellNil)
        let refcell_a = Rc::new(RefCellCons(Rc::clone(&refcell_shared), Rc::new(RefCellNil)));
        println!("refcell_a: {:?}", refcell_a);

        // Visible value: (100, (5, RefCellNil))
        let refcell_b = RefCellCons(Rc::new(RefCell::new(100)), Rc::clone(&refcell_a));
        println!("refcell_b: {:?}", refcell_b);

        // Visible value: (200, (5, RefCellNil))
        let refcell_c = RefCellCons(Rc::new(RefCell::new(200)), Rc::clone(&refcell_a));
        println!("refcell_c: {:?}", refcell_c);

        // Change the shared value
        *refcell_shared.borrow_mut() += 55;

        println!("~After borrowed refcell_shared and added 55~");
        println!("refcell_shared: {:?}", refcell_shared);
        println!("refcell_a: {:?}", refcell_a);
        println!("refcell_b: {:?}", refcell_b);
        println!("refcell_c: {:?}", refcell_c);

        println!("Remove unnused");
        if let RefCellList::RefCellCons(val1, val2) = refcell_c {
            println!("{:?}, {:?}", val1, val2);
        };

        println!();
        println!("[Weak]");

        // Create a leaf over a Rc to allow multiple immutable borrowing
        // And it's chidren over a RefCell, to allow mutable borrowing
        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new())
        });

        // .borrow() returns an immutable reference (RefCell)
        // .upgrade() evaluate if there's a value and returns an Optional
        println!("leaf parent before: {:?}", leaf.parent.borrow().upgrade());

        // Create a root over a Rc to allow multiple immutable borrowing
        // And it's chidren over a RefCell, to allow mutable borrowing
        // Now the value in leaf has two owners
        let root = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        
        println!("root.value: {}", root.value);
        println!("root.parent: {:?}", root.parent);
        println!("root.children: {:?}", root.children);
        // .downgrade() creates a Weak<T> reference to root
        *leaf.parent.borrow_mut() = Rc::downgrade(&root);
        println!("leaf parent after: {:?}", leaf.parent.borrow().upgrade());
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
