// Using dependency
use rand::Rng;

fn main() {
    // Variables and mutability
    {
        println!("=== Variables and mutability ===");

        let x = 5;
        println!("The value of x is: {x}");
        // Results in an error, because variables in Rust are immutable by default
        // x = 6;
        println!("The value of x is: {x}");

        // The keyword mut allow to change y value
        let mut y = 7;
        println!("The value of y is: {y}");
        y = 6;
        println!("The value of y is: {y}");

        // While variables are declared using let, having the possibility to be mutable and
        // not required to specify the type, constants are declared using only const
        // and require type definition. Constants must be declared in caps lock and underscores
        const MINUTES_IN_A_DAY: u32 = 24 * 60;
        println!("There is {MINUTES_IN_A_DAY} minutes in a day");
    }

    println!();

    // Shadowing
    {
        println!("=== Shadowing ===");

        // Rust allows developer to reutilize variables names to new declared variables inside the same scope
        // Shadowing is the same as declaring a mutable variable, but will prevent accidental compile-time error
        // from changing a immutable variable value
        let tasi = 18;
        println!("Value of tasi is: {tasi}");

        // This variable is shadowing the previous definition
        let tasi = 22;
        println!("The new value of tasi is: {tasi}");

        // This represents a different scope inside a function
        // which means that all variables declares here will be destroyed
        // at the end of this block
        {
            println!("[Entering Inner scope]");
            // This variable is shadowing the previous definition, but only inside this scope
            let tasi = tasi * 2;
            println!("Inner tasi has value of: {tasi}");
            println!("[Exiting Inner scope]");
        }

        // Outside the Inner Scope, the last value of the variable returns
        println!("Meanwhile, outside tasi keeps it's value of: {tasi}\n");
        // Shadowing can be used by different types of data, which couldn't be done using mutable variables
        // Declaring a immutable variable without specifying the datatype, the compiler will assume some,
        // in this case, is String
        let name = "Tasi";
        print!("Hello, my name is {name}");
        // Shadowing, the compiler will redeclare as u32 (default for numerics)
        let name = name.len();
        println!(" and it has a length of {name}!");
    }

    println!();

    // Data Types
    {
        println!("=== Data Types ===");

        // Rust is statically typed and the compiler must know the type of the variables at compile time
        // But there is no need to specify, as the compiler can infer based on the value, unless there's a need to specify

        println!("[Numerics]");
        // Integers can be defined by signed and unsigned (i or u, respectively), end by length (8,16,32,64,128):
        // i8, u8, i16, u16, i32, u32, i64, u64, i128, u128
        // Theres also isize and usize dependent on the architecture of the computer (32 or 64-bits)
        // Big integers could be declared using underscore to separate and make numbers more readable, but will result in the same
        let big_integer = 98_222;
        println!("Big Integer: {big_integer}");
        // Also could be declared using hexadecimals, using prefix 0x
        let in_hex = 0xA5;
        println!("Hex: {in_hex}");
        // in octal, using prefix 0o
        let in_oc = 0o54;
        println!("Octal: {in_oc}");
        // in binary, using prefix 0b, using undescore to separate in groups, like big integers
        let in_bin = 0b0010_1101;
        println!("Binary: {in_bin}");
        // and, for encrypt purpouse, in Byte (u8)
        let in_byte = b'T';
        println!("Byte: {in_byte}");

        println!("[Numeric Operations]");
        // All language-commom operations
        // Detail: all vlaues must have the same type
        let lhs = 8;
        let rhs = 19;
        println!("Sum: lhs + rhs [{lhs} + {rhs}] = {}", lhs + rhs);
        println!("Difference: lhs - rhs [{lhs} - {rhs}] = {}", lhs - rhs);
        println!("Multiplication: lhs * rhs [{lhs} * {rhs}] = {}", lhs * rhs);
        println!(
            "Division (truncated): lhs / rhs [{lhs} / {rhs}] = {}",
            lhs / rhs
        );
        // To have a division in quocient notation, the operation must be done using floats
        {
            let lhs = 8.0;
            let rhs = 19.0;
            println!(
                "Division (quocient): lhs / rhs [{lhs} / {rhs}] = {}",
                lhs / rhs
            );
        }
        println!("Remainder: lhs % rhs [{lhs} % {rhs}] = {}", lhs % rhs);

        // Reminder: all types can be auto determined by the compiler. Here it's explicit for learning purpouse only
        println!("[Boolean]");
        let t: bool = true;
        println!("bool t value: {t}");

        // Chars are declared with SINGLE quotes; DOUBLE is a string
        // Chars represente Unicode Scalar Value (more of it in section H)
        println!("[Chars]");
        let c: char = 't';
        println!("char c value: {c}");
        let heart_eyed_cat = '😻';
        println!("emoji?????: {heart_eyed_cat}");
    }

    println!();

    // Compound types
    {
        println!("=== Compound types ===");

        // Compund types can group multiple value types
        println!("[Tuples]");
        // Tuples have an fixed size and can't grow or shrink in size
        // While declaring a tuple, the type can be defined for each value it will store (or infered by the compiler)
        let my_tuple: (u32, i8, u16) = (4_000_000_000, -110, 60_000);
        // Accessing a tuple element is like accessing object element, using the dot (.) and the index
        println!("First value from tuple: {}", my_tuple.0);
        println!("Second value from tuple: {}", my_tuple.1);
        println!("Third value from tuple: {}", my_tuple.2);
        // Also, we can separate tuple values in diferent variables in one line
        let (first, second, third) = my_tuple;
        println!("First value: {first}");
        println!("Second value: {second}");
        println!("Third value: {third}");

        // Arrays values must all have the same type
        println!("[Arrays]");
        // Arrays also have fixed length
        // To explicit declare an array, must be specified type and size, separated by a semicolon
        let my_array: [char; 4] = ['a', 'b', 'c', 'd'];
        // An array can be initialized with all elements beeing the same
        let all_values_equals = [55; 5];
        // Accessing array element is the name and index inside brackets, starting in zero
        println!(
            "my_array values: {},{},{},{}",
            my_array[0], my_array[1], my_array[2], my_array[3]
        );
        println!(
            "all_values_equals values: {},{},{},{},{}",
            all_values_equals[0],
            all_values_equals[1],
            all_values_equals[2],
            all_values_equals[3],
            all_values_equals[4]
        );
    }

    println!();

    // Functions
    {
        println!("=== Functions ===");
        my_function();
        parameter_function(16, 'G');

        // Creating a statement with an expression
        let statement = {
            let temp = 5;
            // Do not has semicolon at the end -> Expressions do not require ending semicolons
            // An expression will always returns it's value. So, here, it is like a function call
            temp * 2
        };
        println!("Here's the value of statement {}", statement);

        println!("Double the value of 6 using return: {}", return_function(6));
        println!(
            "Double the value of 48 without semicolon: {}",
            return_expression_function(48)
        );
    }

    println!();

    // Control Flow
    {
        println!("=== Control Flow ===");

        println!("[If]");
        let (min, max) = (1, 100);
        // Getting a random number to use on If
        let secret_number = rand::thread_rng().gen_range(min..=max);
        println!("Generated number [{min}-{max}]: {secret_number}");

        // If statement do not require parenthesis
        if secret_number > 75 {
            println!("Higher than 75");
        } else if secret_number > 50 {
            println!("Higher than 50");
        } else {
            println!("Lower, or equals, than 50");
        }

        // An if statement can be used to initialize a variable
        // Note: all returning types must be the same, or will cause compilation error
        // Note the abscence of semicolon at the end of the arms
        let variable_condition = if secret_number % 5 == 0 {
            "Multiple of 5"
        } else {
            "Not multiple of 5"
        };
        println!("The generated number is {variable_condition}");
        println!();

        println!("[Loop]");
        // The loop executes infinitely, until it's been breaked by user or by the keywork break
        // While using the keyword break to finish loop execution, it will return anything declared after the keyword
        // The continue keyword will ignore the rest of the block
        let mut counter = 0;
        let loop_value = loop {
            counter += 1;
            if counter <= 5 {
                continue;
                println!("If this gets printed, somethings wrong! PANIC!");
            }
            println!("This will be ignored while counter is lower than 5");
            println!("Actual counter value: {counter}");
            if counter == 10 {
                break "done";
            }
        };
        println!("loop is {loop_value}");

        // There's always the possibility to put a loop inside other. Loops can be labeled to identify whether should be used by break and continue
        // A loop label uses a SINGLE quote in the beginning. It's the same syntax to use it
        'outer_loop: loop {
            println!("counter value: {counter}");
            let mut wow = 0;
            loop {
                println!("wow value: {wow}");
                if wow == 5 {
                    break;
                }
                if counter == 8 {
                    break 'outer_loop;
                }
                wow += 1;
            }
            counter -= 1;
        }

        println!();

        println!("[While]");
        // The while loops follow the same logic from other programming languages and the same syntax from if
        // Works similar to loop, but simpler since there's no need for ifs and breaks and continues
        let mut while_value = 0;
        while while_value < 10 {
            while_value += 1;
        }
        println!("Smeagol is free!");

        println!();

        println!("[For]");
        // For is used to run through a collection (for-each)
        let coll = ['a', 'b', 'c', 'w'];
        for letter in coll {
            print!("{letter} ");
        }
        println!();

        // As the While loop, For can be used to run a block of code a couple of times using range
        // Here, the "range" is interpreted as a collection of values
        // Note that there's no equal sign, so end is not included
        println!("Printing 1 to 7 (not included) using for");
        for number in 1..7 {
            print!("{number} ");
        }
        println!();
        println!("Printing 6 to 1 (included) using for and reverse");
        // And now in reverse
        // Note the rev() method, a type of iterator
        for number in (1..7).rev() {
            print!("{number} ");
        }
    }
}

// Defining a function in Rust is using the keyword fn + function_name_snake_case + parameters
// The function definition can be anywhere in the code, before or after main, as long as it is in a scope the caller can see
fn my_function() {
    println!("I've created a new function!");
}

// Defining parameter for a functions requires the type
fn parameter_function(variable: u8, letter: char) {
    println!("I received the value {}, wow! {letter}", variable);
}

// Return values are declared using the sufix -> type
// To return a value the keyword return must be used or not inserting semicolon at the end, turning the line into a expression
fn return_function(double_it: u32) -> u32 {
    return double_it * 2;
}

fn return_expression_function(double_it: u32) -> u32 {
    double_it * 2
}
