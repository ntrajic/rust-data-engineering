/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
* The provided code is a Rust implementation of the classic concurrency problem known as the “Dining Philosophers” problem1.
* This problem involves multiple threads (philosophers) needing synchronized access to shared resources (forks), risking deadlock1.
* In this code, philosophers are modeled as threads and forks as shared Mutex<()> wrapped in Arc for thread-safe reference counting1.
* To prevent deadlock, philosophers acquire the lower-numbered fork first1. This breaks symmetry and avoids circular waiting1.
* The unwrap() function in Rust is a method that can be called on Option or Result types23.
* If called on an Option, unwrap() will return the value inside an Some variant of the Option23.
* If called on a Result, unwrap() will return the value inside an Ok variant23.
* If unwrap() is called on a None or Err variant, the function will panic and the program will terminate23.
*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _first_guard = first_fork.mutex.lock().unwrap();     // unwrap() can be invoked on None or  
        println!("{} picked up fork {}.", self.name, first_fork.id);
        let _second_guard = second_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, second_fork.id);
    }

    /*  To make the code more robust and avoid panicking, you could replace unwrap() with more error-handling code.
     *  For example, instead of calling unwrap() on the MutexGuard returned by lock(),
     *  you could handle the Err variant explicitly. Here’s how you might modify the eat function:
     */
    fn eat_deterministic(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _first_guard = match first_fork.mutex.lock() {
            Ok(guard) => guard,
            Err(e) => {
                println!("Failed to acquire lock on first fork: {:?}", e);
                return;
            }
        };
        println!("{} picked up fork {}.", self.name, first_fork.id);

        let _second_guard = match second_fork.mutex.lock() {
            Ok(guard) => guard,
            Err(e) => {
                println!("Failed to acquire lock on second fork: {:?}", e);
                return;
            }
        };
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, second_fork.id);
    }
}

fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");

    //we only have 4 forks at the table
    let forks = (0..4)
        .map(|id| {
            Arc::new(Fork {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect::<Vec<_>>();

    let philosophers = vec![
        ("Jürgen Habermas", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Karl Marx", 2, 3),
        ("Thomas Piketty", 3, 0),
        ("Michel Foucault", 0, 1),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Epicurus", 0, 1),
        ("Zeno of Citium", 1, 2),
        ("Thales of Miletus", 2, 3),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (name, left, right))| {
        Philosopher::new(
            id as u32,
            name,
            Arc::clone(&forks[left]),
            Arc::clone(&forks[right]),
        )
    })
    .collect::<Vec<_>>();

    let start = Instant::now();

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                //philosopher.eat();
                philosopher.eat_deterministic();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}
