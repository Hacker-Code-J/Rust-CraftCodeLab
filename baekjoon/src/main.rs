use std::io;

// Function to find GCD
// fn gcd(a: i32, b: i32) -> i32 {
//     if b == 0 { a } else { gcd(b, a % b) }
// }

// Function to find extended GCD
fn gcd_extended(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = gcd_extended(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// Function to find the minimum positive x that satisfies: x % a = r1, x % b = r2
fn crt(a: i32, b: i32, r1: i32, r2: i32) -> i64 {
    let (g, x, _) = gcd_extended(a, b);
    if (r2 - r1) % g != 0 {
        return -1;
    }

    let mod_val = a / g * b;
    let x0 = ((r1 as i64 + x as i64 * ((r2 - r1) / g) as i64 % (b / g) as i64 * a as i64) % mod_val as i64) as i64;

    if x0 < 0 { x0 + mod_val as i64 } else { x0 }
}

// Function to find the smallest integer N
fn find_smallest_n(p1: i32, p2: i32, p3: i32, x1: i32, x2: i32, x3: i32) -> i64 {
    let n12 = crt(p1, p2, x1, x2);
    let n23 = crt(p2, p3, x2, x3);
    let n31 = crt(p3, p1, x3, x1);

    if n12 == -1 || n23 == -1 || n31 == -1 {
        return -1;
    }

    if n12 > i32::MAX as i64 || n23 > i32::MAX as i64 || n31 > i32::MAX as i64 {
        return -1;
    }

    let (gcd, _, _) = gcd_extended(p1, p2);
    let n = crt(p1 * p2 / gcd, p3, n12 as i32, x3);

    if n < 1000000000 { n } else { -1 }
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|num| num.parse().expect("Please type a number"))
        .collect();

    if nums.len() != 6 {
        println!("Please enter exactly six integers.");
        return;
    }

    let result = find_smallest_n(nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);
    println!("{}", result);
}
