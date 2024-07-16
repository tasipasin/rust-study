// Include traite example implementations
pub mod summary;
use std::{cmp::PartialOrd, fmt::Display};

use summary::{Summary, Tweet};

// A struct to define a position of an element
// The generic type can be anything, as long as they're the same
// The syntax of generic data type is similar to the function one
#[derive(Debug)]
struct Position<T> {
    x: T,
    y: T,
}

// The implementation block definition for generic data types
// must include the data type after the keyword impl
impl<T> Position<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This implementation only exists for Position datatype where x and y
// has datatypes that implements Display and PartialOrd (like numbers)
impl<T: Display + PartialOrd> Position<T> {
    fn is_ahead(&self, other: &Position<T>) -> bool {
        self.x() > other.x() && self.y > other.y
    }
}

impl Position<i32> {
    fn y(&self) -> &i32 {
        println!("Here is signed");
        &self.y
    }
}

// This struct also defines a position of an element
// but in this case there's a new dimension with a different
// data type for it, the U generic type.
// It may, or may not, be the same as the T type.
// The generic types can be as many as needed using commas
#[derive(Debug)]
struct PositionDiff<T, U> {
    x: T,
    y: T,
    z: U,
}

impl<T, U> PositionDiff<T, U> {
    fn z(&self) -> &U {
        &self.z
    }
}

// The generic lifetime is defined inside angle brackets
#[derive(Debug)]
struct HoldRef<'a> {
    my_ref: &'a str,
}

