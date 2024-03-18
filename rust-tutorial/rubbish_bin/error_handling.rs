pub fn run() {
    // Example of error handling
    match check_number(42) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
}

fn check_number(value: i32) -> Result<String, String> {
    if value < 50 {
        Ok("Number is less than 50".to_string())
    } else {
        Err("Number is too large".to_string())
    }
}