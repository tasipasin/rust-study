use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("=== Error Handling ===");

    // Panic
    {
        println!("[Panic]");
        // Panic is a type of error called unrecoverable.
        // This type of error happens when something in the code goes wrong
        // and it should be stopped
        // The panic! macro will display the error, unwind, stack and quit.

        // This will exit the code
        // panic!("OMG WHAT HAPPENED");

        // When displayed on console, the last two lines are the important
        // (a line called note can appear between them forming three lines, but it doesn't matter)
        // The first one of them will write out the panic message and one line before
        // will show where the error occurred (file:row:column)

        println!(".Backtrace.");

        let vect = vec![1, 2, 3];
        println!("My vector: {:?}", vect);
        // This will cause panic because the vector barely has 3 values, who will say 56

        // println!("Value from position 55: {}", vect[55]);

        // When running this with environment variable RUST_BACKTRACE=1 will show the stack of
        // the error to where it happened, or cause to happened.
    }

    println!();

    // Result
    {
        println!("[Result]");
        // When an error can be recoverable, it will return Result to be evaluated on runtime
        // Result data type can have the values OK(T) or Err(E), where T and E are generic types
        // T represents a data on succeed, and E, when fails.

        // Reads from a text file
        // The open method returns a Result<T, E> type
        // T is an implementation of File::open with the success value, std::fs::File
        // used to handle the File
        // E is the std:io:Error.
        // This types can change to match the specific method
        // FunFact: the root directory from file is the root project folder
        let file_content_try = File::open("myFile.txt");
        let file_content = match file_content_try {
            Ok(value) => {
                println!("Opened file :)");
                // Returns the file handler
                value
            }
            // Err(error) => panic!("I'm panicking: {:?}", error),
            // Can treat the error for kind
            Err(error) => {
                println!("Failed opening file");
                match error.kind() {
                    // Another match to check the kind of the error, if needed
                    ErrorKind::NotFound => match File::create("myFile.txt") {
                        Ok(fc) => {
                            println!("File Created yey");
                            fc
                        }
                        Err(err) => panic!("Error creating File: {:?}", err),
                    },
                    // That will panic if error is not expected
                    _ => panic!("Wut?"),
                }
            }
        };
        println!("file_content: {:?}", file_content);
    }

    println!();

    // unwrap and expect
    {
        // Even match working very nice, it's a little too much verbose
        // A shortcut to this is the unwrap implementation
        // It returns the value inside the Ok value from Result,
        // or will call the panic! macro
        println!("[unwrap and expect]");

        // This will return the file handler, because the file will be created
        // previously in this code
        let file_handler = File::open("myFile.txt").unwrap();
        println!("file handler unwrap: {:?}", file_handler);

        // This will cause a panic, cause the file doesn't exists
        // The error will be the one show before
        // let file_handler = File::open("newFile.txt").unwrap();

        // The expect message also returns the file handler from the Ok match arm,
        // but it also defines the panic! error message
        // let file_handler = File::open("newFile.txt").expect("newFile.txt not found");

        // The expect format is most used, because the error message can be clearer
        // and give more context about the error
    }

    println!();

    // Propagating Errors
    {
        // Propagating error is when an error is returned from a function with a implementation
        // that can fail
        // Using this, the one who calls it can handle the error the way it intends
        println!("[Propagating Errors]");

        // Will not cause an error
        let file_content = read_from_file("readLine.txt").unwrap();
        println!("file_content read from file readLine.txt: {}", file_content);

        // Will cause an error
        // Choose to not to call panic! here
        match read_from_file("readLineNotExists.txt") {
            Ok(value) => println!("Won't use this arm {value}"),
            Err(_) => println!("This is an error"),
        };

        println!();
        println!("Using a version with shortcut");

        let file_content = read_from_file_shortcut("readLine.txt").unwrap();
        println!("file_content read from file readLine.txt: {}", file_content);

        // Will cause an error
        // Choose to not to call panic! here
        match read_from_file_shortcut("readLineNotExists.txt") {
            Ok(value) => println!("Won't use this arm {value}"),
            Err(_) => println!("This is an error"),
        };

        println!();
        println!("Using a simpler version");

        let file_content = read_from_file_simpler("readLine.txt").unwrap();
        println!("file_content read from file readLine.txt: {}", file_content);

        // Will cause an error
        // Choose to not to call panic! here
        match read_from_file_simpler("readLineNotExists.txt") {
            Ok(value) => println!("Won't use this arm {value}"),
            Err(_) => println!("This is an error"),
        };

        println!();
        println!("Using an atomic version");

        let file_content = read_from_file_atomic("readLine.txt").unwrap();
        println!("file_content read from file readLine.txt: {}", file_content);

        // Will cause an error
        // Choose to not to call panic! here
        match read_from_file_atomic("readLineNotExists.txt") {
            Ok(value) => println!("Won't use this arm {value}"),
            Err(_) => println!("This is an error"),
        };
    }

    println!();

    // The ? operator
    {
        println!("[The ? operator]");
        // This operator can only be used when the function using it return type is the same
        // as the type returned by the function that returns it
        // For instance, the next line will cause a compiler error, because main is returning nothing
        // let test = File::open("someFile.txt")?;

        // The ? operator expects the function to return a FromResidual implementation like Result or Option
        // When using with Option, it will return None early or Some<T> later
        println!(
            "last from [{}]: {}",
            "nois é mau",
            question_mark_option("nois é mau").unwrap()
        );
        println!(
            "last from [{}]: {}",
            "",
            match question_mark_option("") {
                Some(_) => "some",
                None => "noni",
            }
        );
        println!(
            "last from [{}]: {}",
            "\\nopa",
            match question_mark_option("\nopa") {
                Some(_) => "some",
                None => "noni",
            }
        );
    }

    println!();

    // panic! or not
    {
        println!("[panic! or not]");
    }
}

