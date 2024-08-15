use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    // Threads
    {
        println!("=== Threads ===");
        // Creates a child thread
        // It will run until the main thread shutdown
        // When multiple threads are running in parallel, there's no
        // guaranteed order for them to run. It will depend on the OS
        // If not handled, the spawned thread might be shutdown by the
        // main thread
        let new_thread = thread::spawn(|| {
            for i in 1..10 {
                println!("Number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(500));
            }
        });

        for i in 1..5 {
            println!("Number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        println!("Main thread ended. Waiting spawned thread");
        // thread::spawn returns a JoinHandle. Using it's join() method,
        // the main thread will stop and wait for the spawned thread to finish.
        // The threads keep alternating, but it's blocked waiting spawned thread.
        new_thread.join().unwrap();
        println!("Everything's ended");

        println!();
        println!("[move Closures]");

        let to_new_thread = vec![1, 2, 3, 4];

        // Closures captures the environment where it's called.
        // But because the compile can't infer how long the spawned thread
        // will run, it doesn't know if the reference to to_new_thread will
        // always be valid.
        // thread::spawn(|| println!("Received a value from main thread: {:?}", to_new_thread)).join().unwrap();

        // Using move for closure will force it to take ownership of the variable
        // This will run, but to_new_thread is no more accessible in the main scope
        thread::spawn(move || println!("Received a value from main thread: {:?}", to_new_thread))
            .join()
            .unwrap();
    }

    println!();

    // Message Passing
    {
        println!("=== Message Passing ===");
        // Message Parsing is the communications between threads by sending
        // each other messages containing data. Share memory by communicating.
        // This process can suffer of concurrency, but Rust provides the channels implementation.
        // Channels, like water channels, are a one way stream. The transmitter half is
        // the upstream. The receiver one is the downstream.

        // A channel is created using the command mpsc::channel()
        // mpsc stands for multiple producer/single consumer.
        // This means a single consumer can receive data from different producers.
        // The mpsc::channel() returns a tuple containing a sending end, the transmitter,
        // and a receiving end, the receiver, respectively.
        let (tx, rx) = mpsc::channel();

        println!("`Spawning new thread and moving tx into it`");
        // The transmitter is moved into the new spawned thread, which means the
        // main thread is the consumer of it's data.
        thread::spawn(move || {
            let val = String::from("hi");
            // The send methods returns a Result, because if the receiver has been
            // already dropped, an Error will be returned
            tx.send(val).unwrap();
            // When a data is sent through the stream, the current thread loses its
            // ownership and passes to the receiver
            // println!("val is: {:?}", val);
        });

        println!("`Receiving data from spawned thread with rx`");
        // The recv() method, like the .join from JoinHandle, will block main thread waiting
        // until something is sent by the stream.
        // It will only return Err when the transmitter closes
        // If the thread receiving has work to do while expects new data,
        // the method try_recv() should be used
        match rx.recv() {
            Ok(received_value) => println!("I've received this: {:?}", received_value),
            Err(_) => println!("Received nothing"),
        }

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let values = vec!["i'm", "sending", "stuffs"];
            for value in values {
                tx.send(value).unwrap();
                // Sends a value per second
                thread::sleep(Duration::from_secs(1));
            }
        });

        // This way treats rx as an iterator. When the channel is closed, the iterator ends.
        for received in rx {
            println!("Received: {:?}", received);
        }

        println!();
        println!("[Cloning the transmiter]");
        let (tx, rx) = mpsc::channel();

        // Creates a clone from tx
        let tx1 = tx.clone();
        println!("Spawns a thread sending each 500 millis");
        // Spawns two threads that sends values from 1 to 9
        // with different times
        thread::spawn(move || {
            for i in 1..10 {
                tx.send(format!("first thread: {}", i)).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
        });

        println!("Spawns a thread sending each 330 millis");
        thread::spawn(move || {
            for i in 1..10 {
                tx1.send(format!("second thread: {}", i)).unwrap();
                thread::sleep(Duration::from_millis(330));
            }
        });

        println!("Reading:");
        for rec in rx {
            println!("Main received: {rec}");
        }
    }

    println!();

    // Shared-State Memory
    {
        println!("=== Shared-State Memory ===");
        // While channels are very useful, once a value is transferred down a channel
        // the owner of the data is also transferred and cannot be used in the old scope.
        // In addition to that, the data has single ownership.
        // Using shared-state memory approach, multiple threads can acess the same
        // memory location at the same time and have access to the data

        // In order to use shared-state memory, a mutex is required
        // Mutex allows only one thread to access some data at any given time.
        // The thread must ask to mutex access to the data, then mutex register there's
        // someone accessing the data so no one else can get access.
        // After the thread ends accessing the data, it should tell the mutex to release.

        // Mutexes are created using the data value.
        // In this example, a mutex is created for a i32 value, Mutex<i32>
        let mutex = Mutex::new(5);
        println!("Mutex before addition: {:?}", mutex);
        // creates a new scope
        {
            // The lock() method is used to acquire the value of the mutex
            // It waits until the current thread can acquire the mutex and get the value
            // If another thread is holding the mutex and panics, lock() will fail
            // The mutex is lost because, no one else would ever be able to get the lock
            let mut exp_mut_num = mutex.lock().unwrap();
            println!("exp_mut_num: {:?}", exp_mut_num);
            // lock() returns a smart pointer called MutexGuard
            // MutexGuard has Drop implemented, where it released the mutex hold by default
            // so the developer won't forget to do it
            *exp_mut_num += 1;
        } // Here, exp_mut_num will go out of scope and the Drop trait implemented will
          // release the mutex hold, so it can be used for another context
        println!("Mutex after addition: {:?}", mutex);

        println!("[Sharing Mutex through threads]");

        // The counter value is under a mutex
        // The mutex is created using the Arc smart pointer to assure multiple ownership
        // It works just like the Rc, but is safe for concurrent situations.
        // When using an std::sync::atomic, thread safety is assured, but it comes
        // with a performance penalty.
        // Mutex is immutable, but provider interior mutability
        let mutex_counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        // Creates 10 threads moving the mutex inside to each one.
        // Each thread holds mutex value and adds one in the counter.
        // Once the thread finishes, the mutex hold dies
        for index in 0..10 {
            // The mutex, under Arc, is clonned
            let clonned_mutex_counter = Arc::clone(&mutex_counter);
            handles.push(thread::spawn(move || {
                // thread::sleep(Duration::from_millis((1 * 2) ^ index) / (3 * 2));
                println!("The thread {} is running", index);
                let mut inner_thread_mutex = clonned_mutex_counter.lock().unwrap();
                println!("The thread {} is holding mutex", index);
                *inner_thread_mutex += 1;
            }));
        }

        for handle in handles {
            // Waits all the threads to fully run
            handle.join().unwrap();
        }

        println!(
            "mutex_counter after all running threads: {:?}",
            mutex_counter.lock().unwrap()
        );
    }

    println!();

    // Sync and Send
    {
        println!("=== Sync and Send ===");

        // The Send marker trait from std::marker indicates that ownership
        // values can be transferred between threads. It can be used for almost
        // every Rust type. An exception is the Rc smart point because, if it
        // uses Send and clones it, then there will be two owners updating the count.
        // If any compose type has only Send types, so the set is

        // The Sync marker indicates that it is safe to be REFERENCED from multiple threads.
        // It means that any type is Sync if an immutable referente to this type is Send
    }
}
