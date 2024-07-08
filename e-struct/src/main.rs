// A struct is defined using the keywork struct, a user custom name for the data type and a code block
// The properties of the struct are defined inside this code block, aka, the curly brackets
// Make debug print out functionality available
#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    nickname: String,
}

// Tuple Struct. Define with parentesis instead of curly brackets
struct Position(i32, i32);

struct Rectangle {
    height: u32,
    width: u32,
}

// Struct methods are defined OUTSIDE the struct using the impl block
impl Rectangle {
    // Everything inside this block will be associated to the correspondent struct
    // The first and always parameter is self: &Self, or &self for the close friends
    fn calculate_area(&self) -> u32 {
        return self.height() * self.width;
    }

    // In Rust, the getters are named the same as the attributes
    // Methods can have as many parameters as needed, after the self
    fn height(&self) -> u32 {
        self.height
    }

    // An associated function (not method) that creates a new instance of Rectangle as square
    // This type of associated functions are often used as constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // A struct is a custom data type that allows to package together and name multiple related values in a meaningful group.
    // The data inside a struct can be of different types in any order, and can be labeled

    {
        println!("=== Declaration and use ===");
        // With the struct created, the initialization can be done using the given name
        let mut user = User {
            // Need to initialize every property inside curly brackets with their respectives names
            // The order of the properties mustn't be the same as declared
            age: 27,
            name: String::from("Tasi"),
            nickname: String::from("tasi"),
        };
        // To use a value stored inside a struct, the dot notation is used
        println!("user nickname: {}", user.nickname);
        println!("user age: {}", user.age);
        println!("user name: {}", user.name);
        // To change the value is the dot notation as well, as long as the struct variable is mutable
        // All data inside struct will be mutable if the variable is defined as mutable
        // There's no option for one single data be immutable
        user.age = 28;
        println!("user new age: {}", user.age);

        // If a variable has the same name as the struct property, it's no necessary to rewrite the variable name in declaration
        let name = String::from("Steve");
        let usr_stv = User {
            age: 55,
            nickname: String::from("Universe"),
            name,
        };
        println!("usr_stv nickname: {}", usr_stv.nickname);
        println!("usr_stv age: {}", usr_stv.age);
        println!("usr_stv name: {}", usr_stv.name);

        println!();

        println!("=== Update Syntax ===");
        // Update syntax allows to use an instance to initiate another, MOVING the values, but with some values different
        // First, the values that differ must be declared, then the origin object where the "default" values will MOVED from
        // using the syntax: ..instance
        let usr_stv_copy = User { age: 12, ..usr_stv };
        println!("usr_stv_copy nickname: {}", usr_stv_copy.nickname);
        println!("usr_stv_copy age: {}", usr_stv_copy.age);
        println!("usr_stv_copy name: {}", usr_stv_copy.name);
        // All the moving values standards as applied here, so all Strings and complex types have been moved to the new instance
        // and cannot be used anymore
        // The next line will cause an error and will not compile, since the value inside nickname has been moved
        // println!("{}", usr_stv.nickname);
    }

    println!();

    // Tuple Structs
    {
        println!("=== Tuple Structs ===");
        // Tuple struct aren't named, like tuples =)
        // It's a STRUCTured data type and a Tuple. Might be a tuple default format type.
        // It's used like a tuple and initialized and typed as a struct
        let player_pos = Position(2, 55);
        println!("Player on position x: {}", player_pos.0);
        println!("Player on position y: {}", player_pos.1);
    }

    println!();

    // Example
    {
        println!("=== Example ===");
        let width = 52;
        let height = 6;
        println!("Given the rectangle with width of {width} and height of {height}");
        // Too much parameters. They have some relation?
        println!(
            "Area calculated using function with two variables: {}",
            calculate_area(width, height)
        );
        // Too much parentesis. Not named, witch is what?
        println!(
            "Area calculated using function passing a tuple: {}",
            calculate_area_tuple((width, height))
        );
        // Creating instance of struct
        let my_rec = Rectangle { width, height };
        // Clean. A set of data, well related
        // The instance ownership has passed to the function and do not exists anymore
        println!(
            "Area calculated using function passing the struct ownership: {}",
            calculate_area_struct(my_rec)
        );
        // Rereating instance of struct
        let my_rec = Rectangle { width, height };
        // Clean. A set of data, well related
        // The instance ownership has passed to the function and do not exists anymore
        println!(
            "Area calculated using function borrowing a struct: {}",
            calculate_area_struct_borrow(&my_rec)
        );
    }

    println!();

    // Display trait
    {
        // The print macro, and its derivatives, all uses the Display trait to print the values informed
        // By default, the primitive types (like u8, i32, boolean) all have the trait implemented so Rust can display them
        // Structs doesn't already have them implemented, but can be implemented
        println!("=== Display Trait ===");
        let usr = User {
            name: String::from("Tasi"),
            age: 28,
            nickname: String::from("tasigp"),
        };

        // There is a form to print an object in debug format using the placeholder :?
        // This placeholder will use the Trait Debug (not implemented yet. what a surprise)
        println!("Using :? to print usr struct User: {:?}", usr);
        // The placeholder :#? will print the data struct debug even prettier
        println!("Using :#? to print usr struct User: {:#?}", usr);
        // The dbg! macro also prints in debug mode, but it takes ownership, so it's smarter to pass the reference
        // This macro adds file and line information about the point where it is inserted
        // Every string divided by comma is a value to be displayed as debug.
        // dbg! macro does't format values
        // dbg! macro prints to the Error Output (stderr) while print! macros prints to standard output (stdout)
        dbg!("Using dbg! to print {}", &usr);
        // dbg! macro can be used to evaluate operations and, since it gives back the ownership, to evaluate and declare
        let sum = dbg!(2 * 4);
        println!("sum value: {sum}");
        let boolean_op = dbg!((false && true) || (!false && false));
        println!("boolean_op value: {boolean_op}");
    }

    println!();

    // Methods
    {
        println!("=== Methods ===");
        // Methods are the same as functions, but are defined within the context of a struct and the first parameter is always self

        let my_rect = Rectangle {
            width: 20,
            height: 44,
        };
        println!(
            "My Rectangle [height: {}, width: {}] has area calculated by method of: {}",
            my_rect.height,
            my_rect.width,
            my_rect.calculate_area()
        );

        println!("[Associated Functions]");
        // Associated are functions declared inside impl block, and will be associated with the type defined after impl
        // Not all associated functions should have self as first parameter
        // To call the associated function, double double-colon is used
        let my_square = Rectangle::square(5);
        println!(
            "my_square [height: {}, width: {}] has area calculated by method of: {}",
            my_square.height,
            my_square.width,
            my_square.calculate_area()
        );
    }
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn calculate_area_tuple(rec: (u32, u32)) -> u32 {
    rec.0 * rec.1
}

fn calculate_area_struct(rec: Rectangle) -> u32 {
    rec.height * rec.width
}

fn calculate_area_struct_borrow(rec: &Rectangle) -> u32 {
    rec.height * rec.width
}
