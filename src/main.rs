 
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

/*
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
 */

/*
struct Bar {
    x: i32,
    mut y: i32, // Will not work, since mut is "a property of either a borrow (&mut) or a binding (let mut)."
}
 */

/*
struct Point {
    x: i32,
    y: std::cell::Cell<i32>, // Making x immutable and y emulate mutability
}

fn main () {
    let my_point = Point { x: 11, y: std::cell::Cell::new(3)};
    my_point.y.set(8);
    println!("y: {:?}", my_point.y); // Apparently :? lets you print info about an obj or something
}
 */


struct Point {
    x: i32,
    y: i32,
}

/*
fn main () {
    let origin = Point { x: 0, y: 0 }; // (x,y) -> (0,0)
    println!("The origin is at ({},{})", origin.x, origin.y);
}
 */

/*
fn main () {
    let mut point = Point {x: 0, y: 0}; // "Temporarily" makes point mutable
    point.x = 5;
    println!("The point is now ({},{})", point.x, point.y);
    let point = point; // Point is no longer mut for some unexplained reason. Possibly point is now owned by point?
    println!("The point is now ({},{})", point.x, point.y);
    // point.y = 6; -- this would cause an error
}
 */


struct PointRef <'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

/*
fn main () {
    let mut point = Point { x: 0, y: 0 };
    {
        let r = PointRef{ x: &mut point.x, y: &mut point.y }; //---+ r borrows point with mutable referene
        *r.x = 5;                                             //---|
        *r.y = 6;                                             //---|
    }                                                         //---+ r goes out of scope, along with its borrow, point's x and y now point to (5,6)
    println!("(x,y) is ({},{})", point.x, point.y);
}
 */

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

/*
fn main () {
    let mut point = Point3D { x: 0, y: 0, z: 0 };
    point = Point3D { x: 1, .. point } ; // "Update syntax"
    println!("({},{},{})", point.x, point.y, point.z);
    let origin = Point3D { x: 0, y: 0, z: 0 };
    point = Point3D { z: 5, .. origin};
    println!("({},{},{})", point.x, point.y, point.z);
}
 */

struct IAmEmpty; // Empty struct, can be used to take on traits
// Rust's enums are pretty cool:

