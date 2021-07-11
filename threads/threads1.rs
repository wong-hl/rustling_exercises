// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// Want interior mutability => need mutex
// Refer to ch 16-03
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    // This field is to be updated so interior mutability is required
    // Hence, Mutex
    jobs_completed: Mutex<u32>,
}

// NOTE: https://erasin.wang/books/easy-rust/Chapter_42.html
// This reference was useful in understanding what Mutex is and how to use it

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: Mutex::new(0),
    });
    // Instead of clone, Arc::clone should be used insead.
    let status_shared = Arc::clone(&status);

    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));

            // To get the mutex, the Arc must be locked and unwrapped
            // This returns a MutexGuard
            let mut num_jobs_completed = status_shared.jobs_completed.lock().unwrap();

            // As it is a MutexGuard, * is used to change it to u32
            *num_jobs_completed += 1;
        }
    });

    // For the same reasons as above, *, lock() and unwrap() have been used.
    while *status.jobs_completed.lock().unwrap() < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
