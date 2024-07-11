// Absolute path to a module bringing the struct itself
use crate::module::submodule::Newmodule;
// Absolute path to HashMap
use std::collections::HashMap;

// This set of declarations will cause problem cause the compiler won't know which Resulte is being used
// This could be used if one of them had an alias, using the 'as' keyword
// use std::fmt::Result;
// use std::io::Result;

// This set will be ok, event using the Result, cause will be needed to specify,
// in use, which module is being used
use std::fmt;
use std::io;

// This tells the compiler to include module (not the whole module structure)
pub mod module;

fn main() {
    // Declaring and using Modules and Submodules
    {
        println!("=== Declaring and using Modules and Submodules ===");
        // With all the declaration, the struct Newmodule can now be used
        // But since there's a private property in Newmodule, it can't be
        // initialized like this
        // let newmodule = Newmodule{name: String::from("Tasi")};

        // Using this way, Newmodule can, now, be initialized.
        let newmodule = Newmodule::init("My Module", 2024);
        println!("newmodule data: {:?}", newmodule);
        println!("newmodule age: {}", newmodule.get_age());

        // Next line will cause a compiler error, since get_year is private
        // newmodule.get_year();

        let mut map = HashMap::new();
        map.insert("Tasi", 85);
        println!("map: {:?}", map);
    }
    // Prevent warnings
    println!("{:?}", func1());
    println!("{:?}", func2());
}

fn func1() -> fmt::Result {
    Ok(())
}

fn func2() -> io::Result<()> {
    Ok(())
}