enum Message {
    Quit,
    ChangeColor (i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

enum BoardGameTurn {
    Move { squares: i32 },
    pass,
}

fn constructor_as_function_demo(x: String) -> Message {
    Message::Write(x)
}

/*
fn main () {
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1};
    println!("I compile even though Move is used twice! In this line - 2 it is scoped to Message, then in this line - 1 to BoardGameTurn.");
    let m = Message::Write("Hello World".to_string());
    let x = constructor_as_function_demo("Hello World".to_string());
    println!("This line -2 and this line -1 do the same thing; enum constructors can be used as functions");
}
 */

/*
fn main () {
    let x: i32 = 5;
    let number = match x {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        _ => println!("*"),
    };

}
 */

/*
fn main () {
    let x: i32 = 1;
    match x {
        y => println!("x: {} y: {}", x, y), // Creates "a binding for the value in the any case" (Catch all binding)
        // Using a _ here would break the code
    }
}
 */

/*
fn main () {
    let x = 1;
    let c = 'c';
    match c {
        x => println!("x: {} c: {}", x, c), // Binds x to c, but binding goes out of scope
    }
    println!("x: {}", x);
}
 */

/*
fn main () {
    let x: i32 = 1;
    match x {
        1 | 2 => println!("x == 1 || x ==2 returns true"),
        3 => println!("Tres"),
        _ => println!("*"),
    }
}
 */

/*
fn main () {
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, y } => println!("({},{})", x, y), // Compound data type can be destructured w/ pattern
    } // Destructuring works on any compound data including tuple and enum
    // For some godforsaken reason you may want to give the binding a different name
    match origin {
        Point { x: x1, y: y1} => println!("({},{})", x1, y1),
    }
    //And ofc you can destructure and only use one of the bindings
    match origin {
        Point {x, ..} => println!("x is: {}", x),
    }
}
 */

fn coordinate() -> (i32, i32, i32) {
    (0, 0, 0)
}

/*
fn main () {
    let (x, _, z) = coordinate(); // _ is ignored, never binds a value
    println!("{}", x);
    println!("{}", z);
    let _ = String::from("  hello  ").trim(); // Function runs, string is created, string is dropped b/c its not bound to anything
}
 */

enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}

/*
fn main () {
    let x = OptionalTuple::Value(1, 2, 3);
    let matc_for_rf = 5;
    match matc_for_rf {
        ref r => println!("Got me here a reference! {}", r), // "Creates a reference for use in the pattern"
    }
    let mut matc_for_mu_rf = 5;
    match matc_for_mu_rf {
        ref mut mr => println!("Got me here a mutable reference! {}", mr),
    }
    match x {
        OptionalTuple::Value(..) => println!("Got me here a tuple"),
        OptionalTuple::Missing => println!("For me there exists no tuple"),
    }
    let my_char = '💩';
    match my_char {
        'a' ... 'j' => println!("This is an early letter"),
        'k' ... 'z' => println!("This is a late letter"),
        to_matched_bound @ _ => println!("This isn't a letter its a pile of {}", to_matched_bound),
    }
    let matc_me: i32 = 6;
    match matc_me {
        e @ 0 ... 5 | e @ 7 ... 9 => println!("Je n'ecris pas"),
        _ => println!("Bonsoir"),
    }
}
 */

enum OptionalInt {
    Value(i32),
    Missing,
}

/*
fn main () {
    let x = OptionalInt::Value(8);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int larger than five!"),
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("I don't recall saying good luck"),
    }
    let y = false;
    match x {
        OptionalInt::Value(8) | OptionalInt::Value(9) if y => println!("bien sur"), // if y for both OptInt::V(8) and OptInt::V(9)
        _ => println!("non"),
    }
}
 */

// On to method syntax
// I don't actually think baz(bar(foo)) is that confusing
// But rust has a circumlocution that I suppose they find ergonomic

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn immutable_reference(&self) {
        println!("Taking myself by immutable reference!");
    }
    
    fn mutable_reference(&mut self) {
        println!("Taking self by mutable reference - this isn't very nice");
    }
    
    fn takeing_ownership(self) {
        println!("I'm not borrowing myself, this is very rude");
    }
    
    fn area(&self) -> f64 {
        let pi: f64 = std::f64::consts::PI;
        pi * (self.radius * self.radius)
    }
    
    fn grow(&self, increment: f64) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius + increment }
    }
    
    // Associated functions / static methods:
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x: x, y: y, radius: radius, }
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder { x: 0.0, y: 0.0, radius: 1.0, }
    }
    
    fn set_x(&mut self, coord: f64) -> &mut CircleBuilder {
        self.x = coord;
        self
    }
    
    fn set_y(&mut self, coord: f64) -> &mut CircleBuilder {
        self.y = coord;
        self
    }

    fn set_radius(&mut self, coord: f64) -> &mut CircleBuilder {
        self.radius = coord;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius, }
    }

}

fn main() {
    let my_circle = Circle { x: 0.0, y: 0.0, radius: 2.0, };
    println!("My circle's area is: {}", my_circle.area());
    let my_bigger_circle_area = my_circle.grow(2.0).area();
    println!("My bigger circle's area is: {}", my_bigger_circle_area);
    println!("The circle made by a static function's area is: {}", Circle::new(0.0, 0.0, 5.0).grow(2.0).area()); // I().think().I().get().where().this().is().going
    let a_builder_built_this = CircleBuilder::new().set_x(0.0).set_y(0.0).set_radius(5.0).finalize();
    println!("A builder (A lot like a constructor) this circle:");
    println!("X coordinate: {}, Y coordinate: {}, Radius: {}", a_builder_built_this.x, a_builder_built_this.y, a_builder_built_this.radius);
}
