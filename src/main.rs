// Closures
fn call_with_closure<F>(some_closure: F) -> i32
    where F: Fn(i32) -> i32 {
    some_closure(1)
}


fn main() {
    let plus_one = |x: i32| x + 1; // | are pipes, through which the arguments of the closure go
    if plus_one(1) == 2 {
        println!("Tres bien!");
    }
    // {} is an expression, so closures can be multi-line
    let plus_two = |x: i32| {
        x + 2
    };
    let plus_three = |x: i32| {
        let mut y: i32 = x;
        y += 3;
        y
    };
    if plus_three(plus_two(1)) == 6 {
        println!("Very good!");
    }

    let mut num: i32 = 6;
    {
        let plus_num = |x: i32| x + num; // Borrows 'num' as a mutable
    }
    // let y = &mut num; // Works because plus_num went out of scope
    // Closures copy, to take ownership of a copy, use 'move'
    {
        let mut add_num = move |x: i32| num += x;
        add_num(5);
    }
    
    if num == 6 {
        println!("Very nice!");
    }

    let answer = call_with_closure(|x: i32| x + 23);
    println!("This answer is the result of a function that takes a closure {}", answer);
}
