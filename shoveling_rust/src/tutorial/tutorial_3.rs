pub fn run_stack_memory() {
    let x = 12;
    let y = 14;

    let z = square_fn(x);

    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn square_fn(a: u32) -> u32 {
    a * a
}

pub fn run_string_memory() {
    let a = print_hi();
}

fn print_hi() -> String {
    let mut s = String::from("hi");
    s.push_str("!");

    println!("{}", s);
    s
}

// pub fn run_error_clone() {
//     let s1: String = String::from("Hello, Rust!");
//     let s2 = s1;

//     println!("{}", s1);
//     println!("{}", s2);
// }

pub fn run_correct_clone() {
    let s1: String = String::from("Hello, Rust!");
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);
}

pub fn run_borrow() {
    let s1: String = String::from("Hello, Rust!");
    let s2 = &s1;

    println!("{}", s1);
    println!("{}", s2);
}

pub fn run_borrow2() {
    let s1: String = String::from("Hello, Rust!");
    println!("{}", s1);
    func1(&s1);
    println!("{}", s1);
}

fn func1(s: &String) {
    println!("From func: {}", s);
}

pub fn run_interaction() {
    
}