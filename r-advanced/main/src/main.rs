use depender::{Item, PrintBox};
use disambiguation::{Bird, Object, Pepperoni, Pizza, Plane};
use hello_macro::HelloMacro;
use macros::Pancakes;
use newtype::Wrapper;
use overrider::Point;

pub mod depender;
pub mod disambiguation;
pub mod newtype;
pub mod overrider;
pub mod macros;
static mut COUNTER: u32 = 0;

fn main() {
    // Unsafe Rust
    {
        println!("=== Unsafe Rust ===");
        // Unsafe Rust is Rust, but with root powers!
        // Sometimes, unsafeness are required to do certain tasks.
        // To write unsafe Rust codes, the keyword unsafe is used on the start
        // of a new block of code.

        // In unsafe Rust, theres five unsafe Rust superpowers:
        // 1. Deference a raw pointer
        // 2. Call an unsafe function or method
        // 3. Access of modify a mutable static variable
        // 4. Implement an unsafe trait
        // 5. Access field of a union

        // Even on a unsafe Rust block, the borrow checker is not turned off
        // and all the Rust's safety checks still works. The unsafe keyword only
        // gives access to theses five superpowers and make the compiler doesn't
        // check for memory safety.

        // An unsafe Rust code block is not necessarily dangerous, it only tell
        // that the programmer knows something the compiler doesn't.

        println!("[Deference a raw pointer]");
        // Raw pointers:
        // 1. Are allowed to ignore borrowing rules by having both immutable and mutable
        //    pointers os multiple mutable pointers to the same location
        // 2. Aren't guaranteed to point to valid memory
        // 3. Are allowed to be null
        // 4. Don't implement any automatic cleanup
        let mut num = 5;

        // Raw pointers can be created outside an unsafe block, but can't be dereferenced

        // Raw pointer created using as to cast an immutable and mutable reference into
        // the corresponding raw pointer type.
        // For this, the * is not the dereferencing operator, but part of the data type
        // Created from a valid value and pointed to the same memory address
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // Creates a raw pointer to an arbitraty location in memory. Not sure, though, if
        // it is valid. (Commented because the result is uncertain)
        // let from_address = 0x1234usize;
        // let raw_from_address = from_address as *const i32;

        unsafe {
            println!("Inside unsafe :)");
            println!("r1: {}", *r1);
            println!("r2: {}", *r2);
        }

        // Main use for raw pointer: interface with C code!

        println!("[Call an unsafe function or method]");
        // Unsafe methods/functions are the same as safe ones, but have the unsafe keyword
        // before the definition.
        // The way to call an unsafe function is the same as dereferencing raw pointers
        unsafe {
            do_not_trespass();
        }

        // ------------------------------------------
        println!("Safe Abstraction over Unsafe Code");
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        // split_as_mut() is an example from the standard library; it requires some unsafe code
        // It only works for slices of i32
        // Rust' borrow checker will acuse two mutable borrowing, but it can't understands the
        // slices aren't overlapping.
        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);

        // ------------------------------------------
        println!("extern");
        // extern facilitates the creation and use of a Foreign Function Interface (FFI).
        // With FFI, different programming languages can call these functions
        let negative = -50;
        // It's unsafe because Rust doesn't know what happens there
        unsafe {
            println!("Calling C abs() for {}: {}", negative, abs(-50));
        }

        // ------------------------------------------
        println!("[Access and Modify Mutable Static Variable]");
        // Rust allows global variables, but can be problematic because of the ownership rules
        // In Rust, global variables are called Static Variables
        // Although constants and Static Variables are similar, values in a static variable
        // have a fixed memory address. This means that every acess will acess the same data.
        // Constants can duplicate it's data whenever used.
        // Static Variable can be mutable, but is unsafe.
        // Both reading and changing a static variable must be done inside a unsafe block.
        unsafe {
            println!("COUNTER before: {}", COUNTER);
        }
        increment_counter(22);
        unsafe {
            println!("COUNTER after: {}", COUNTER);
        }

        // [Unsafe Traits]
        // A whole trait is unsafe when at least one of its methods is unsafe
        // Like every other uses, the unsafe keyword is put before the trait and implement definition

        // [Accessing fields from Union]
        // A union is similar to a struct, but one declared field is used in a particular instance
        // at a time.
    }

    println!();

    // Advanced Traits
    {
        println!("=== Advanced Traits ===");
        println!("[Associated Types]");
        // Associated types connect a type placeholder with a trait such that
        // a trait method definitions can these placeholders types in their
        // signature. The trait implementor specify the concrete type of the trait.
        // This way, the trait definition is based in some basic generic datatype.

        // The Iterator is an example with an associated type.
        // The associated type is named Item and it is the type Iterator is iterating
        // over.
        // The type Item is a placeholder and Iterator methods use them like a generic.
        // However, generics and Associated Types are different.
        // Using generics, the type annotation must be used in each implementation.
        // Using associated types, the annotation is not required. The type of Item
        // will be only one at once.

        println!("[Operator Overloading]");
        let p1 = Point { x: 1, y: 4 };
        let p2 = Point { x: 2, y: 5 };
        // Make use of the + operator overloaded
        let p12 = p1 + p2;
        println!("p1: {:?}", p1);
        println!("p2: {:?}", p2);
        println!("p12: {:?}", p12);
        let p2i32 = p2 + 32;
        println!("p2i32: {:?}", p2i32);

        println!("[Disambiguation]");
        // When calling methods with the same name, should be specified to Rust which one
        // it's being required

        let obj = Object;
        print!("obj fly() Object: ");
        // The compiler will call the direct implementation of fly() on Object
        obj.fly();
        // Expliciting the fly trait method
        print!("Plane::fly(&obj): ");
        Plane::fly(&obj);
        print!("Bird::fly(&obj): ");
        Bird::fly(&obj);

        println!("Pepperoni::pizza_name(): {}", Pepperoni::pizza_name());
        // Because the pizza_name() doesn't have the self parameter, it can't decide
        // which implementation to call. Using the fully qualified syntax will help
        // the compiler understand which pizza_name() implementation is required.
        // This means to call Pizza implementation for Pepperoni struct.
        println!(
            "Pizza::pizza_name(): {}",
            <Pepperoni as Pizza>::pizza_name()
        );

        println!("[Trait requires Trait]");
        let item = Item::new("Sword", "Rare");
        PrintBox::outline_print(&item);

        println!("[Newtype Pattern]");
        // The newtype patterns allows implement a trait on a type even if both are
        // outside the crate.
        let wrapper = Wrapper(vec![
            String::from("ho"),
            String::from("ho"),
            String::from("santa's"),
            String::from("comming"),
        ]);
        println!("wrapper: {}", wrapper);
    }

    println!();

    // Advanced Types
    {
        println!("=== Advanced Types ===");
        println!("[Newtype for Type Safety and Abstration]");
        // The newtype pattern can encapsulate some type to assure no one will
        // access some interal methods.
        // Also, it can shape a regular data type, like u32, into some specific
        // one and prevent the programmer to call some method using the wrong data.

        println!("[Type Synonyms]");
        // In the same way as newtypes, Rust can mask data type into specific ones.
        // However, it's just an alias to the type. Rust do not check for same type.
        // But Synonyms is very usefull for repetition. If some type is too long,
        // with type synonym there's a way to alias into a smaller name, making the
        // code easier to write and read.

        println!("[The Never Type]");
        // The exclamation point (!) is used in Rust as the Never Type and are used
        // in functions never return. These function are called diverging functions.
        // The never type can be coersed to any data type, since it's nothing.

        println!("[Dynamically Sized Types and the Sized Trait]");
        // Dynamically Sized Types are always hidden under a smart pointer.
        // Every trait is a DST.
    }

    println!();

    // Advanced Functions and Closures
    {
        println!("=== Advanced Functions and Closures ===");
        println!("[Function Pointers]");
        // fn are function pointers (lowercase f)
        // Fn are closure trait
        let result = make_it_double(double_it, 3);
        println!("make_it_double(double_it, 3): {}", result);
        let result = make_it_double(sum_up, 3);
        println!("make_it_double(sum_up, 3): {}", result);
        // Function Pointer implements all three closure traits (Fn, FnMut and FnOnce).
        // This allows to pass a function pointer as argument to any function
        // that expects a closure.

        println!("[Returning Closures]");
        // Closures can't be returned directly because they're represented by traits.
        // Closures does not have a concrete type that is returnable.
        println!("returns_closure()(24): {}", returns_closure()(24));
    }

    println!();

    // Macros
    {
        println!("=== Macros ===");
        // Macros are a family of features of Rust: declarative macros with macro_rules!
        // and three kinds of procedural macros:
        // 1. Custom #[derive] macros that specify code added with the derive attribute
        //    used on structs and enums
        // 2. Attribute-like macros that define custom attributes usable on any item
        // 3. Function-like macros that looks like function calls but operate on the
        //    tokens specified as their arguments.

        // Macros are a way of writing code that writes other code, known as metaprogramming.
        // Metaprogramming reduces tha amount of code to write and maintain (functions).
        // But macros have some additional powers that functions don't.

        // A function requires a signature with a set of variables and their types.
        // Macros can take a variable number and types of parameters.

        // Macros are expanded before the compiler interpretation.
        // Function are called at runtime.

        // Macros are more complex than function to read, understand and maintain.

        println!("[Declarative Macros]");
        // In essence, Declarative Macros are similar to a Rust match expression.
        // Like the match expression, macros also compare a value to a pattern associated
        // to a particular code. When a match happens, all the code associated with the
        // patterns replaces the code passed as macro during compilation.

        //     #[macro_export]
        //     macro_rules! vec {
        //          ( $( $x:expr ),* ) => {
        //              {
        //                  let mut temp_vec = Vec::new();
        //                  $(
        //                      temp_vec.push($x);
        //                  )*
        //                  temp_vec
        //              }
        //          };
        //      }

        // This is simpler version of the vec! macro.
        // The #[macro_export] annotation indicates that this macro is available whenever
        // the crate in which the macro is defined is brought into scope.
        // The macro starts with the macro_rules! and the macro name, followed by the
        // curly brackets denoting the body of the macro.
        // Then comes an arm for the macro. It follows the same structure as match:
        // An expression to be matched, => and the block to be executed for this arm.
        // The expression, however, is different from the one seem before:
        // a. The expression uses a set of parentheses just to wrap the whole pattern
        // b. The dollar sign ($) is used to declare a variable in the macro system
        // c. Then comes a set of parenthesis to capture values that match the pattern
        //    to use in the replacement code.
        // d. $x:expr matches any Rust expression and gives the expression the name $x
        // e. The comma after this indicates that an optional (* - zero or more) separator
        //    can appear
        // The code inside the curly brackets are executed and the code inside $() is executed
        // the amount of times a value matched the $() at the expression

        // Using the macro use:
        let my_vec = vec![1, 2, 3, 4];
        println!("using vec!: {:?}", my_vec);
        // The vec macro is called. Then the 1,2,3,4 is checked by the expression. The match, then
        // assigns values 4 times for $x. So, inside the body block, the code inside the $()
        // are repeated 4 times, one for each value. The previous code, then, is replaced for:
        let my_vec = {
            let mut temp_vec = Vec::new();
            temp_vec.push(1);
            temp_vec.push(2);
            temp_vec.push(3);
            temp_vec.push(4);
            temp_vec
        };
        println!("expliciting vec!: {:?}", my_vec);

        println!("[Procedural Macros: from Attributes]");
        // Procedural macros acts like a function and is type of a procedure.
        // It accepts some code as an input, operate on that code, and produce
        // some code as an output instead of matching patterns and declarative macros' stuff.
        // All the three previously listed types of procedural macros work in
        // a similar way.
        // The definitions of a procedural macro resides in a special crate type inside
        // their own crate.

        //    use proc_macro;
        // 
        //    #[some_attribute]
        //    pub fn some_name(input: TokenStream) -> TokenStream {
        //    }

        // This is the definition of a procedural macro. "some_attribute" is a placeholder
        // for a specific macro variety.
        // It takes some TokenStream and produces a TokenStream as output.
        Pancakes::hello_macro();
    }
}

unsafe fn do_not_trespass() {
    println!("You've trespassed");
}

// Integration with C standard library
// This way, Rust can't check its rules and guarantees.
// The value presented after the extern keyword defines which ABI should be used
// ABI stands for Application Binary Interface.
extern "C" {
    // abs() function
    // Inside extern scope are listed the signatures os external functions from another language
    fn abs(input: i32) -> i32;
}

// This creates an interface to call this Rust method from C code.
// The #[no_mangle] is used to keep the name of the function as is, so it could be used
// from other programming languages.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("I'm at a Rust binary! :)");
}

fn increment_counter(amount: u32) {
    unsafe {
        COUNTER += amount;
    }
}

fn double_it(num: i32) -> i32 {
    num * 2
}

fn sum_up(num: i32) -> i32 {
    num + 1
}

// The function parameter takes a pointer to any function that takes
// an i32 and returns an i32.
fn make_it_double(function: fn(i32) -> i32, num: i32) -> i32 {
    function(num) * function(num)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x * 2)
}
