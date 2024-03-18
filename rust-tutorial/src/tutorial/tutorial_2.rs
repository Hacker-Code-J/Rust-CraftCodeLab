pub fn run_mutability() {
    let mut x = 10;
    println!("{}", x);
    x = 12;
    println!("{}", x);

    println!();

    let mut arr = [ 2, 3, 5, 3 ];
    println!("{:?}", arr);
    arr[1] = 12;
    println!("{:?}", arr);
}

/*
fn function_name(parameters) -> return type {
    // body
}
*/

fn square(x: u32) -> u32 {
    // return x * x;
    x * x
}

fn square2(x: &mut u32){
    // return x * x;
    (*x) = (*x) * (*x)
}

pub fn run_func() {
    let mut arr = [ 2, 3, 5, 3 ];
    
    arr[1] = square(arr[1]);
    square2(&mut arr[2]);
    println!("{:?}", arr);
}

/*
if condition1 {

} else if condition2 {

} else {

}
*/

pub fn run_condition() {
    let a = 10;
    let b = 12;

    if a < b {
        println!("a < b");
    } else if b < a {
        println!("a > b");
    } else {
        println!("a = b");
    }

    // let mut c = 0;
    // if a < b {
    //     c = a;
    // } else {
    //     c = b;
    // }
    let c =  if a < b {
        a
    } else {
        b
    };
    println!("min(a, b) = {}", c);
}

/*
- loop
- while
- for
*/

pub fn run_loop() {
    /* 1. loop */
    // let mut i = 0;
    // loop {
    //     println!("{}", i);
    //     i += 1;
    //     if i >= 10 {
    //         break;
    //     }
    // }

    // let mut counter = 0;

    // let a = loop {
    //     counter += 1;

    //     if counter >= 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("counter: {}, {}", counter, a);

    /* 2. while */
    /*
    while condition is true {
        // do something
    }
    */
    // let mut i = 0;
    // let arr = [ 2, 3, 5, 3 ];

    // while i < arr.len() {
    //     println!("{}", arr[i]);
    //     i += 1;
    // }

    /* 3. for */
    /*
        for something in some range {
            // do something
        }
    */
    let arr = [ 2, 3, 5, 3 ];

    // for i in 0..arr.len() {
    //     println!("{}", arr[i]);
    // }

    for a in &arr {
        println!("{}", a);
    }
}