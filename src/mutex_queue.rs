use std::{
    sync::{Arc, Mutex},
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let clone_a = Arc::clone(&mutex);
    let clone_b = Arc::clone(&mutex);
    let clone_c = Arc::clone(&mutex);

    let thread_a = spawn(move || {
        let mut guard = clone_a.lock().unwrap();
        *guard += 1;
        println!("Thread A: {}", *guard);
        sleep(Duration::from_secs(2));
    });

    let thread_b = spawn(move || {
        let mut guard = clone_b.lock().unwrap();
        *guard += 1;
        println!("Thread B: {}", *guard);
        sleep(Duration::from_secs(2));
    });

    let thread_c = spawn(move || {
        let mut guard = clone_c.lock().unwrap();
        *guard += 1;
        println!("Thread C: {}", *guard);
        sleep(Duration::from_secs(2));
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}
