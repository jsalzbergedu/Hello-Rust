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

fn do_something<T: Foo>(x: T) {
    x.method();
}

fn main() {
    let x = 2u8;
    let y = "Bonjour".to_string();
    println!("This is static dispatch: {:?}", do_something(x));
    println!("And so is this: {:?}", do_something(y));
}
