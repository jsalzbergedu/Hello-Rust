// Coercion:
// let, as, const, and static
// is implicit, e.g., &mut T is passed as &T to a function

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
