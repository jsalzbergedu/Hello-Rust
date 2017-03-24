// Skipping (gasp) crates because I already have to use them elsewhere

// const and static: this should be quick

static mut JE_SUIS_MONDIAL: i32 = 10;

fn main() {
    const ABACKDEE: i32 = 4;
    println!("I am inlined wherever I'm found: {}", ABACKDEE);
    unsafe {
        JE_SUIS_MONDIAL += JE_SUIS_MONDIAL;
        println!("Modifying static mut s is not safe (bad): {}", JE_SUIS_MONDIAL);
    }
}
