pub fn run() {
    // Ownership concept examples
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // This would result in a compile-time error

    let x = 5;
    makes_copy(x);
    println!("{}", x); // This is fine
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}