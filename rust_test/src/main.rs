fn main() {
    let a = String::from("Alice");
    println!("{}", a);

    let b = &a;
    println!("{:p}", b);

    let c = a.clone();
    println!("{}", c);
}
