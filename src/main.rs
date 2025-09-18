use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use std::time::Duration;
//use std::{sync::{Arc, Mutex}, thread} // this is for Arc<Mutex<T>> + scoped threads

fn main() {
//     println!("Demonstrating Fearless concurrency with closures and Memory Safety in Rust using Arc, Mutex");

//     let shared_results = Arc::new(Mutex::new(vec![]));
//     println!("Initial shared_results: {:?}", shared_results.lock().unwrap());

//     let numbers_to_process = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     let mut handles = vec![];

//     for number in numbers_to_process {
//      // clone Arc for each thread:
//      // Each thread needs its own Arc pointer to the shared data.
//      let results_clone = Arc::clone(&shared_results);
        
//      /*   Spawn a new thread for each number:
//         the closure 'passed' to spawn runs in the new thread.
//         'move' keyword is key: it moves 'results_clone' into the closure's environment,
//         ..tranferring ownership to the new thread = Rust's ownership rules are then satisfied.
//      */

//        let handle = thread::spawn(move || {
//           println!("[Thread for {}] Processing number...", number);
//           thread::sleep(Duration::from_millis(50 * number as u64));

//        let result = number * 2;

//         // Acquire Mutex Lock:
//         // this is where memory safety is enforced at runtime for shared mutable state.
//          // Only one thread can hold the lock at a time, preventing data races.

//           let mut results_guard = results_clone.lock().unwrap();
//           results_guard.push((number, result)); // safely modify this shared vector
//           println!("[Thread for {}] Added ({}, {}) to results", number, number, result);

//         // The lock is automatically released when 'results_guard' goes out of scope.

//      });
//        handles.push(handle);

    // }  

//   //Wait for all threads to complete:
//        for handle in handles {
//           handle.join().unwrap();    
//     }

    
//        //Access final results:
//        //Acquire the lock one last time to read the final state.
//        let final_results = shared_results.lock().unwrap();
//        for (input, output) in final_results.iter() {
//             println!("{} double is {}", input, output);
       //}
       
     // - ?? DEMONSTRATE a compile time error , therefore explain why its prevented.
     //   if you uncomment this line

       /* 

       let outside_variable = 50;
       let handle_error = thread::spawn(|| {
            // this would cause a compile-time error bcs 'outside_variable' is borrowed by the closure
            // but the closure is moved to a new thread,
            // and 'outside_variable' might go out of scope before the thread finishes.
            // Rust's ownership and lifetime rules prevent this use-after-free scenario.
            println!("Accessing outside_variable: {}", outside_variable);
       });
          */

       //println!("\nRust prevented a potential use-after-error (see commented code)");
//         println!("This is the core aspect of Rust's compile time memory for concurrency.");



//      // Achieving Fearless concurrency with Rust closures, scoped thread with spawn, AND achieving memory safety:

// //      fn main() {
//      //Create a vector of numbers to process

//      let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//      println!("Original data: {:?}", data);

// // Process data in parallel using scoped threads
//      thread::scope(|s| {
//           // Split data into mutable chunks for parallel processing
//           let chunk_size = (data.len() + 2) / 3; // this equals 3 chunks
//           for chunk in data.chunks_mut(chunk_size) {
//           // Spawn thread with closure that captures mutable chunk

//           s.spawn(move || {
//           //  Closure captures chunk by move (exclusive ownership)

//           for num in chunk.iter_mut() { 
//           // Transfer each element 
//           *num *= 2;
//           *num += 1;
//           }
//           // Memory safety: At this point, the closure exclusively owns 'chunk'
//           // No other thread can access this memory region

//           println!("Thread {:?} processed chunk: {:?}",
//                     thread::current().id(), chunk);
//           });

//           }
//      }); // All threads are joined here, automatically

//          println!("Transformed data: {:?}", data);

     

//     // SCOPE from the lib
//      let mut a = vec![1, 2, 3];
//      let mut x = 0;

//       thread::scope(|s| {
//           s.spawn(|| {
//                println!("hello, Dolly, this is Rust, Dolly, i am glad to have you back where you belong from the Std Library");
//                // We can borrow 'a' here.
//                dbg!(&a);
               
//           });
//           s.spawn(|| {
//                println!("You are looking swell, Dolly, I can tell Dolly, you keep crowing, you keep blowing, from the second scoped thread");
//                // we can even mutably borrow 'x' here,
//                // bcs no other threads are using it.
               
//                x += a[0] + a[2];
//                dbg!(&x);
//           });
//               println!("You keep going, you are still going strong, i can see the room swaying, and the band still playing, from the main thread");
//               println!("This is Satchmo Dolly, you keep blowing, you keep going, you are looking swell Dolly, Yeah we play the old favourite tunes, from way back..")
//      });

//     // After the scope, we can modify and access our variables again
//        a.push(4);
//        assert_eq!(x, a.len());



     // LIFETIMES:

     // Scoped threads involve TWO lifetimes: 'scope and 'env
     // The 'scope LTime represents the lifetime of the scope itself
     // The 'env LTime represents the lifetime of whatever is borrowed by the scoped threads.
     // The 'env: 'scope bound is part of the definition of the Scope type. 


// BONUS on Chunks with a REMAINDER - mitigation

// A. Pre-check that the data divides evenly:

// let chunk_size = 3;
// if data.len() % chunk_size != 0 {
//      panic!("Data size is not evenly divisible by chunk size");

// }

// // B. Use .chunks_exact() & handle the remander explicitly:

// let mut iter = data.chunks_exact(3);
// for chunk in iter.by_ref() {
//      // spawn thread ......
// }
// let remainder = iter.remainder();
// if remainder.is_empty() {
//      // handle separately or discard
// }

// fn main() {

     let counter = Arc::new(Mutex::new(0));

     thread::scope(|s| {
          for _ in 0..3 {
               let counter = Arc::clone(&counter);
               s.spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                    println!("Thread incremented: {}", *num);
               });
          }
     });
          println!("Final Counter: {}", *counter.lock().unwrap());
