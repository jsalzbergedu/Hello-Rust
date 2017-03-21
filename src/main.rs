 
fn add_two(x: i32) -> i32{
    x + 2
}

fn add_one(x: i32) -> i32{
    x + 1
}
/*
fn main () {
    println!("hello world!");
    let mut y: i32 = 1;
    let mut f: fn(i32) -> i32 = add_one;
    let y = f(y);
    println!("y is equal to {}",y);
    let f = add_two;
    let y = f(y);
    println!("y is equal to {}",y);
    let my_array: [i32; 3] = [1, 2, 3];
    println!("my_array has {} elements", my_array.len());
    for i in 0..(my_array.len()) {
        println!("Element in array: {}", my_array[i]);
    }
    let slicin_str: &str  = "Slice me up!";
    println!("All sliced {}", &slicin_str[9..12]);
    let mut x: (i32, &str) = (6, "hello!");
    println!("Ecce Tuple: {} {}", x.0, x.1);
    let y = (add_one(x.0), "Ah Bartleby! Ah humanity!");
    x.0 = y.0;
    println!("Ecce Tuple: {} {}", x.0, x.1);
}
 */

/* Numeric types:
i8 -- i stands for integer (signed)
i16
i32
i64 -- probably not supported on many systems
u8 -- u stands for unsigned integer
u16
u32
u64 -- probably not supported on many systems
isize -- pointer sized integer
usize
f32 -- 32 bit floating
f64 
 */

/*
fn main () {
    // if is an expression in Rust.
    let x: i32 = 5;
    let y:i32 = if x == 5 {10} else {15}; // if without else always returns empty tuple
    println!("y = {}", y);
}
 */

/*
fn main () {
    loop {
        println!("Looping forever sounds like a bad idea.");
    }
}
 */

/*
fn main () {
    let mut x: i32 = 5;
    let mut first_bool: bool = false;
    while !first_bool {
        x += x - 3;
        println!("{}", x);

        if x % 5 == 0 {
            first_bool = true;
        }
    }
}
 */

/*
fn main () {
    for (i, j) in (5..10).enumerate() {
        println!("Are i: {} and j: {} similar to how it works in C?", i, j);
    }
    for (i, j) in (7..10).enumerate() {
        println!("Parfois i: {} gets set to zero and j: {} gets set to tuple.1?", i, j);
    }
}
 */

/*
fn main () {
    let lines = "hello\nworld".lines(); // An iterator, apparently
    for (i, j) in lines.enumerate() {
        println!("{}:{}", i, j);
    }
   /*  for (i, j, y) in (5..10).enumerate() {
        println!("So i: {} starts at zero, j: {} starts at tuple.1, and y: {} starts at ?", i, j, y);
    } I see, this does not work. */
        
}
 */

/*
fn main () {
    let mut x = 1;

    loop {
        println!("Looping forever is a bad idea. Using continue, x = {}", x);
        x += x;
        if x <= 30 {continue;}
        else {break;}
    }
    loop {
        x += x;
        println!("Looping forever is a bad idea. {}", x);
        if x >= 50 {break;}
    }

    'first_for: for i in (0..10) {
        'second_for: for j in (0..10) {
            if i % 3 == 0 {continue 'first_for;}
            if j % 3 == 0 {continue 'second_for;}
            println!("i: {} j: {}", i, j);
        }
    }
}
 */
/*
fn main () {
    let mut my_vector: Vec<i32> = vec![1, 2, 3, 4, 5]; // I store my data on the heap, but my size has to be known at compile time
    let my_other_vector = vec![0;10]; // Initializes vec with 10 elements, all 0
    let i: usize = 0;
    println!("My rust vector's first elements are {} & {}", my_vector[i], my_other_vector[i]);
    match my_vector.get(8) {
        Some(j) => println!("This will not run; my_vector has no eigth element"),
        None => println!("I can go out of bounds without panicking!")
    }
    
    for i in &my_vector {
        println!("Elements iterated using a reference: {}", i);
    }

    for i in &mut my_vector {
        println!("Elements iterated using a mutable reference : {}", i);
    }

    for i in my_vector {
        println!("Elements iterated by i taking ownership of the vector and its elements: {} cannot be re-used.", i);
    }
    // Vectors have pretty normal functions, like vector.push(element) and vector.pop()
}
    
 */

/*
fn take (v: Vec<i32>) {
    //take will take ownership of a vec used as param
    println!("I'm taking ownership of the vector that starts with {}", v[0]);
}

fn main () {
    let my_vector: Vec<i32> = vec![1,2,3]; // creates Vec's pointers on stack, allocates heap space for elements and points pointers to heap
    let my_other_vector = my_vector;
    // println!("This won't work : {}", my_vector[0]);
    take(my_other_vector);
    println!("This wont work: {}", my_other_vector[0]);
    // Rust only allows one pointer to one point on the heap, so vectors, being afaik an array of pointers, can't be copied
    // Primitives OFC can be copied
} // my_vector goes out of scope, resources are free()'d
*/

//Use references to "borrow" things for functions rather than taking ownership

/*
fn my_borrower (v_one: &Vec<i32>, v_two: &Vec<i32>) -> i32 {
    println!("First element of v_one: {}", v_one[0]);
    println!("First element of v_two: {}", v_two[0]);
    42
} // Borrowing does not deallocate 

fn main () {
    let my_vector: Vec<i32> = vec![0, 1, 2, 3];
    let my_other_vector: Vec<i32> = vec![0, 1, 2, 3, 4];
    println!("This will work: {} {}", my_vector[1], my_other_vector[2]);
    my_borrower(&my_vector, &my_other_vector);
    println!("And so will this: {} {}", my_vector[2], my_other_vector[2]);
}
 */

/*
fn main () {
    let mut x = 5;
    {
        let y = &mut x; // &mut allows changing the resource being borrowed, this borrows x as an &mut
        *y += 1; // Apparently y is an &mut so it needs a *
    } // ends the scope of y's borrow as a mut
    println!("Lets see what happens to x: {}", x); // This borrows x as an immutable, possible because y's mutable borrow ended
    // As many borrows as a reference as needed, only one borrow as a mutable reference
    // Data race impossible: only one thing can have the &mut (be writing to mem location) Neato!
    // Moreover, any borrow has to have a smaller scope than the scope of the thing its borrowing from
}
 */

// A function, e.g., fn bar <>(), can have generics in the <>, including lifetimes
// The lifetime, eg 'life_time, goes inbetween the reference and the primitive type,
// e.g., fn bar<'life_time>(i: &'life_time mut i32) or fn bar <'life_time>(i: &'life_time i32)

struct Foo <'a> {
    x: &'a i32, // Lifetime to keep reference of Foo from outliving the things it owns
}

impl <'a> Foo <'a> {
    fn x(&self) -> & 'a i32 { self.x } // "implementing a method on Foo" Apparently I'm giving Foo a method
}

fn main () {
    let y = &5;
    let f = Foo { x: y };
    println!("{}", f.x()); // Apparently the method on foo returns its member's value

}
