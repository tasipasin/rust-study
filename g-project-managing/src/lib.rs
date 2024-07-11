// development module is parent of front_end and back_end

// All items are private to parent modules by default. This means the parent module
// can't access their children items, but the children can use parent's things.

// development module is define inside curlybrackets like said previously
mod development {
    // front_end module is nested inside development.
    // front_end and back_end are siblings bacause they're nested inside development.
    pub mod front_end {
        pub fn shows_to_user() {
            println!("Called shows_to_user()");
        }

        pub fn get_user_input() {
            println!("Called get_user_input()");
        }
    }

    pub mod back_end {
        pub fn process_data() {}

        pub fn persistence_data() {}
    }
}

// The use keyworkd is a shortcut to don't need to write all the long path
use development::front_end;

// display_screen and development are siblings, so they can see each other. That's why there's no problem
// letting front_end and shows_to_user public. It'll only be visible inside this scope. Of any other
// scope includes this lib, will see only display_screen() function.
pub fn display_screen() {
    // They're both do the same, it's a project decision to whether use one or another
    // Absolute path
    crate::development::front_end::shows_to_user();

    // Relative path
    development::front_end::shows_to_user();

    // Shortcut path using the keyword use
    front_end::get_user_input();

    // Only to remove warnings
    front_end_programmer::do_front_end();
    back_end_programmer::do_back_end();
}

mod front_end_programmer {
    pub fn do_front_end() {
        // This line won't compile, case the use is at an outside scope
        // front_end::shows_to_user();
    }
}

mod back_end_programmer {
    // This use is in the same scope
    use super::development::back_end;

    // The use can also referentiate another use and to a function
    // Referencing directly to a function is called idiomatic way but can cause confusion.
    use back_end::persistence_data;

    pub fn do_back_end() {
        back_end::process_data();

        // At the use of a idiomatic referenced function, it looks like a local function
        // rather than an outside module function.
        persistence_data();
    }
}
