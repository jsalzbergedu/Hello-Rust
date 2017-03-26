use std::env; // cool as shizzle

fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset); // Some is a "value" constructor for Option type
        }
    }
    None
}

/*
fn main() {
    let mut argval = env::args();
    let arg: String = argval.nth(1).unwrap(); // If no args are passed to it
    let n:i32 = arg.parse().unwrap(); // It crashes. Plus here the first arg needs to be an integer
    println!("Two times the number you entered is {}", n * 2);
    // Unwrap is a function that gives the result of computation or panics if no computation
    // Unwrap is implemented on Option and Result types
    // Unwrap does the Some, None for you
}
 */

fn extension_explicit(file_name: &str) -> Option<&str> { // Just an example, use std's extension for extensions
    match find(file_name, '.') {
        Some(i) => Some(&file_name[i+1..]),
        None => None,
    }
}

/*
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1]..)
}
 */ // Doesn't work for case analysis, map is always with some

fn main() {
    let file_name = "foobarbaz.rs";
    match find (file_name, '.') {
        None => println!("None"),
        Some(i) => println!("File extension: {}", &file_name[i+1..]), // Once again the documentation shows indexing the string... strange - oh wait, nvm
    }
    match extension_explicit(file_name) {
        Some(i) => println!("File extension: {}", i),
        None => println!("None"),
    }
    
    /*
    match extension(file_name) {
        Some(i) => println!("File extension: {}", i),
        None => println!("None"),
    }
     */
    
}
