pub fn run() {
    let password = "ExamplePassword123";
    println!("Validating password: {}", password);
    if is_password_strong(password) {
        println!("Password is strong.");
    } else {
        println!("Password is weak.");
    }
}

fn is_password_strong(password: &str) -> bool {
    // Simple criteria for demonstration purposes:
    password.len() > 8 && password.chars().any(char::is_numeric)
}