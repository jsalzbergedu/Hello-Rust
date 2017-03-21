 
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
}
    
