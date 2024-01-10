// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

fn main() {
    let life = 24;
    println!("hello");
    println!("{:?}", life);
    println!("{:?} {:?}", life, life);
    println!("the meaning is {:?}", life);
    println!("{life:?}");
    println!("{life}");

    // let a = 500;
    // // if a > 200 {
    // //     println!("Huge number");
    // // } else if a > 99 {
    // //     println!("Big number");
    // // } else {
    // //     println!("Small number");
    // // }
    // if a > 99 {
    //     println!("Big number");
    // } else if a > 200 {
    //     println!("Huge number");
    // } else {
    //     println!("Small number");
    // }

    // let mut a = 0;
    // loop {
    //     if a == 5 {
    //         break;
    //     }
    //     println!("{:?}", a);
    //     a = a + 1;
    // }

    let mut a = 0;
    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }
}