//}

// SHARED STATE - concurrency - Google (Arc and Mutex)

// Arc<T> - allows shared, read-only ownership via Arc::clone

use std::sync::{Arc, Mutex};
use std::thread;


// A struct that prints which thread drops it
#[derive(Debug)]
struct WhereDropped(Vec<i32>);

impl Drop for WhereDropped {
     fn drop(&mut self) {
          println!("Dropped by {:?}", thread::current().id())
     }
}

//fn main() {
     let v = Arc::new(WhereDropped(vec![10, 20, 30]));
     let mut handles = Vec::new();
     for i in 0..5 {
          let v = Arc::clone(&v);
          handles.push(thread::spawn(move || {
               // Sleep for 0-500ms.
               std::thread::sleep(std::time::Duration::from_millis(500 - i * 100));
               let thread_id = thread::current().id();
               println!("{thread_id:?} {v:?}");
          }));
     }

     // Now only the spawned threads will hold clones of 'v'
     drop(v);

     // When the last spawned thread finishes, it will drop 'v's contents
     handles.into_iter().for_each(|h| h.join().unwrap());
//}

// Mutex - Mutex<T> ensures mutual exclusion and allows mutable access to T behind a read-only interface(another form of interior mutability)

// fn main() {
     let v = Mutex::new(vec![100, 200, 300]);
     println!("v: {:?}", v.lock().unwrap());

     {
          let mut guard = v.lock().unwrap();
          guard.push(40);
     }

     println!("v: {:?}", v.lock().unwrap());     

     // Mutex<T> implements both Send and Sync if and only if T implements Send.
     // A read-write lock counterpart is: RwLock
     // Why does lock() return a Result?
     // - If the thread that held Mutex panicked, the Mutex becomes 'POISONED' to signal that the data if protected might be in an inconsistent state
     // Calling lock() on a poisoned mutex fails with a "PoisonError"
     // YOu can call into_inner() on the error to recover the data regardless

     // Struct PoisonError - a type of error that can be returned whenever a lock is acquired
     // Both "Mutex" and "RwLock" are poisoned whenever a thread fails while the lock is held.For a lock in a poisoned state, unless the state is cleared manually, all future acquisitions will return this error

// IMPLEMENTATIONS:

// 1.

let mutex = Arc::new(Mutex::new(1));
// Mutex::new(1) creates a mutex that initially holds an int value 1
// Arc allows this mutex to be shared across threads

// 2. POISONING the mutex

let c_mutex = mutex.clone();
let _ = thread::spawn(move || {
     let mut data = c_mutex.lock().unwrap();
     *data = 2;
     panic!();     
}).join();
    
// A new thread is spawned, and it clones the Arc, so both main and worker threads share tyhe same mutex.
// Inside the spawned thread:

/*   - c_mutex.lock() acquires the lock
     - unwrap() succeeds bcs the mutex is NOT poisoned yet.
     - the value inside is changed from 1 -> 2.
     - the panic!() happens while the mutex is still locked.

     * this is where the poisoning occurs:
     When a thread panics while holding a mutex lock, Rust's Mutex marks the mutex as 'poisoned'. 
     This is a safeguard, bcs data might be in an inconsistent state after a panic.

*/

// 3. Main thread tries to lock again

match mutex.lock() {
     Ok(_) => unreachable!(),
     Err(p_err) => {
          let data = p_err.get_ref();
          println!("recovered: {}", data);
     }
};

