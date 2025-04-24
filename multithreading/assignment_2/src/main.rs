use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a shared counter using Arc and Mutex, initialized to 0
    let counter = Arc::new(Mutex::new(0u32)); 
    
    // Create a vector to store thread handles
    let mut handles = vec![];
    
    // Spawn 5 threads
    for _ in 1..=5 {
        // Clone the Arc for the thread. 
        let counter_clone = Arc::clone(&counter);
        
        // Spawn a thread that increments the counter 10 times
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter_clone.lock().unwrap(); 

                *num += 1; 
            }
        });
        
        handles.push(handle); 
    }
    
    // Wait for all threads to complete
    println!("Main thread waiting for counter threads...");
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print the final value of the counter
    let final_value = *counter.lock().unwrap();
    println!("Final counter value: {}", final_value); 
    // Expected output: Final counter value: 50 (5 threads * 10 increments each)
}
