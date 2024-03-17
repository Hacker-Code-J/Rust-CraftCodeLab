pub fn run_variable() {
    // let x: u32 = 10 + 6;
    // let y: f32 = 12.4;

    // let a: bool = true;

    // let ch: char = 'A';
    // let s: &str = "Hello, Rust!";

    /* Type Inference */
    let x = 10 + 6;
    let y = 12.4;
    let z = x;

    let a = true;

    let ch = 'A';
    let s = "Hello, Rust!";

    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("a: {}", a);
    println!("ch: {}, s: {}", ch, s);
}

pub fn run_data_type() {
    // let my_tup: (u8, char, &str) = (255, 'h', "Rust!");
    // let my_arr: [&str; 3] = [
    //     "First String",
    //     "Second String",
    //     "Another String",
    // ];

    /* Type Inference */
    let my_tup = (255, 'h', "Rust!");
    let my_arr = [
        "First String",
        "Second String",
        "Another String",
    ];

    // println!("({}, {}, {})", my_tup.0, my_tup.1, my_tup.2);
    // println!("[{}, {}, {}]", my_arr[0], my_arr[1], my_arr[2]);

    /* Debugging on Code */
    // println!("{:?}", my_tup);
    // println!("{:?}", my_arr);

    /* Readability */
    println!("{:#?}", my_tup);
    println!("{:#?}", my_arr);
}

pub fn run_variable_shadowing() {
    let my_arr: (u8, char, &str) = (255, 'h', "Rust!");
    println!("{:?}", my_arr);
    let my_arr: [&str; 3] = [
        "First String",
        "Second String",
        "Another String",
    ];
    println!("{:?}", my_arr);
}

pub fn run_ptr_ref() {
    let a = 65;
    let b = &a;
    let c = *b;

    let d = &b; // &&a

    // println!("{} {} {} {}", a, b, c, d);
    // println!("{} {} {} {}", *a, *b, *c, **d); // error
    println!("{} {} {} {}", a, *b, c, **d);
}

