use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

#[derive(Debug)]
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // The unwrap_or_else function takes one argument, a closure
        // This closure has no arguments and returns a value T, the same as
        // the Option variable stores.
        // If the user_preference is Some, it will return it's content
        // If the user_preference is None, it will run the code passed as closure
        // The closure parameters must be define between the two vertical bars
        // Since no parameter is required for the closure, no parameter is defined
        // The closure automatically captures an immutable reference from to &self
        // Closures also doesn't require to define the data types of the paramaters and return
        // Function DO, because it can be part of an interface exposed, so who
        // will use it must know what kind of types are going in and what's going out
        // Closures are short and specific to the context it's been inserted
        user_preference.unwrap_or_else(|| self.most_stocked())

        // This line uses explicit datatype on closure definition
        // It also works, but it gets more verbose
        // user_preference.unwrap_or_else(|| -> ShirtColor {self.most_stocked()})
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    {
        // Closures are functions stored in variables that can be passed as
        // arguments to other functions
        // Closures can capture values from the scope in which they're defined
        println!("=== Closures ===");
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );

        // Definition of a closure into a variable, from verbose to less verbose
        let mut start = 0;
        let my_closure = |num: u32| -> u32 { num + 1 };
        start = my_closure(start);
        println!("my_closure 1: {}", start);

        let my_closure = |num: u32| num + 1;
        start = my_closure(start);
        println!("my_closure 2: {}", start);

        let my_closure = |num| num + 1;
        start = my_closure(start);
        println!("my_closure 3: {}", start);

        let my_closure = |num| num + 1;
        start = my_closure(start);
        println!("my_closure 4: {}", start);

        let multiple_type_closure = |value| format!("Received value :{}", value);
        let value = "Tasi";
        println!(
            "multiple_type_closure for string [{}]: {}",
            value,
            multiple_type_closure(value)
        );

        // A closure has the type defined, so if it has a value but the type is infered by the compiler
        // the type will be the same as the first use, it means since the multiple_type_closure was
        // used passing a string as parameter, it will work only for strings.
        // The next line doesn't compile for that reason. But if those lines are exchanged, the
        // line using integer will work and the one using string won't
        // println!("multiple_type_closure for int [{}]: {}", 55, multiple_type_closure(55));

        println!();
        println!("[Moving Ownership]");
        println!(".Immutable Borrow.");

        let mut list = vec![1, 2, 3];
        println!("Before closure definition: {:?}", list);

        // Borrows can reference the list because, like said before, a closure can capture
        // a immutable borrowed reference from the scope it is inserted
        let only_borrows = || println!("Inside only_borrows: {:?}", list);

        println!("After closure definition; Before closure usage: {:?}", list);

        // No arguments
        only_borrows();

        // And since all the uses of list are immutable borrowed, it can be used
        // anywhere over and over
        println!("After closure usage: {:?}", list);

        println!();
        println!(".Mutable Borrow.");

        // The closure captures a mutable borrow of list
        let mut change_value = || list.push(7);
        println!("There's no usage of list between the mutable borrow and the closure use");

        // This line won't compile, because it's changing the list while there's a mutable
        // borrow pending
        // list.push(8);

        // When the closure is called, the mutable borrow ends
        change_value();
        change_value();

        // Here it works, because the mutable borrow ended
        list.push(8);
        // It's using the list as immutable borrow
        println!("After change_value definition: {:?}", list);

        println!();
        println!(".Moving to Inside.");

        // This is a new thread and the closure is what have inside the run() function
        // After run, it'll die and everything cleared
        // The println! macro only needs the immutable borrow to work, so list would be
        // passed as so. But since there's the keyword move at the beginning of the closure
        // definition, the closure, now, owns it and list is no more accessible from this context.
        // Also, as it's running in other thread, for this case, the move keyword is required
        // because it's unknown which one of them will finish first, so the reference might be
        // invalid when the new thread runs.
        thread::spawn(move || println!("Took the list: {:?}", list))
            .join()
            .unwrap();
    }

    println!();

    // Iterators
    {
        // Iterators pass through each item and determinates when the sequence has finished.
        println!("=== Iterators ===");

        // This is a vector
        let vec_1 = vec![2, 4, 8];

        // This is the vector iterator
        // It does nothing by itself
        let vec_1_it = vec_1.iter();
        print!("Printing the vector using iterator explicity: ");
        // But can be used in a for to pass through every item from vec_1
        // Although is not required to extract the iterator to a variable
        // or even call the iter() method, because the for loop does it automatically
        for item in vec_1_it {
            print!("{item} ");
        }
        println!();

        println!();
        println!("[next()]");
        // Gets a new iterator, from the beginning
        let mut vec_1_it = vec_1.iter();
        // The iterator implements the Iterator trait, and it requires only one
        // method to be implemented: the next() method.
        // It returns the next item wrapped in Some
        // When there's no next value, it returns None
        // The next() method consumes the iterator, that's why the variable must be
        // define as mutable
        // The value returned is a immutable reference to the value inside vec_1 in that position
        println!(
            "vec_1_it.next() is equals to Some(&vec_1[0]): {}",
            vec_1_it.next() == Some(&vec_1[0])
        );
        println!(
            "vec_1_it.next() is equals to Some(&vec_1[1]): {}",
            vec_1_it.next() == Some(&vec_1[1])
        );
        println!(
            "vec_1_it.next() is equals to Some(&vec_1[2]): {}",
            vec_1_it.next() == Some(&vec_1[2])
        );
        println!(
            "vec_1_it.next() is equals to None: {}",
            vec_1_it.next() == None
        );

        println!();
        println!("[Consuming the Iterator]");
        println!(".sum(0).");
        // The sum() method takes ownership of the iterator an iterates through the items
        // calling next() method.
        // It sums up all the values and returns that value
        // This uses the old vec_1 iterator, already consumed, so the sum will be 0
        let total: i32 = vec_1_it.sum();
        println!("vec_1 sum() using consumed iterator: {:?}", total);
        let total: i32 = vec_1.iter().sum();
        println!("vec_1 sum() using new iterator: {:?}", total);

        println!();
        println!("[Producing other Iterators]");
        println!(".map().");

        let vec_2 = vec![2, 66, 4, 15, 98, 3, 65, 78, 42, 55];
        println!("vec_2: {:?}", vec_2);
        // The map() method accepts a closure as parameter to evaluate every
        // value inside an iterator and returns another iterator
        // This new iterator must be stored using collect() method
        // The collect() method required an explicit data type
        // The collect() method consumes the new produced iterador and return
        // the values in the pre defined collection data type
        let resulting_vec: Vec<i32> = vec_2.iter().map(|value| value * 2).collect();
        println!("resulting_vec - vec_2 * 2: {:?}", resulting_vec);

        println!();
        println!("[Performance]");
    }
}
