use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    // Create 2 producer threads
    let mut producer_handles = Vec::with_capacity(NUM_PRODUCERS);
    for id in 0..NUM_PRODUCERS {
        let producer_tx = tx.clone();
        // Each producer will generate half of the total items
        let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;
        let handle = thread::spawn(move || {
            producer(id, producer_tx, items_per_producer);
        });
        producer_handles.push(handle);
    }

    // Create 3 consumer threads
    let mut consumer_handles = Vec::with_capacity(NUM_CONSUMERS);
    for id in 0..NUM_CONSUMERS {
        let consumer_rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, consumer_rx);
        });
        consumer_handles.push(handle);
    }

    // Wait for all producer threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    println!("All producers have finished. Sending termination signals to consumers...");

    // Send termination signals to all consumers
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumer threads to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    println!("Producer {} starting", id);

    for i in 0..item_count {
        // Generate a random number between 1 and 100
        let number = rng.gen_range(1..101);

        // Send the number to the channel
        tx.send(number).unwrap();

        println!(
            "Producer {} produced: {} (item {}/{})",
            id,
            number,
            i + 1,
            item_count
        );

        // Sleep for a random duration to simulate work
        let sleep_duration = rng.gen_range(50..200);
        thread::sleep(Duration::from_millis(sleep_duration));
    }

    println!("Producer {} finished producing {} items", id, item_count);
}

// Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    println!("Consumer {} starting", id);

    loop {
        // Lock the mutex to access the receiver
        let value = {
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };

        // Check if this is the termination signal
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }

        // Process the received value (in this case, just display it)
        println!("Consumer {} processed: {}", id, value);

        // Sleep for a short duration to simulate processing time
        thread::sleep(Duration::from_millis(100));
    }

    println!("Consumer {} shutting down", id);
}
