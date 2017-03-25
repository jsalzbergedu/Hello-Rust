fn guess_num(n: i32) -> bool {
    match n {
        1 => true,
        _ => panic!("Invalid number!"),
    }
}

fn main() {
    guess_num(11);
}