// Should return the first line or an error
// Returns a type of Result<T, E>
// T is the concrete data type to be returned as Ok, String in this case
// E is the concrete error type to be returned, io::Error in this case
fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    // If the function exit without any problem, it means, the file exists and there's a line to be read
    // it will return Ok with the first line as value
    // If any problems are encountered, will be retured Err containing the error

    let file_handler_result = File::open(file_name);
    let mut file_handler = match file_handler_result {
        Ok(file) => file,
        // An error can be returned from here io::Error
        Err(e) => return Err(e),
    };
    let mut result = String::new();
    match file_handler.read_to_string(&mut result) {
        Ok(_) => Ok(result),
        // An error can be returned from here
        Err(e) => Err(e),
    }
}

// Same as previous function, but using shorter
fn read_from_file_shortcut(file_name: &str) -> Result<String, io::Error> {
    // The ? operator means that if the Result from the operation is OK,
    // the value inside it will be assigned to the variable
    // If the value of Result is an Err, it will return from the whole function,
    // ending it's execution
    // Using match, the error returned must be the same as defined in return type
    // The ? operator automatically converts to the type defined in return type
    // It can be used to converge to one error type having multiple types inside the function
    // An error can be returned here
    let mut username_file = File::open(file_name)?;
    let mut username = String::new();
    // An error can be returned here
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Simpler version from previously function
fn read_from_file_simpler(file_name: &str) -> Result<String, io::Error> {
    // The ? operator means that if the Result from the operation is OK,
    // the value inside it will be assigned to the variable
    // If the value of Result is an Err, it will return from the whole function,
    // ending it's execution
    // Using match, the error returned must be the same as defined in return type
    // The ? operator automatically converts to the type defined in return type
    // It can be used to converge to one error type having multiple types inside the function
    let mut username = String::new();
    // An error can be returned here
    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}

// Atomic version from previously function
fn read_from_file_atomic(file_name: &str) -> Result<String, io::Error> {
    // The standard library provides an easier way to read files
    // All the processes done in the previous functions are done inside it
    fs::read_to_string(file_name)
}

fn question_mark_option(text: &str) -> Option<char> {
    // The lines() will create an iterator over the text lines
    // next() will get the next line of the iterator (as is the first call, the first line)
    // At this point, if text is empty, next() will return None and finish function execution
    // If text is not empty, it will return Some<String>
    // The ? extract the String from Some and now the data type is String (or &str)
    // Then chars() will provide an iterator of chars
    // and last() will return the Option of the last of the iterators position
    text.lines().next()?.chars().last()
}
