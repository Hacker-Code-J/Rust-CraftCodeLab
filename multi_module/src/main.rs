mod operations;

use operations::arithmetic::{add, sub, mul, div}; // Add these functions in your arithmetic module
use operations::bitwise::basic::{logic_and, logic_or, logic_xor}; // Add xor to your basic module
use operations::bitwise::advanced::masks::apply_mask;

fn main() {
    // Arithmetic operations
    let sum = add(15, 27);
    let subtracted = sub(50, 23);
    let multiplied = mul(4, 7);
    let divided = div(100, 5);

    println!("Arithmetic Operations:");
    println!("Sum: {}", sum);
    println!("Subtraction: {}", subtracted);
    println!("Multiplication: {}", multiplied);
    println!("Division: {}", divided);

    // Bitwise basic operations
    let and_result = logic_and(0b1100, 0b1010);
    let or_result = logic_or(0b1100, 0b1010);
    let xor_result = logic_xor(0b1100, 0b1010); // Assuming you add this function

    println!("\nBitwise Basic Operations:");
    println!("AND: {:04b}", and_result);
    println!("OR: {:04b}", or_result);
    println!("XOR: {:04b}", xor_result);

    // Bitwise advanced operations - Masks
    let applied_mask = apply_mask(0b11110000, 0b10101010);

    println!("\nBitwise Advanced Operations - Masks:");
    println!("Applied Mask: {:08b}", applied_mask);
}
