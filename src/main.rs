#[config(not(windows))] // Compilation on the condition that its not windows

macro_rules! five_times {
    ($x: expr) => (5 * $x); // Notice how it is similar to a match branch
}

/*
fn main () {
    println!("Unlike in C, macros are not inline, but have thier own syntax rules. Five times 2 + 3 = {}", five_times!(2+3));
}
 */


// I am already familiar with raw poitners, so in the interest of time I'll just go strait to stack / heap

/*
fn main () {
    let i = Box::new(5);
    let j = Box::new(6);
    println!("i and j are allocated on the heap: {}, {}", i, j);
    if cfg!(target_os = "linux") {
        println!("You are running a linux-based os"); // cfg!(target_os = "linux") replaced with true at compile time
    }
}
 */

// Okay this is really interesting: for i..j is, in interval notation, [i,j)
// and for i...j is [i,j]
// My gawd, you dont do C style for loops to iterate over vectors:

/*
fn main () {
    let not_doing_it_c_style = vec!["\nHello", " this", " is", " crazy!\n"];
    for not_doing_it_c_style in &not_doing_it_c_style {
        print!("{}", not_doing_it_c_style);
    }
}
 */

/*
use std::thread;

fn main() {
    let x = 10;
    let handle = thread::spawn(move || {
       println!("Hello from the othersiiiiiide, x is {}", x);
    });
    // println!("{}", handle.join().unwrap()); handle.join().unwrap() is what thread::spawn returns
    
}
 */

use std::thread;
use std::time::Duration;
// use std::rc::Rc; // Reference count, not thread safe
use std::sync::{Arc, Mutex};

/*
fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4]));

    for i in 0..3 { // Hey I thought you weren't supposed to use C style iteration?!
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap(); // fn lock(&self) -> LockResult<MutexGuard<T>>
            data[0] += i;
        });
    }

    thread::sleep(Duration::from_millis(50));
}
 */

// Using channels instead of waiting for a set time:

use std::sync::mpsc;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let (tx, rx) = mpsc::channel();
    for _ in 0..10 { // for _, thats a new one... for anything in that iterator w/o binding anything, I suppose
        let (data, tx) = (data.clone(), tx.clone());
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
    // Panics are confined to one thread; you can use panics to isolate errors
    let handle = thread::spawn(move || {
        panic!("I am isolated");
    });

    let result = handle.join();
}
