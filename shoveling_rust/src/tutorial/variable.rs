use std::collections::HashMap;

pub fn run_basic_variable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // mutable variable
    println!("The value of x is: {}", x);
}

pub fn run_intermediate_variable() {
    
}

pub fn run_advanced_variable() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Update the score for "Blue" team
    let team_name = String::from("Blue");
    let score = scores.entry(team_name).or_insert(0);
    *score += 10;

    println!("{:?}", scores);
}
