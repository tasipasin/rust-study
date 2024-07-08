fn main() {
    // Rust uses ownership to deal with memory managing following the rules:
    // Each value has an owner
    // There can only be one owner at a time
    // When the owner is gone, the value is gone too

    println!("=== Scope ===");
    let outter_scope = "I'm from outter scope";
    // This creates a inner scope, so the variables inside this is new scope will only be available inside it
    // Inside the inner scope, the variables declares BEFORE it's definition also can be used
    // Also, when you shadow a variable inside an inner scope, it'll be shadowed only inside. Once the scope
    // is finished, the variable will assume the outter scope value
    {
        let inner_scope = "I'm from inner scope";
        println!("outter_scope: {outter_scope}");
        println!("inner_scope: {inner_scope}");
        let outter_scope = "I'm shadowing outter_scope";
        println!("shadowing outter_scope: {outter_scope}");
    }
    println!("outter_scope outside inner scope: {outter_scope}");

    println!();

    // String Type as Example for Ownership
    {
        println!("=== String Type as Example for Ownership ===");
        // Strings are a complex data type of unknown size, so they're stored at the heap (slower)
        // String is different from string literal
        // In string literal the content is known at compile time, so they're fast and efficient
        // string literal is immutable

        // Creating String from string literal
        // This type of String can be mutable and grow, so the amount of heap memory needed is unknown at compile time
        // meaning that the memory will be requested at runtime and the memory should be returned to the allocator when
        // the String is "gone"
        // The double colon operator (::) is used to use from from the String namespace
        // The String::from is the one who request memory at runtime
        // To free the allocated space, Rust automatically understands that the variable that owns that space
        // went out of scope and returns the value to the allocator
        let mut string = String::from("I'm will become a String");
        string.push_str("! WOOOOW");
        println!("{string}");
    }
    // At this moment, the scope is over, and the space required for the string variable will now be returned to the allocator
    // To return the memory to the allocator, once the variable goes out of scope, a function called drop is called

    println!();

    // Move
    {
        println!("=== Move ===");
        // Always from the beginning

        // This will create variable a with value of 3, and the next line will create a COPY from a value. So b has the value of 3 as well
        // b been mutable, it's value can be changed and won't affect a value
        let a = 3;
        let mut b = a;
        println!("a value: {a}");
        println!("b value: {b}");
        println!("Changing b value");
        b = 4;
        println!("new a value: {a}");
        println!("new b value: {b}");

        // While using Strings, and other complexes data types, when the same is done, it's not the value that's copied, but the structure
        // This means that the pointer, the capacity and the length are copied (that are on the stack), but not the value (on the heap)
        // In another words: Rust copies the Stack, not the Heap
        // Some attention must be drawn into this:
        // First, when this variables goes out of scope, drop will be called and will try to free both memories, but they're the same!
        // It's a bug called double free error, and must cause memory corruption, leading to bigger problems
        // So, Rust detects this is something wrong and a waste of memory, and prevent some error to occur, considering the first variable
        // is no longer needed (valid) and it's out of scope already. This condition means that Rust won't call drop function like has never
        // seen this variable. Obviously, it means that it cannot be used after again, it's dead!
        // It have the principles os shallow copy, but adds the step of invalidating the first variable, so it's called move.
        let str_a = String::from("My Original String");
        println!("Before copy str_a: {str_a}");
        let str_b = str_a;
        // The following command will cause an compile time error (safety)
        // println!("{str_a}");
        println!("After copy str_b: {str_b}");
    }

    println!();

    // Clone
    {
        println!("=== Clone ===");

        // For some case, if deep copy is needed, ie, the complex object must copy the value and not the structure,
        // clone function must be called, so the process defined by move operator won't be execute and both
        // memories should have to be free on exit scope
        let str_a = String::from("Origin is nice");
        println!("str_a before clone: {str_a}");
        let mut str_b = str_a.clone();
        println!("str_b cloned from str_a: {str_b}");
        str_b.push_str(", but Revelations is better");
        println!("str_a after str_b push: {str_a}");
        println!("new str_b: {str_b}");

        // In stack-only data, like integers, "clone" is always the proccedure
        // It happens because this data types have fixed data size to be stored in stack
    }

    println!();

    // Ownership and functions
    {
        println!("=== Ownership and functions ===");

        // Passing values to functions works exactly as move: complex data types gives ownership
        // and simple data types makes copy
        let my_str = String::from("I'm traveling to a function!");
        takes_ownership(my_str);
        // The next line will cause a compiler error, since the ownership was given to the function
        // Even the function only printing the value and nothing else, Rust understands that the
        // "original" variable is no longer valid in this scope. Like before in "Move"
        // my_str.push_str("adding String");

        // Passing a simple data in the same way as a complex data, won't cause the original variable
        // to be invalid and can be reused over and over
        let mut my_nmbr = 55;
        makes_copy(my_nmbr);
        my_nmbr += 2;
        makes_copy(my_nmbr);

        // Using return will cause the same simptoms as passing or moving values: the ownership will change
        let mut my_str = String::from("Going and back!");
        my_str = takes_and_giveback(my_str);
        println!("Received back the string: {}", my_str);
        takes_ownership(my_str);
        println!("Now i've lost it :(");
    }

    println!();

    // References and Borrowing
    {
        println!("=== References and Borrowing ===");

        // Instead of delivery the variable ownership to the method it's been passing to,
        // a reference for the variable will be passed, in Rust, it's called Borrowing.
        // Using in this way, the variable can be used after the function call
        // For being borrowed, str cannot be changed
        let str = String::from("I'm original");
        let size = knows_where_is(&str);
        println!("My str \"{str}\" has lenght of {size}");

        // To be allowed to change a borrowed variable, first it must be declared to me mutable
        let mut str = String::from("Another String to be mutable");
        // Then the function should define mutability in it's definition
        // Then, when the variable is being borrowed to function, it also be defined as mutable
        println!("str before be borrowed: {str}");
        by_reference_and_change(&mut str);
        println!("str after be borrowed: {str}");
        // A mutable borrowed variable can be borrowed more than once preventing data races at compile time
        {
            // This miniblock of code will cause a compile exception
            // let brr_1 = &mut str;
            // let brr_2 = &mut str;
            // println!("{} {}", brr_1, brr_2);
        }
        // In other hand, a data reference can be borrowed more than once if it's not mutable borrowed
        {
            let brr_1 = &str;
            let brr_2 = &str;
            // But once a data is borrowed being mutable, a compiler error will be caused
            // let brr_3 = &mut str;
            println!("Multiple non mutable borrowed: {} {}", brr_1, brr_2);
        }
        // But if the non mutable ones are not being used after mutable one is created, there is no problem
        {
            let brr_1 = &str;
            let brr_2 = &str;
            // Last use/reference of brr_1 and brr_2
            println!("Multiple non mutable borrowed: {} {}", brr_1, brr_2);
            let brr_3 = &mut str;
            // This will change the original string as well
            brr_3.push_str(" brr_3");
            println!("After ultiple non mutable borrowed, but mutable and changed: {brr_3}");
        }

        // Rust prevents Dangling References
        // Function simulating a dangling reference (do not compile)
        // let &s = dangling_reference();
        // Corrected function
        let s = not_a_dangling_reference();
        println!("Safe return: {s}");
    }

    println!();

    // Slices
    {
        println!("=== Slices ===");
        // Slices let you reference a contiguous sequence of elements in a collection, or the whole collection
        // It creates it's own properties from the string, like start address and length
        // A slice does not have ownership
        // A slice cannot be changed, are immutable
        // A slice is tied to it's origin

        println!("=== String Literal ===");
        let full_str = String::from("I'm a full String");

        println!("full_str: {full_str}");
        let pt1 = &full_str[0..=5];
        println!("pt1: {pt1}");
        let pt2 = &full_str[5..];
        println!("pt2: {pt2}");
        let slice = first_word(&full_str);
        println!("First Word from full_str: {slice}");
        // Once the originary collection goes out of scope, or is changed, all slices are droped

        // String literals are, actually, string slices &str
        let literal = "Wanna play";
        // A literal can't be use as String. The next line cause compiler error
        // let slice = first_word(literal);
        // But a String can be used as literal string
        // String literals are not borrowed or given ownership
        let literal_slice = first_word_literal(literal);
        println!("literal: {literal}");
        println!("literal slice: {literal_slice}");
        let not_literal_slice = first_word_literal(&full_str);
        println!("not literal slice: {not_literal_slice}");

        println!();
        println!("=== Other slices ===");
        // Strings are a collection of chars. This means slices works for every type of colletion
        let array = [1, 2, 3, 4, 5];
        let arr_slice = &array[2..5];
        print!("Full Array: ");
        for element in array {
            print!("{element} ");
        }
        println!();
        print!("Sliced Array: ");
        for element in arr_slice {
            print!("{element} ");
        }
        println!();
    }
}

