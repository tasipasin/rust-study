use std::collections::HashMap;

fn main() {
    // Rust's standard library contains a lot of collection data structures.
    // These data structures can hold multiple values.
    // Unlike array and tuple, collection's values are stored in the heap, ie, the amount of data
    // doesn't need to be known at compile time and can fluctuate in execution time.
    println!("=== Collections ===");

    // Vector
    {
        println!("[Vectors]");
        // Vectors only store data from the same type.
        // Vector's values are stored next to each other in memory

        // While declarating a new vector, the type should be specified because it can't be
        // inferred right away.
        let mut my_vector: Vec<i32> = Vec::new();
        println!("my_vector: {:?}", my_vector);

        // Another way to initialize a vector is using the macro vec!
        // Because the vector already have some values, the type can be inferred
        // In this case, i32 because is the default integer type
        let initialized_vector = vec![1, 2, 55];
        println!("initialized_vector: {:?}", initialized_vector);

        println!("[Updating a vector]");

        // To update/alterate a vector, it must be declared as mut
        // Pushing new values to a vector can cause the compiler
        // to change their addresses in memory, since the values are put next to each other
        my_vector.push(45);
        my_vector.push(7);
        my_vector.push(99);
        println!("my_vector updated: {:?}", my_vector);

        println!("[Reading from a vector]");

        // Values can be retrieved from a vector using direct indexing
        // This method can cause an runtime error if the wrong index is inserted
        let first = &my_vector[0];
        println!("first element (indexing): {first}");
        // By using the vector's method get(), it will return an Option
        // This is the nullpointerexception free method
        let third = my_vector.get(2);
        println!("Getting the third value from my_vector");
        print_returned_value(third);
        // this case will try to get a value from an inexistent index
        // Here, the value is only borrowed, it do not change the ownership
        let inexistent = my_vector.get(55);
        println!("Getting the fifty-sixth value from my_vector");
        print_returned_value(inexistent);

        println!("[Iterating a vector]");
        print!("Values from my_vector on for-each iteration: ");
        for value in &my_vector {
            print!("{value} ");
        }
        println!();

        // Iterators can be used to apply some changes to vector's values
        // For this to occurr, the collection must be marked as mutable
        println!("Changing value over iterations (adding 1):");
        for value in &mut my_vector {
            print!("old value [{value}], ");
            // Dereference must be used
            *value += 1;
            println!("new value [{value}]");
        }
        println!("my_vector's new values: {:?}", my_vector);
        // Iterating a collection is always the safer option because it verifies
        // the borrowing rules.
    }

    println!();

    // Strings
    {
        println!("[Strings]");
        // Strings are collections of bytes
        // To concatenate an string using the + operator, the first value will take ownership
        // and no longer exists
        let string_1 = String::from("Init");
        let string_2 = " System";
        let result_str = string_1 + string_2;
        println!(
            "The result of adding up the two strings are: {}",
            result_str
        );
        // But using format, a new string will be returned and all the others will still exists
        println!(
            "Formatting a string using format!: {}",
            format!("{}-{}-{}", 11, 07, 2024)
        );

        // Indexing to get a piece of a string will cause compiler error. This happens because
        // the size of each letter will depend on codification
        // To get a slice of a string (substring) the slice syntax should be used (still can break the program)
        println!("slicing the result_str[0..1]: {}", &result_str[0..1]);

        // The best way to get pieces of a string is calling the .chars() method
        // This method really separates each letter in a collection of chars
    }

    println!();

    // HashMaps
    {
        println!("[Hash Maps]");
        // Hash maps stores a mapping of keys and values, both can assume any data type.
        // Using this allows to find a value by a certain key, instead of an index, like vectors
        // All keys must have the same data type, and all values must have their data type
        let mut my_hash_map = HashMap::new();
        my_hash_map.insert("opa", 1);
        my_hash_map.insert("Tasi", 28);
        println!("my_hash_map: {:?}", my_hash_map);

        // To access a value from the hashmap, just use the key of the value
        // Each key appears only once in a hash map. If a insert has a key that
        // already exists, it's value is updated.
        // Using inline handling of Option
        println!(
            "Value from key Tasi: {}",
            my_hash_map.get("Tasi").copied().unwrap_or(-1)
        );
        println!(
            "Value from key that doesn't exists: {}",
            my_hash_map.get("rust").copied().unwrap_or(-1)
        );

        // Like every collection, hashmap also can be iterate over
        println!("Printing HashMap on iterator for-each");
        for (key, value) in &my_hash_map {
            println!("From key [{}], value [{}]", key, value);
        }

        // If a data type has the Copy trait implemented, the value is copied to the hashmap when inserter
        // To complex data types, like string, the ownership will move to the hashmap
    }
}

fn print_returned_value(result: Option<&i32>) {
    match result {
        Some(value) => println!("Got a value: {value}"),
        None => println!("Got no value. Index not existent"),
    }
}
