   
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
    let my_array: [i32; 3] = [1, 2, 3];
    println!("my_array has {} elements", my_array.len());
    for i in 0..(my_array.len()) {
        println!("Element in array: {}", my_array[i]);
    }
    let slicin_str = "Slice me up!";
    println!("All sliced {}", &slicin_str[9..11]);
}
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
f34 
*/
