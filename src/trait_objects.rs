trait Foo {
    fn method(&self) -> String;
    
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }

}

impl Foo for String {
    fn method(&self) -> String { format!("String: {}", *self) }

}

// Static dispatch (No overhead abstraction, 'monomorphization,' i.e., rust optimizer creates seperate functions for u8 and String

fn do_something(x: &Foo) {
    x.method();
}

/*
fn main() {
    let x = 2u8;
    let y = "Bonjour".to_string();
    println!("This is static dispatch: {:?}", do_something(x));
    println!("And so is this: {:?}", do_something(y));
}
 */

// And now on to dynamic dispatch
// Trait objects, e.g., &Foo or Box<Foo>, store a value of any type that implements a given trait,
// allowing the actual type to be used at runtime.
// They can be used by casting or coercing, e.g., &x as &Foo or using &x as a function with the
// parameter &Foo

fn main () {
    let x = 5u8;
    println!("This uses dynamic dispatch (type casting): {:?}", do_something(&x as &Foo));
    println!("And so does this (coercion): {:?}", do_something(&x));
}

// One cannot use trait objects for things that use Self or have type parameters, e.g.,
// let this_will_not_work = &v as &Clone
// Because it is not "Object safe."
