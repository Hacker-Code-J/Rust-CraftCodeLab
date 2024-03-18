// fn main() {
//     let mut s1 = String::from("Rust!");

//     unsafe {
//         let (capacity, ptr, len): (usize, usize, usize) = std::mem::transmute(s1);
//         println!("capacity = {capacity:#x}, ptr = {ptr:#x}, len = {len}");
//     }
// }

// fn main() {
//     let s1: String = String::from("Rust!");
//     let s2: String = s1;
//     println!("s2: {s2}");
//     // println!("s1: {s1}");
// }

// fn say_hello(name: String) {
//     println!("Hello {name}")
// }

// fn main() {
//     let name = String::from("Alice");
//     say_hello(name.clone());
//     say_hello(name);
// }

// fn main() {
//     let five = Box::new(5);
//     println!("five: {}", *five);
// }

#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn main() {
    let list: List<i32> =
        List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}