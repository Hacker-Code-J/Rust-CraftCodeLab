#[derive(Debug)]
struct User {
    username: String,
    email: String,
    logged_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        logged_in_count: 0,
        active: true
    }
}

pub fn run_struct() {
    let user1 = build_user(String::from("Admin"), String::from("admin@gmail.com"));
    let user2 = User {
        username: String::from("Ji, Yong-Hyeon"),
        email: String::from("hacker3740@gmail.com"),
        logged_in_count: 0,
        active: true,
    };

    // println!("{} {} {} {}",
    //           user.username,
    //           user.email,
    //           user.logged_in_count,
    //           user.active
    // );
    println!("{:?}", user1); // #[derive(Debug)]
    println!("{:?}", user2);
}

#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct Color(i32, i32, i32);

pub fn run_stroke() {
    let black = Color(0, 0, 0);
    let zero = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", zero);
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle {
            length,
            width,
        }
    }

    fn clone(&self) -> Rectangle {
        Rectangle {
            ..*self
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

pub fn run_stroke2() {
    let rect1 = Rectangle::new(10, 5);
    let mut rect2 = rect1.clone();
    rect2.set_width(25);

    println!("{:?}", rect1);
    println!("{:?}", rect2);
    
}