pub fn run_basic_shadowing() {
    let x = 5;
    let x = x + 1; // shadowing by re-using the let keyword
    {
        let x = x * 2; // shadowed within this scope only
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

pub fn run_intermediate_shadowing() {
    
}

pub fn run_advanced_shadowing() {
    
}