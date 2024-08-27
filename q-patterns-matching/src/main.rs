#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
enum Action<'a> {
    End,
    Move { x: i32, y: i32 },
    Eat { food: &'a str },
}

fn main() {
    // Patterns syntax is used to give more control over a program's flow.
    // A pattern consists of some combination of the following:
    // 1. Literals
    // 2. Destructured arrays, enums, structs or tuples
    // 3. Variables
    // 4. Wildcards
    // 5. Placeholders
    // Patterns describes the shape of the data. The program matches the value
    // against a pattern if correspond to the shape to run a piece of code.
    // If the pattern matches with some value, the value is used in that piece of code.
    // Patterns are widely used in Rust.

    // match
    {
        println!("=== match ===");
        // The match expression uses Patterns in their arms. Each arm is a pattern match.
        let option: Option<i32> = Some(5);
        // match expressions needs to be exhaustive. That means it will covers all the
        // pattern possibilites
        match option {
            None => println!("Noni"),
            Some(x) => println!("Some({})", x),
        }
        // If, for some reason, the possibilities are endless and only a few matters
        // the rest of the options can be catched using the catchall pattern, an underscore _
        match option {
            None => println!("Noni"),
            _ => println!("Catchau"),
        }
    }

    println!();

    // if let
    {
        println!("=== if let ===");
        // The if let expression is a short way of the match when only a single pattern matter.
        // This will use the if structure, so if else's and else's are allowed and can be mixed
        // between pure if and if let structures when the pattern doesn't match.
        let fav_game: Option<&str> = None;
        let is_coop = false;
        let my_age: Result<u8, _> = "28".parse();

        if is_coop {
            println!("Coop games are fun!");
        } else if let Some(game) = fav_game {
            println!("{} is my fav game", game);
        } else if let Ok(age) = my_age {
            // age is a shadowed variable containing the value Ok() stores
            if age < 30 {
                println!("I'm {} years young", age);
            } else {
                println!("I'm old");
            }
        }

        // This structure, otherwise, doesn't check for exhaustiveness, like match
    }

    println!();

    // while let
    {
        println!("=== while let ===");
        // Works the same way as if let, but acts like the pure while: while some value
        // matches some pattern, that block of code will run
        let mut stack = vec![1, 2, 3];

        // The pop() function returns an Option<>, so, while the return of pop() is
        // Some(something), the block will run. For this example, should run 3 times.
        // pop() returns, and removes, the next element in line. When there's no value
        // to return, pop() returns None and break the pattern while loop.
        while let Some(top) = stack.pop() {
            println!("{top}");
        }
        println!("Out of while let");
    }

    println!();

    // fruit Loops
    {
        println!("=== for Loops ===");
        let games = vec!["cs", "7dtd", "sv", "gta"];
        // Using for each loops, the values of each iteration are the pattern.
        // The enumerate() method produces a tuple as result containing the current index
        // and the correspondent value. This matches with the pattern inserted. So,
        // enumerate() index will be shadowed as curr_index and the value as game.
        for (curr_index, game) in games.iter().enumerate() {
            println!("{}. {game}", curr_index + 1);
        }
    }

    println!();

    // let
    {
        println!("=== let ===");
        // Patterns goes beyond matches and if/while lets.
        // Every let patter is like using patterns. The expression after the equal sign
        // must match the shape of the declaration before the equal sign.
        let single = 5;
        println!("single is the pattern and {single} is the expression");

        // The expression is a tuple of three values as expected in the left side pattern.
        let (first, second, third) = ("some", "random", "value");
        println!("Truple: {first}, {second}, {third}");

        // This like doesn't compile because the expression doesn't match the pattern.
        // let (im, wrong) = ("it", "will", "break");

        // This line compiles, but the last value is lost
        let (im, wrong, _) = ("it", "will", "break");
        println!("im: {im}");
        println!("wrong: {wrong}");
    }

    println!();

    // Function Parameters
    {
        println!("=== Function Parameters ===");
        // Function parameters are also patterns, because it requires the same amount
        // of values of the same data types ans permissions

        // The value passed matches the function expectations: a borrowed tuple of unsigned 8-bit integers.
        print_day(&(15, 08));
    }

    println!();

    // Refutability
    {
        println!("=== Refutability ===");
        // The let expresion, without determining the type is always irrefutable,
        // because the expression will always match the pattern
        let irrefutable = 0;
        println!("im irrefutable: {irrefutable}");

        let refutable: Option<i32> = Some(irrefutable);
        // This is a refutable case, because refutable can be Some() or None and the if let
        // only matches for Some() and other values will never be treated
        if let Some(value) = refutable {
            println!("im refutable: {value}");
        }

        // let and for loops only accepst irrefutable patterns.
        // if let and while let accepts refutable patterns because it will determinate,
        // at runtime, what decision to take over the pattern match.

        let the_none: Option<u32> = None;
        // This line won't compile because let only accepts irrefutable patterns, and None
        // is not covered
        // let Some(value) = the_none;

        if let Some(value) = the_none {
            println!("the_none has some value: {value}");
        }
    }

    println!();

    // Pattern Syntax
    {
        println!("=== Pattern Syntax ===");
        println!("[Matching Literals]");

        // Match literal, like the name sugests, will check if the value is exactly
        // equals.
        let literal = 2;
        match literal {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("too much"),
        };

        println!("[Matching Named Variables]");
        let named = Some(10);
        let to_be_shadowed = "i'm shadowed";

        match named {
            // It will match the shape and then the literal
            Some(15) => println!("Fifteen"),
            // match creates a new scope, so outter "to_be_shadowed" does not exists inside here
            // Here, any value inside Some() will be named as to_be_shadowed, a i32, not a &str
            Some(to_be_shadowed) => println!("Value: {to_be_shadowed}"),
            _ => println!("Anything else"),
        }

        println!("Outter to_be_shadowed: {to_be_shadowed}");

        println!("[Multiple Patterns]");
        let multiple = 50;
        match multiple {
            5 | 10 | 50 => println!("five or ten or fifty"),
            2 => println!("two?"),
            _ => println!("don't know"),
        }

        println!("[Matching ranges]");
        let range = 14;
        match range {
            0..=17 => println!("Underage"),
            _ => println!("Can be arrested"),
        }

        let grade = 'd';
        match grade {
            'a'..='c' => println!("Passed"),
            'd'..='f' => println!("Failed"),
            _ => println!("Failed too hard"),
        }
    }

    println!();

    // Destructuring Value
    {
        println!("=== Destructuring Value ===");
        println!("[Structs]");
        // Matching patterns can be use to destructure a data type like structs to use
        // the inner values

        // Creates a new instance of Person
        let tasi = Person {
            name: "Tasi Pasin",
            age: 28,
        };

        println!("{:?}", tasi);

        // Destructs the Person instance into values.
        // Person.name becomes destruct_name
        // Person.age becomes destruct_age
        let Person {
            name: destruct_name,
            age: destruct_age,
        } = tasi;

        println!("destruct_name: {destruct_name}");
        println!("destruct_age: {destruct_age}");

        // Destructuring can also be used inside match syntax
        match tasi {
            Person { name, age: 0..=18 } => println!("{name} is underage"),
            Person { name, age: 19..=70 } => println!("{name} can be arrested"),
            Person { name, age } => println!("{name} is {age} yo, can be arrested and can retire"),
        }

        println!("[Enums]");

        // Enums can also be destructured and the syntax is almost the same as struct.
        // Destructuring also works for nested Structs and Enums or Tuples.
        match_action(Action::Eat { food: "Meat" });
        match_action(Action::Move { x: 1, y: -9 });
        match_action(Action::End);
    }

    println!();

    // Ignoring values in a Pattern
    {
        println!("=== Ignoring values in a Pattern ===");
        println!("[Entire value with _]");
        // The catchall wildcard will ignore the value completely and won't bind the value
        // It can be usefull in a trait function implementation where, for that particular
        // struct, some parameter is not relevant.
        ignoring_first_value(2, 3);

        println!("[Parts of a value with Nested _]");
        let returned_value = Some(123);
        // The value, for some reason, is not relevant for the process, only the shape.
        match returned_value {
            Some(_) => println!("Some value were returned"),
            None => println!("Returned nothing"),
        }

        // When a variable is not used, putting an underscore at the beggining will
        // prevent compiler warnings.
        let _not_used = 1000;

        let owned = Some(String::from("teste"));
        // Note: owned is owned by the if let, so it can't be used again after this
        if let Some(value) = owned {
            println!("value: {value}");
        }
        // println!("{:?}", owned);

        println!("[Remaining parts using ..]");
        let person = Person {
            name: "Gerald",
            age: 78,
        };
        match person {
            // The age is discarted
            Person { name, .. } => println!("Hello, {name}"),
        }

        let number_series = (1, 2, 3, 4, 5, 6, 73);
        match number_series {
            // All the values between the first and the last are discarted
            (first, .., last) => println!("first: {first} and last: {last}"),
            // This does not work because the compiler can't tell how many values
            // must be skipped before and after second
            // (.., second, ..) => println!("second: {second}"),
        }

        println!("[Extra conditionals with match]");

        let multiple_five = Some(50);
        match multiple_five {
            // Matches Some() and uses its value to an extra condition
            Some(value) if value % 5 == 0 => println!("The value {value} is multiple of 5"),
            Some(value) => println!("The value {value} is not multiple of 5"),
            None => println!("NaN"),
        }

        // The extra if conditional can be used along with other patterns syntaxes:
        // Literals, Matching Names, Multiple and Ranges
    }

    println!();

    // The @ binding
    {
        println!("=== The @ binding ===");
        // The @ operator allow to create a variable that holds a value that passes a pattern match

        let action = Action::Move { x: 5, y: 10 };
        match action {
            // This arm evaluates the pattern and holds the value at temp variable so it
            // can be used inside the arm
            Action::Move {
                x: temp @ 1..=5, ..
            } => println!("Move to x {temp}"),
            // This arm evaluates the pattern, but the value can't be used inside the arm
            Action::Move { x: 10..=50, .. } => println!("Move somewhere between x 1 and 5"),
            _ => println!("Some Action"),
        };
    }
}

fn print_day(&(day, month): &(u8, u8)) {
    println!("Today is {day}/{month}");
}

fn match_action(action: Action) {
    match action {
        Action::Eat { food } => println!("Eating {food}"),
        Action::Move { x, y } => println!("Moving to x:{x}, y:{y}"),
        Action::End => println!("The End"),
    }
}

fn ignoring_first_value(_: i32, used: i32) {
    println!("I'm using only the {used} value");
}