fn main() {
    // Generics allows to express a behavior of a generic data type without knowing
    // what will be in their place when compiling and running the code
    {
        println!("=== Avoiding Code Duplication ===");
        // Generics are placeholders to represent multiple data types to avoid code duplication

        // Code to be duplicated (find the largest number)
        println!("[Code duplication]");
        {
            let number_list = vec![34, 50, 25, 100, 65];
            println!("First number_list: {:?}", number_list);
            let mut largest = &number_list[0];

            for number in &number_list {
                if number > largest {
                    largest = number;
                }
            }
            println!("First Largest number: {largest}");
        }
        println!();
        {
            // Duplication code
            // This is the only line changing
            let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
            println!("Second number_list: {:?}", number_list);
            let mut largest = &number_list[0];

            for number in &number_list {
                if number > largest {
                    largest = number;
                }
            }
            println!("Second Largest number: {largest}");
        }
        println!();
        // Both codes do the same thing: find the largest number from a vector
        // So it can be a function that receives a vector
        println!("[Using function to avoid code duplication]");

        let number_list = vec![34, 50, 25, 100, 65];
        println!("First number_list: {:?}", number_list);
        println!(
            "First Largest number: {}",
            find_largest_vector(&number_list)
        );

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        println!("Second number_list: {:?}", number_list);
        println!(
            "Second Largest number: {}",
            find_largest_vector(&number_list)
        );
        // The results are the same as before, while the code were duplicated
        // But now the code is more readable
    }

    println!();

    // Generic Data Types
    {
        println!("=== Generic Data Types ===");

        println!("[Divided by function but still code duplication]");
        // To find the largest of any vector, a new function is needed
        // because the type inside the vector could change.
        // Even though this works, there's a new code duplication, because
        // both functions perform the same actions inside them
        let number_list = vec![34, 50, 25, 100, 65];
        println!("number_list: {:?}", number_list);
        println!(
            "Largest number from number_list: {}",
            find_largest_vector(&number_list)
        );

        let char_list = vec!['y', 'm', 'a', 'q'];
        println!("char_list: {:?}", char_list);
        // This line will cause an error because the function find_largest_vector is
        // expecting a vector of i32, but char_list is a vector of chars
        // println!("Largest char from char_list: {}", find_largest_vector(&char_list));
        println!(
            "Largest char from char_list: {}",
            find_largest_char_vector(&char_list)
        );

        println!();
        println!("[Elimitating duplicated code. Pt2]");
        // Now that a generic vector can be passed to the comparator, both
        // lists can use the same function and there's no code duplication
        // The results must all be the same
        println!(
            "Largest number from number_list (generic function): {}",
            find_largest_generic_vector(&number_list)
        );
        println!(
            "Largest char from char_list (generic function): {}",
            find_largest_generic_vector(&char_list)
        );

        println!();
        println!("[Generics in Structs]");

        // The position_precise is a Position using float as the generic type
        let mut position_precise = Position { x: 4.5, y: 9.99 };
        println!("position_precise: {:?}", position_precise);
        // The position_near is a Position using i32 as the generic type
        let mut position_near = Position { x: 5, y: 10 };
        println!("position_near: {:?}", position_near);
        // Even if position_precise is define as mutable, the next line will cause
        // a compilation error because the type expected is float, not i32
        // position_precise.x = 10;
        position_precise.x += 1.2;
        println!("new position_precise: {:?}", position_precise);
        position_near.y -= 9;
        println!("new position_near: {:?}", position_near);

        println!();

        // All the data types are the same: float
        let mut position_y_precise = PositionDiff {
            x: 4.5,
            y: 9.99,
            z: 22.01,
        };
        println!(
            "position_y_precise (all same data type): {:?}",
            position_y_precise
        );
        position_y_precise.z -= 1.0;
        println!("new position_y_precise: {:?}", position_y_precise);

        // There are two differents data types
        let mut position_y_diff = PositionDiff {
            x: 4.5,
            y: 9.99,
            z: "z coordinate",
        };
        println!("position_y_diff (z value is String): {:?}", position_y_diff);
        position_y_diff.y -= 1.0;
        position_y_diff.x += 0.4;
        println!("new position_y_diff: {:?}", position_y_diff);

        // These principles can be used in Enums, as well, like seen in the Option<T> enum,
        // the Result<T,E> enum

        println!();
        println!("[Generics in Methods Definitions]");
        println!("position_precise.x: {}", position_precise.x());
        println!("position_y_diff.z: {}", position_y_diff.z());
        println!("position_near.y: {}", position_near.y());
        // Next line will cause compilation error because, it's the same type
        // as position_near, there's no implementation for y() f32, only for y() i32
        // println!("position_precise.y: {}", position_precise.y());

        // Rusts used monomorphization to deal with generics. This means that it generates
        // the code correspondent for each data type the generic are gonna have to deal during runtime.
    }

    println!();

    // Traits
    {
        println!("=== Traits ===");
        // Traits defines particular funcionalities to a certain data type that can be shared with other data types.
        // Traits groups methods signatures to define a set of a type behavior.
        let my_article = summary::NewsArticle::new(
            "Tasi is learning Rust",
            "For about a month, Tasi has reading Rust's book and learning a lot.",
            "Tasi Pasin",
        );
        println!("my_article: {}", my_article.summarize());

        let my_tweet = summary::Tweet::new(
            "tasi.pasin",
            16,
            "i've been studying so much rust. it's so coool 🤓",
        );
        println!("my_tweet: {}", my_tweet.summarize());

        println!(
            "my_article (never_implemented()): {}",
            my_article.never_implemented()
        );
        println!(
            "my_tweet (never_implemented()): {}",
            my_tweet.never_implemented()
        );

        println!();
        println!("[Trait as Parameter]");
        make_summary(&my_article);
        make_summary(&my_tweet);

        println!();
        println!("[Multiple Traits as Parameter]");
        make_summary_multiple(&my_article);
        // The next line will cause compiler error because the function expects
        // the object to implements Summary AND Display, but Tweet doesn't
        // implements Display
        // make_summary_multiple(&my_tweet);

        println!();
        println!("[Traits Bounds using where]");
        make_summary_multiple_where(&my_article);

        println!();
        println!("[Returning Types that Implements traits]");
        // Function returns_trait() returns a Tweet, which implements Summary
        make_summary(&returns_trait());

        println!();
        println!("[traits to conditionally implement methods]");
        let pos_a = Position { x: 8, y: 9 };
        let pos_b = Position { x: 2, y: 4 };
        let pos_c = Position { x: 1, y: 10 };
        let pos_string_a = Position { x: "X1", y: "y1" };
        let pos_string_b = Position { x: "X2", y: "y2" };
        println!("pos_a: {:?}", pos_a);
        println!("pos_b: {:?}", pos_b);
        println!("pos_c: {:?}", pos_c);
        println!("pos_string_a: {:?}", pos_string_a);
        println!("pos_string_b: {:?}", pos_string_b);
        println!("pos_a is_ahead pos_b: {}", pos_a.is_ahead(&pos_b));
        println!("pos_a is_ahead pos_c: {}", pos_a.is_ahead(&pos_c));
        println!("pos_b is_ahead pos_c: {}", pos_b.is_ahead(&pos_c));
        println!(
            "pos_string_b is_ahead pos_string_a: {}",
            pos_string_b.is_ahead(&pos_string_a)
        );
    }

    println!();

    // Validating References with Lifetimes
    {
        println!("=== Validating References with Lifetimes ===");

        println!("[Lifetime Annotation]");

        // It's used to describe the relationship of references lifetimes
        // Explicits and name a lifetime for the references
        let str_a = String::from("tasi");
        let str_b = "pasin";
        println!("str_a: {}", str_a);
        println!("str_b: {}", str_b);
        println!("Longest String: {}", longest(str_a.as_str(), str_b));

        let str_a = String::from("longer string");
        println!("str_a: {}", str_a);
        let result;
        {
            let str_b = String::from("not that longer");
            println!("str_b: {}", str_b);
            // This won't compile because str_b is String and it's lifetime ends before result
            // and the compiler doesn't know which reference it will return
            // result = longest(str_a.as_str(), str_b.as_str());
            result = longest(str_a.as_str(), "i compile");
        }
        println!("Longest String: {}", result);

        println!();
        println!("[Using Generic Lifetime in Structs]");
        let str_a = String::from("This string is divided_by two for an underscore");
        // Contains a slice of str_a
        let first_part = str_a
            .split('_')
            .next()
            .expect("Please, let this message not be shown");
        let my_struct = HoldRef { my_ref: first_part };
        println!("my_struct: {:?}", my_struct);

        // The three rules the compiler uses to figure out lifetimes in fn and impl
        // After considering these three rules, no reference should be left
        // to figure out it's lifetime. If it happens, compiler will stop with an error
        // 1. Every fn parameter receives a different lifetime parameter
        // fn example<'a>(x: &'a i32, y: &'b i32);
        // 2. If there's only one input lifetime, all output lifetime will be the same
        // fn example<'a>(x: &'a i32) -> &'a i32;
        // 3. If there's multiple input lifetime parameters, but one of them is &self
        // of it's mutable, the self lifetime is assigned to all output lifetime

        // Literal strings always have 'static lifetime
    }
}

