
fn add_two(x: i32) -> i32{
    x + 2
}

fn add_one(x: i32) -> i32{
    x + 1
}
fn main () {
    println!("hello world!");
    let mut y: i32 = 1;
    let mut f: fn(i32) -> i32 = add_one;
    let y = f(y);
    println!("y is equal to {}",y);
    let f = add_two;
    let y = f(y);
    println!("y is equal to {}",y);
}
