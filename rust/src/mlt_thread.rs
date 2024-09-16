use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, spawn};
use std::time::Duration;

fn mult_thred_fn() {
    let handle = thread::spawn(|| {
        for _ in 0..5 {
            println!("{}", "Hello from spawned threads");
            thread::sleep(Duration::from_millis(500));
        }
    });
    for _ in 0..5 {
        println!("{}", "hello from main thread");
        thread::sleep(Duration::from_millis(300));
    }
    let _ = handle.join().unwrap();
}

fn vec_fn() {
    let x = 1;
    {
        let v = vec![1, 3, 4, 7];
        thread::spawn(move || {
            println!("{:?}", v);
        });
    }
    println!("{}", x)
}

fn recieve_msg() {
    let (tx, rx) = mpsc::channel();
    spawn(move || {
        tx.send("Hello Ajay from some thread".to_string()).unwrap();
    });
    let val = rx.recv();
    match val {
        Ok(val) => println!("{}", val),
        Err(err) => println!("Failed to recieveMsg: {}", err),
    }
}

//*******Sharing Data Between Threads: Arc and Mutex********//

fn data_share() {
    let counter = Arc::new(Mutex::new(0)); // Shared counter
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc to share ownership
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the mutex to get exclusive access
            *num += 1; // Increment the counter
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}

fn high_cal() {
    let num_threads = 4; // Number of threads for a 4-core machine
    let chunk_size = 100_000 / num_threads; // Split the range for each thread
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size + 1;
        let end = (i + 1) * chunk_size;

        let handle = thread::spawn(move || {
            let sum: u64 = (start..=end).sum();
            sum
        });

        handles.push(handle);
    }

    let mut total_sum = 0;

    // Collect results from all threads
    for handle in handles {
        total_sum += handle.join().unwrap();
    }

    println!("The sum from 1 to 100,000 is: {}", total_sum);
}

//******** usig MPSC ********//

fn mpsc_cal() {
    let num_threads = 4; // Number of threads for a 4-core machine
    let chunk_size = 100_000 / num_threads;

    // Create a channel for communication between producers and the consumer
    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];

    // Spawn multiple producer threads
    for i in 0..num_threads {
        let tx = tx.clone(); // Clone the transmitter for each producer thread
        let start = i * chunk_size + 1;
        let end = (i + 1) * chunk_size;

        let handle = thread::spawn(move || {
            let sum: u64 = (start..=end).sum();
            tx.send(sum).unwrap(); // Send the result to the consumer thread
        });

        handles.push(handle);
    }

    // Drop the original transmitter to ensure all messages are sent
    drop(tx);

    // Consumer thread to receive and aggregate results
    let total_sum: u64 = rx
        .iter()
        .take(num_threads as usize) // Take results from all threads
        .map(|x| x) // Unwrap the Result<u64, _>
        .sum(); // Sum the received values

    // Wait for all producer threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("The sum from 1 to 100,000 is: {}", total_sum);
}
// **************Here is the full explaination *****************//
// ************Explanation by AI:****************//
//************MPSC Channel:

//We create a channel using mpsc::channel(). The producers (worker threads)
//will send their partial sums via the tx (transmitter), and the consumer
//(main thread) will receive the partial sums via the rx (receiver).
//*************Cloning the Transmitter:

//For each producer thread, we clone the transmitter tx so that each thread
//can send its result to the consumer.
//*************Sending Results:

//Each thread computes the sum for its portion of the range
//(from start to end) and sends it through the tx channel using tx.send(sum).unwrap().
//Dropping the Transmitter:

//We drop the original tx after spawning the threads. This is necessary because
//the consumer will only finish when all transmitters are done sending.
//*************Consumer Thread:

//The main thread (acting as the consumer) aggregates the results
//by iterating over the received values with rx.iter() and summing
//them up. The .take(num_threads) ensures that we only consume
//as many messages as there are producers.

pub fn main() {
    mult_thred_fn();
    data_share();
    vec_fn();
    recieve_msg();
    high_cal();
    mpsc_cal();
}