// The functions receives any concrete slice of i32
fn find_largest_vector(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// The functions receives any concrete slice of i32
fn find_largest_char_vector(list: &[char]) -> &char {
    let mut largest = &list[0];
    for char in list {
        if char > largest {
            largest = char;
        }
    }
    largest
}

// The generic data type can be any letter um UpperCamelCase
// T is the most used because it's the initial of the word "type"
// The type parameter must be declared it, inside angle brackets,
// in the signature to let te compiler knows what that means
// Readable: Function find_largest_generic_vector is generic over some type T
// It receives a parameter named list and is a slice of some T type.
// It returns a reference to a value of the same T type.
fn find_largest_generic_vector<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        // This line will cause a compiler error because
        // the compiler doesn't know the concrete type and
        // the comparator doesn't work for every data type
        // To fix this, the compiler must known that the generic
        // data type can be Ordered. To enable this, the trait
        // std::cmp::PartialOrd will be used to defines that, at least,
        // the generic type must be a PartialOrd implementation
        // This declarations goes at the function signature
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Traits can be passed as parameter like a generic data type
// In this case, only implementations of the specified trait will be accepted
// In order to do this, the keyword impl should be used before the parameter data type
fn make_summary(item: &impl Summary) {
    make_summary_verbose(item);
}

// This is a generic type known declaration of a function but has the same
// principle from the previous one: only accepts data type that implements the specified trait
// Using this way, but with multiples parameters to the same generic type, will force them to be
// the same concrete type for every parameter
fn make_summary_verbose<T: Summary>(item: &T) {
    println!("make_summary(): {}", item.summarize());
}

// Only data types that implements Summary AND Display could use this method
fn make_summary_multiple(item: &(impl Summary + Display)) {
    make_summary_multiple_where(item);
}

// Using the where clause makes the function signature more readable
// and actuates the same way as the previous function
fn make_summary_multiple_where<T>(item: &T)
where
    T: Summary + Display,
{
    println!("make_summary(): {}", item.summarize());
}

// This can return any type that implements Summary
fn returns_trait() -> impl Summary {
    Tweet::new("rust", 12, "i'm here")
}

// The lifetime Annotation is done similaras the generic type: inside angle brackets
// The Lifetime Annotation means the return reference will be valid as long as the
// parameters are valid
// When a reference is returned from a function, it should be relate to any of
// entry variables, or a value created inside the function (which will cause a
// dangling reference)
fn longest<'a>(str_a: &'a str, str_b: &'a str) -> &'a str {
    if str_a.len() > str_b.len() {
        str_a
    } else {
        str_b
    }
}
