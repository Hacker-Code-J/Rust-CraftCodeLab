mod tutorial;

fn main() {
    println!("Learning Rust:\n");

    /* 
     * Tutorial #1 - Variables and Data Types
     * - Variables
     * - Data Type
     * - Data Shadowing
     * - Pointer and Reference
    */
    // tutorial::tutorial_1::run_variable();
    // tutorial::tutorial_1::run_data_type();
    // tutorial::tutorial_1::run_variable_shadowing();
    // tutorial::tutorial_1::run_ptr_ref();

    /* 
     * Tutorial #2 - Mutability, Functions and Control Flow
     * - Mutability
     * - Functions
     * - Conditionals (if, else if, else)
     * - Loops (loop, while, for)
    */
    // tutorial::tutorial_2::run_mutability();
    // tutorial::tutorial_2::run_func();
    // tutorial::tutorial_2::run_condition();
    // tutorial::tutorial_2::run_loop();

    /*
     * Tutorial #3 - Ownership and Borrow
     * - Ownership
     * - Borrow
     * - Interaction
    */
    // tutorial::tutorial_3::run_stack_memory();
    // tutorial::tutorial_3::run_string_memory();
    // tutorial::tutorial_3::run_error_clone();
    // tutorial::tutorial_3::run_correct_clone();
    // tutorial::tutorial_3::run_borrow();
    tutorial::tutorial_3::run_borrow2();

}