fn takes_ownership(str: String) {
    println!("takes_ownership took string: {str}");
}

fn makes_copy(simple_value: i32) {
    println!("makes_copy copied the received value: {simple_value}");
}

fn takes_and_giveback(str: String) -> String {
    println!("takes_and_giveback took string: {str}");
    println!("takes_and_giveback giving back string");
    // This returns values because semi-colon is missing (mindblowing)
    str
}

fn knows_where_is(value: &String) -> usize {
    // When value get's out of scope, because the function do not have it's ownership,
    // it won't be dropped
    // In other hand, a borrowed value cannot be modified, because it's immutable by default
    return value.len();
}

fn by_reference_and_change(value: &mut String) {
    value.push_str(" Borrowed and changed rerere");
}

// This function does not even compile, because it's returning the referente to
// a variable that will be droped onde out of function scope
// fn dangling_reference() -> &String{
//     let s = String::from("A dangling reference value");
//     &s
// }

// To solve dangling reference in this case, the string resulted will be returned directly
// moving out it's ownership.
fn not_a_dangling_reference() -> String {
    String::from("Not a dangling reference")
}

// Returns a slice from str
// The return type &str means a "string slice"
fn first_word(str: &String) -> &str {
    first_word_literal(&str)
}

// Returns a slice from str
// The return type &str means a "string slice"
fn first_word_literal(str: &str) -> &str {
    let str_bytes = str.as_bytes();
    for (index, &value) in str_bytes.iter().enumerate() {
        if value == b' ' {
            return &str[0..index];
        }
    }
    &str[..]
}
