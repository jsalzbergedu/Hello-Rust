/*
use std::fmt::Debug;

struct Rectangle<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

impl<T: PartialEq> Rectangle<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn foo<T: Clone, K: Clone + Debug>(x: T, y: K) {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

fn bar<T, K>(x: T, y: K) where T: Clone, K: Clone + Debug {
    x.clone();
    y.clone();
    println!("{:?}", y);
}

// Where clauses also allow bounds on left hand, e.g., T in the where T: Clone, to be types, e.g., i32

fn main () {
    let mut my_rect = Rectangle { x: 0, y: 0, width: 1, height: 1 };
    if my_rect.is_square() {
        println!("Very Good!");
    }
    my_rect.height = 0;
    if !my_rect.is_square() {
        println!("Genial!")
    }
    foo ("Hello", "World");
    bar ("Bonjour", "Le Monde");
}

// Rules for impl traits
//     1) If you are using a trait, i.e., from the standard library, you have to 'use' it first
//        For instance: use std::io::Write to use the Write trait
//     2) Trait/Type must be defined in the same crate that you are crate you write the impl in

 */


// Default method when you know how the other will be implemented:
trait Foo {
    fn is_valid(&self) -> bool;
    fn is_invalid(&self) -> bool { !self.is_valid() }
}

struct DootDoot;

impl Foo for DootDoot {
    fn is_valid(&self) -> bool {
        println!("Called is_valid()");
        true
    }
}
// Overrides:
struct DootDootFooOverridden;

impl Foo for DootDootFooOverridden {
    fn is_valid(&self) -> bool {
        println!("Called is_valid!");
        true
    }
    fn is_invalid(&self) -> bool {
        println!("Called is_invalid!");
        true
    }
}

fn main() {
    let (instance_one, instance_two) = (DootDoot, DootDootFooOverridden);
    instance_one.is_invalid();
    instance_two.is_invalid();
}