/*
mutex.lock() sees the mutex is poisoned
instead of returning Ok(MutexGuard<T>), it returns Err(PoisonError<MutexGuard<T>>).

This PoisonError still gives you access to the data inside the mutex. You can get at it by calling:
- .into_inner() -> consumes the error and get the guard.
- .get_ref() -> borrows a reference

NB: in this code, 'p_err.get_ref()' returns &2 (since the worker thread wrote 2 before panicking) and so Rust prints 'recovered: 2' 
 */

//}


// HEAP IMPLEMENTATION of the PoisonError with a has data structure

use std::collections::HashSet;

let mutex = Arc::new(Mutex::new(HashSet::new()));
// inside the mutex is a HashSet, initially empty ({}).
// Worker thread then locks then does this...'data.insert(10);'
// Now the set = {10}.
// Then panic!() happens -> lock is dropped, mutex poisoned.

// poison the mutex
let c_mutex = mutex.clone();
let _ = thread::spawn(move || {
     let mut data = c_mutex.lock().unwrap();
     data.insert(10);
     panic!();
}).join();


// Main thread:
let p_err = mutex.lock().unwrap_err();
let data = p_err.into_inner();

println!("recovered {} items", data.len());

/* 
- unwrap_err() unwraps the Err(PisonError).
- .into_inner() consumes the error n gives back the guard (instead of just the reference).
- We now have full ownership of the poisoned data (HashSet{10}).
- data.len() = 1.
- Prints: 

.......recovered 1 items


What MIRI would see in this code?

1. Mutex creation

- Arc::new(Mutex::new(HashSet::new())) -> safe, heap alloc for HashSet

2. Worker thread lock

- Miri checks: lock acquired, no race
- Guard returned safely

3. Mutation

- data.insert(10) mutably borrows the set
- Miri checks borrowing rules -> all good
- HashSet internally modifies its buckets (safe, no UB in Rust).

4. Panic!()
- Panic unwinds while guards is held.
- Guard's destructor release the lock
- Mutex poison flag is set
- Miri validates that the lock release is correct

5. Main thread lock attempt

- Miri sees poisoned flag -> returns 1.
- .into_inner() transfers ownership of the guard back to the caller
- Miri confirms this is valid (data structure is intact)

6. Length check

- Safe read of HashSet length -> returns 1
- Printed safely

# Miri Verdic: No UB. Safe, deterministic poisoning + recovery

*/


     

     // ASYNCRONOUS prog
     // Async, Await, Futures for Rust

     // 1. Future Trait - represents an asyncronous computation that may complete later
     // - defined in std ::future (or futures crate for extended functionality)
     // - Output is retrieved via Poll (either Ready(T) or Pending)

     // use std::future::Future;
     // use std::pin::Pin;
     // use std::task::{Context, Poll};

     // struct MyFuture {
     //      completed: bool,
     // }

     // impl Future for MyFuture {
     //      type Output = i32;

     //      fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
     //           if self.completed {
     //                Poll::Ready(42)
     //           } else {
     //                self.completed = true;
     //                Poll::Pending
     //           }
     //      }
     // }

     // 2. async/await Syntax
     // - async: Transforms a block / code into a state machine that implements Future.
     // - await: Suspends execution until a Future completes (without blocking the thread)
     
     // E.g

     // async fn fetch_data() -> i32 {
     //      // Simulate async work (e.g, network call)
     //      42
     // }

     // async fn process() {
     //      let data = fetch_data().await; //Pause until fetch_data completes
     //      println!("Data: {}", data);
     // }

     // 3 Executors - asyncronous code requires an executor to drive Futures to completion...=> tokio or async-std API

     // #[tokio::main]
     // async fn() {
     //      process().await;
     // }

     // 4. Streams - similar to iterator, but yields values asyncronously
     // - defined in futures::stream::Stream (or tokio_stream).
     // - use next().await to fetch the next item.

     // CREATING A Stream

     // use futures::stream::{self, StreamExt}; // Enable 'next()'

     // async fn count_stream() -> impl Stream<Item = i32> {
     //      stream::iter(1..=5) // Syncronous iterator -> Stream
     // }

     // #[tokio::main]
     // async fn main() {
     //      let mut stream = count_stream().await;
     //      while let Some(num) = stream.next().await {
     //      println!("Got: {}", num);

     //      }
     // }

     // // Async Stream with async - stream:

     // use async_stream::stream;
     // use tokio_stream::Stream;

     // fn async_counter() -> impl Stream<Item = i32 {
     //      stream! {
     //           for i in 0..5 {
     //                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
     //                yield i;
     //           }
     //      }
     //}
     
     

     



}
