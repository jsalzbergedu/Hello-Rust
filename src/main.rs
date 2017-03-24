// Skipping (gasp) crates because I already have to use them elsewhere

// const and static: this should be quick

static mut JE_SUIS_MONDIAL: i32 = 10;

/*
fn main() {
    const ABACKDEE: i32 = 4;
    println!("I am inlined wherever I'm found: {}", ABACKDEE);
    unsafe {
        JE_SUIS_MONDIAL += JE_SUIS_MONDIAL;
        println!("Modifying static mut s is not safe (bad): {}", JE_SUIS_MONDIAL);
    }
}
 */

// Since this is such a small file, I'll also include attributes

// Declared for functions like this:
#[test]
fn test_func() {
    println!("I am a test function only compiled with --test.");
}

// #[main] exept for now this cannot be used
fn not_main () {
    println!("I become the entry point");
}

fn main() {
    println!("Another one of interest is should_panic, which inverts the sucess condition.");
}
