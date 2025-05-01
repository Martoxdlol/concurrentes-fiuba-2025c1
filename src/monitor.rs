use std::sync::{Arc, Condvar, Mutex};

pub struct Monitor<T> {
    lock: Mutex<T>,
    condvar: Condvar,
}

impl<T> Monitor<T> {
    pub fn new(value: T) -> Self {
        Monitor {
            lock: Mutex::new(value),
            condvar: Condvar::new(),
        }
    }

    pub fn enter<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.lock.lock().unwrap();
        f(&mut guard)
    }

    pub fn enter_and_signal<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.lock.lock().unwrap();
        let result = f(&mut guard);
        self.condvar.notify_one();
        result
    }

    pub fn enter_and_signal_all<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.lock.lock().unwrap();
        let result = f(&mut guard);
        self.condvar.notify_all();
        result
    }

    pub fn wait<F>(&self, mut predicate: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mut guard = self.lock.lock().unwrap();
        while !predicate(&mut guard) {
            guard = self.condvar.wait(guard).unwrap();
        }
    }
}

fn main() {
    // Example usage
    let monitor = Arc::new(Monitor::new(0));

    let monitor_clone = monitor.clone();
    std::thread::spawn(move || {
        monitor_clone.wait(|x| *x == 10);
        monitor_clone.enter(|x| {
            println!("Value is now: {}", *x);
        });
    });

    std::thread::sleep(std::time::Duration::from_secs(1));

    monitor.enter_and_signal(|x| {
        *x = 10;
        println!("Set value to 10");
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_monitor() {
        let monitor: Arc<Monitor<i32>> = Arc::new(Monitor::new(0));
        let monitor_clone = monitor.clone();

        let handle = thread::spawn(move || {
            monitor_clone.wait(|&mut x| x == 10);
            monitor_clone.enter(|x| {
                assert_eq!(*x, 10);
                *x += 5;
            });
        });

        thread::sleep(Duration::from_millis(100));

        monitor.enter_and_signal(|x| {
            *x = 10;
        });

        handle.join().unwrap();

        monitor.enter(|x| {
            assert_eq!(*x, 15);
        });
    }
}
