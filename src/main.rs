use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
//     println!("Demonstrating Fearless concurrency with closures and Memory Safety in Rust using Arc, Mutex");

//     let shared_results = Arc::new(Mutex::new(vec![]));
//     println!("Initial shared_results: {:?}", shared_results.lock().unwrap());

//     let numbers_to_process = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let mut handles = vec![];

//     for number in numbers_to_process {
//         // clone Arc for each thread:
//         // Each thread needs its own Arc pointer to the shared data.
//         let results_clone = Arc::clone(&shared_results);
        
//         // Spawn a new thread for each number:
//        // the closure 'passed' to spawn runs in the new thread.
//        // 'move' keyword is key: it moves 'results_clone' into the closure's environment,
//        // ..tranferring ownership to the new thread = Rust's ownership then are satisfied.

//        let handle = thread::spawn(move || {
//             println!("[Thread for {}] Processing number...", number);
//             thread::sleep(Duration::from_millis(50 * number as u64));

//             let result = number * 2;

//         // Acquire Mutex Lock:
//         // this is where memory safety is enforced at runtime for shared mutable state.
//         // Only one thread can hold the lock at a time, preventing data races.

//             let mut results_guard = results_clone.lock().unwrap();
//             results_guard.push((number, result)); // safely modify this shared vector
//             println!("[Thread for {}] Added ({}, {}) to results", number, number, result);

//         // The lock is automatically released when 'results_guard' goes out of scope.

//        });
//        handles.push(handle);

//        }  

//      //  Wait for all threads to complete:
//        for handle in handles {
//             handle.join().unwrap();    
//     }

    
//        //Access final results:
//        //Acquire the lock one last time to read the final state.
//        let final_results = shared_results.lock().unwrap();
//        for (input, output) in final_results.iter() {
//             println!("{} double is {}", input, output);
//        }
       
       // - ?? DEMONSTRATE a compile time error , therefore explain why its prevented.
       // if y9u uncomment this line

       /* 
       let outside_variable = 50;
       let handle_error = thread::spawn(|| {
            // this would cause a compile-time error bcs 'outside_variable' is borrowed by the closure
            // but the closure is moved to a new thread,
            // and 'outside_variable' might go out of scope before the thread finishes.
            // Rust's ownership and lifetime rules prevent this use-after-free scenario.
            println!("Accessing outside_variable: {}" outside_variable);
       });
       */    
       println!("\nRust prevented a potential use-after-error (see commented code)");
       println!("This is the core aspect of Rust's compile time memory for concurrency.");



     // Achieving Fearless concurrency with Rust closures, scoped thread with spawn, AND achieving memory safety:

     // fn main() {
     // Create a vector of numbers to process

     // let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
     // println!("Original data: {:?}", data);

     // // Process data in parallel using scoped threads
     // thread::scope(|s| {
     //      // Split data into mutable chunks for parallel processing
     //      let chunk_size = (data.len() + 2) / 3; // 3 chunks
     //      for chunk in data.chunks_mut(chunk_size) {
     //      // Spawn thread with closure that captures mutable chunk

     //      s.spawn(move || {
     //      // Closure captures chunk by move (exclusive ownership)

     //      for num in chunk.iter_mut() {
     //      // Transfer each element 
     //      *num *= 2;
     //      *num += 1;
     //      }
     //      // Memory safety: At this point, the closure exclusively owns 'chunk'
     //      // No other thread can access this memory region

     //      println!("Thread {:?} processed chunk: {:?}",
     //                thread::current().id(), chunk);
     //      });

     //      }
     // }); // All threads are joined here, automatically

     // println!("Transformed data: {:?}", data);

     //}

     // SCOPE from the lib
     let mut a = vec![1, 2, 3];
     let mut x = 0;

     thread::scope(|s| {
          s.spawn(|| {
               println!("hello, Dolly, this is Rust, Dolly, i am glad to have you back, where you belong from the Std Library");
               // We can borrow 'a' here.
               dbg!(&a);
          });
          s.spawn(|| {
               println!("You are looking swell, Dolly, I can tell Dolly, you keep blowing, from the second scoped thread");
               // we can even mutably borrow 'x' here,
               // bcs no other threads are using it.
               
               x += a[0] + a[2];
          });
          println!("You keep going, you are still going strong, i can see the room swaying, and the band still playing, from the main thread");
          println!("This is Satchmo Dolly, you keep blowing, you keep going, you are looking swell Dolly, Yeah we play the old favourite tunes, from way back..")
     });

     // After the scope, we can modify and access our variables again
     a.push(4);
     assert_eq!(x, a.len());


     // LIFETIMES:

     // Scoped threads involve TWO lifetimes: 'scope and 'env
     // The 'scope LTime represents the lifetime of the scope itself
     // The 'env LTime reprsents the lifetime of whatever is borrowed by the scoped threads.
     // The 'env: 'scope bound is part of the definition of the Scope type. 
}
