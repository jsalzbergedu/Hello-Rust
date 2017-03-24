// Coercion:
// let, as, const, and static
// is implicit, e.g., &mut T is passed as &T to a function

/*
use std::mem;

fn main() {
    // safe casts:
    // Numeric casts
    let x: i32 = 7;
    let y = x as i64;

    // Pointer casts
    let a = 343 as *const char;
    let b = a as u32;
    // The above operation is safe
    // The following however is not safe
    unsafe {
        let c = [0u8, 1u8, 0u8, 0u8];
        let d: u32 = mem::transmute(c);
        println!("This is not gaurenteed to be safe, {:?}", c);
    }
}

 */

// Associated traits:

// rather than doing trait Graph<N, E>
// just do this

struct Node;
struct Edge;
struct MyGraph;

trait Graph {
    type N;
    type E;
}

impl Graph for MyGraph {
    type N = Node;
    type E = Edge;
}

fn distance<G: Graph>(Graph: &G, start: &G::N, end: &G::N) {
    println!("Much simpler, and you can just use G::N w/o using E");
}


// You can use ?Sized to opt out of Generic's automatic Sized trait
// I'll go back to the operator overloading chapter when it comes up in something useful
// If something implements the trait Deref, it automatically coerces from & to *
