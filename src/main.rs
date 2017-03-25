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

fn main () {
    let not_doing_it_c_style = vec!["\nHello", " this", " is", " crazy!\n"];
    for not_doing_it_c_style in &not_doing_it_c_style {
        print!("{}", not_doing_it_c_style);
    }
}
