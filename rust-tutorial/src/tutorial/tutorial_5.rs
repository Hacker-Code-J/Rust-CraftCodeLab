#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}
pub fn run_enum() {
    let address1 = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let address2 = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", address1);
    println!("{:?}", address2);
}

/* =============================================================== */

#[derive(Debug)]
enum IPAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
    V7 {x: u32, y: u32},
}

pub fn run_enum2() {
    let address1 = IPAddr2::V4(127, 0, 0, 1);
    let address2 = IPAddr2::V6(String::from("::1"));
    let address3 = IPAddr2::V7 {x: 10, y: 12};

    println!("{:?}", address1);
    println!("{:?}", address2);
    println!("{:?}", address3);
}

/* =============================================================== */
#[derive(Debug)]
enum IPAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
    V7 {x: u32, y: u32},
}

pub fn run_pattern_matching() {
    let address1 = IPAddr3::V4(127, 2, 3, 1);
    let address2 = IPAddr3::V6(String::from("::1"));
    let address3 = IPAddr3::V7 {x: 10, y: 12};

    match address1 {
        // IPAddr3::V4(127, _, _, 1) => println!("localhost"),
        IPAddr3::V4(127, b, c, 1) => println!("localhost"),
        IPAddr3::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
        IPAddr3::V6(s) => println!("{}", s),
        // IPAddr3::V7 {x, y} => {
        //     let z = x + y;
        //     println!("{}", z);
        // }
        _ => println!("Don't know what his is"),
    }
}

// enum Option<T> {
//     Some(T)
//     None
// }

pub fn run_option() {
    // let num: Option<u32> = Some(5);
    let num: Option<u32> = None;

    match num {
        Some(n) => println!("{}", n),
        None => println!("No value!"),
    }
}