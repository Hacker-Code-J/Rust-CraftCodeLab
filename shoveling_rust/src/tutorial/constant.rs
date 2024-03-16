pub fn run_basic_constant() {
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
}

pub fn run_intermediate_constant() {
    
}

const THRESHOLD: i32 = 10;

fn is_above_threshold(value: i32) -> bool {
    value > THRESHOLD
}

pub fn run_advanced_constant() {
    let values = vec![9, 10, 11, 12];
    for value in values {
        if is_above_threshold(value) {
            println!("{} is above the threshold", value);
        } else {
            println!("{} is not above the threshold", value);
        }
    }
}