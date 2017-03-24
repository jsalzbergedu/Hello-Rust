// Universal function call syntax: this should be a quick one:

trait Foo {
    fn f(&self);
}

trait Bar {
    fn f(&self);
}

struct Baz;

impl Foo for Baz {
    fn f(&self) {
        println!("I am Baz' implementation of Foo");
    }
}

impl Bar for Baz {
    fn f(&self) {
        println!("I am Baz' implementation of Bar");
    }
}

fn main() {
    let my_struct = Baz;
    // my_struct.f() is not an option b/c it would be ambiguous
    Foo::f(&my_struct); // However you can call them this way
    Bar::f(&my_struct);
    // Or this way
    <Baz as Foo>::f(&my_struct); // Calls the trait's method, not the method of the inherent one *cough* the instance *cough*
    <Baz as Bar>::f(&my_struct);
}
