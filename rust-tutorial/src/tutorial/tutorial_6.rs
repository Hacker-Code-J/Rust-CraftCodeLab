pub trait MyTrait {
    fn sum(&self) -> u32;    
}

struct MyStruct {}
impl MyTrait for MyStruct {
    fn sum(&self) -> u32 {
        10
    }
}

struct MyStruct2 {
    size: u32
}
impl MyTrait for MyStruct2 {
    fn sum(&self) -> u32 {
        self.size
    }
}

fn print_sum(m: &impl MyTrait) {
    println!("sum: {}", m.sum());
}

pub fn run_trait1() {
    let my_struct = MyStruct {};
    let my_struct2 = MyStruct2 { size: 12};

    print_sum(&my_struct);
    print_sum(&my_struct2);
}