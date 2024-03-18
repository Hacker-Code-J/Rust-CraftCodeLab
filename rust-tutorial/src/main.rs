mod tutorial;

fn main() {
    println!("Learning Rust:\n");

    /*
     * Reference:
     * - https://www.youtube.com/watch?v=HqJqvoqd-jE&list=PLkO5ggdQuRaaeFke7nWS4ajhFVZ1biE7_
     */

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
    // tutorial::tutorial_3::run_borrow2();

    /*
     * Tutorial #4 - Structs and Methods
     * - Structs
     * - Strokes
    */
    // tutorial::tutorial_4::run_struct();
    // tutorial::tutorial_4::run_stroke();
    // tutorial::tutorial_4::run_stroke2();

    /*
     * Tutorial #5 - Enums, Option and Pattern Matching
     * - Enums
     * - Pattern Matching
    */
    // tutorial::tutorial_5::run_enum();
    // tutorial::tutorial_5::run_enum2();
    // tutorial::tutorial_5::run_pattern_matching();
    // tutorial::tutorial_5::run_option();

    /*
     * Tutorial #6 - Trait and Generics
     * - Traits
     * - Generics
     * - Coding
    */
    tutorial::tutorial_6::run_trait1();

}