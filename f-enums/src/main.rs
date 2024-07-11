// The syntax of a enumeration is this:
// The keyword enum, followed by a custom name and curly brackets to determine where the
// enum values will be defined
#[derive(Debug)]
enum GameType {
    RPG,
    FPS,
}

#[derive(Debug)]
struct Game {
    name: String,
    game_type: GameType,
}

// Enums can store values inside it. Each value of the enum will instance distinctly
// Any type of data can be stored inside enum value
// Using this instead of different structs definition makes easy to understands that they are grouped
// and they are the same data type!
#[derive(Debug)]
enum IPAddr {
    IPv4(u8, u8, u8, u8),
    IPv6(String),
}

// It is able to define implementation for enums, like structs
impl IPAddr {
    fn printme(&self) {
        println!("I'm {:?}", self);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    println!("=== Enums ===");
    // Creating and Using Enums
    {
        println!("[Creating and Using Enums]");
        // Enums are used to enumerate ans list all possibilities the program can go across
        // Enum values can only be one at a time
        // Using enums are simple, like structs associated function
        // Enums are like a custom datatype, like Strings, and can be used as so
        let witcher = Game {
            name: String::from("The Witcher 3"),
            game_type: GameType::RPG,
        };
        let cs = Game {
            name: String::from("Counter-Strike"),
            game_type: GameType::FPS,
        };
        println!("cs name: {}", cs.name);
        println!("cs game type: {:?}", cs.game_type);
        dbg!(witcher);
        dbg!(cs);

        let localhost = IPAddr::IPv4(127, 0, 0, 1);
        let loopback = IPAddr::IPv6(String::from("::1"));
        println!("localhost: {:?}", localhost);
        println!("loopback: {:?}", loopback);
        localhost.printme();
    }
    println!();

    // Option
    {
        println!("[Option]");
        // Option<T> is a enum type to define what behavior the code will assume in each case
        // His variants are None and Some(T). The Some variant is an option to do something when the value
        // been evaluated is equal to T value, where T is a generic type.
        // This enum removes the need to null value and it's implications. Either there Some value, or None.

        // By default, this is an Option<i32>
        let some_number = Some(5);
        // This is an Option<char>
        let some_char = Some('e');
        // So T will always assume a real data type at the end of the day
        println!("some_number {:?}", some_number);
        println!("some_char {:?}", some_char);

        // Since None doesn't have a value, Rust compiler can't infer a type for it, so it must be explict defined
        // None value is like there's no valid valid
        let absent_number: Option<i32> = None;
        println!("absent_number {:?}", absent_number);
    }

    println!();

    // match Control Flow
    {
        println!("[match Control Flow]");
        // Match is a control flow like if or for. It checks the pattern of a value to determinate
        // which block of code it will run based on match.
        // match assure that all possible cases are handled.

        println!(
            "cents of {:?}: {}",
            Coin::Penny,
            value_in_cents(Coin::Penny)
        );
        println!(
            "cents of {:?}: {}",
            Coin::Nickel,
            value_in_cents(Coin::Nickel)
        );
        println!("cents of {:?}: {}", Coin::Dime, value_in_cents(Coin::Dime));
        println!(
            "cents of {:?}: {}",
            Coin::Quarter(UsState::Alabama),
            value_in_cents(Coin::Quarter(UsState::Alabama))
        );
        println!(
            "cents of {:?}: {}",
            Coin::Quarter(UsState::Alaska),
            value_in_cents(Coin::Quarter(UsState::Alaska))
        );

        println!("plus_one some(5): {:?}", plus_one(Some(5)));
        println!("plus_one None: {:?}", plus_one(None));

        println!("dice_roll 5: {}", dice_roll(5));
        println!("dice_roll 1: {}", dice_roll(1));
        println!("dice_roll 7: {}", dice_roll(7));
        println!("dice_roll 3: {}", dice_roll(3));
    }

    println!();

    // if let
    {
        println!("[if let]");
        // This syntax it's a simplification for the match case when some of the cases must be covered
        println!("Using if let on Some(8): {:?}", plus_one_if_let(Some(8)));
        println!("Using if let on None: {:?}", plus_one_if_let(None));

        // It also can be used to extract the data stored in enum.
        let v4 = IPAddr::IPv4(127, 0, 0, 1);
        let mut extract: u8 = 0;
        if let IPAddr::IPv4(pt1, pt2, pt3, pt4) = v4 {
            extract = pt1;
            println!("My IPv4: {pt1}.{pt2}.{pt3}.{pt4}");
        }
        println!("Extracted field pt1: {extract}");
        let v6 = IPAddr::IPv6(String::from("::1"));
        if let IPAddr::IPv6(addr) = v6 {
            println!("My IPv6: {addr}");
        }
    }
}

// It checks which coin has been inserted and return it's current value
fn value_in_cents(coin: Coin) -> u8 {
    // The match syntax is the match keyword followed by an expression
    // In this case, the variable coin is the expression
    // Different from if, the value must be evaluated as a boolean value, any type works here
    // Continuing, there's the arms. An arm has a pattern and some code separated by =>
    // The pattern is the base comparision of the expression. The code indicates what will be executed on a match
    // There could exists as many arms as needed
    // For an extensive code, curly brackets should be used
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter state: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plus_one_if_let(x: Option<i32>) -> Option<i32> {
    // Using this sintax, the pattern and the expression are separated by the equal sign
    if let Some(value) = x {
        Some(value + 1)
    } else {
        x
    }
}

fn dice_roll(dice_number: i8) -> String {
    // match is exaustive, it means it cover all possibilites
    // But using a data type that can have infinite values, it will be really exaustive to
    // enumerate all possibilites
    // For this kind of case exists the case _other. Any value that do not match an arm will
    // execute _other block code. Also, only the underscore can
    match dice_number {
        1 => String::from("Stick"),
        5 => String::from("Iron"),
        8 => String::from("Gold"),
        _ => String::from("Trash"),
    }
}
