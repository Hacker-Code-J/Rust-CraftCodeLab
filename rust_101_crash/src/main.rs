enum Light {
	Bright,
	Dull,
}

fn display_light(light: &Light) {
	match light {
		Light::Bright => println!("bright"),
		Light::Dull => println!("dull"),
	}
}

fn main() {
	// let dull = Light::Dull;
	// display_light(&dull);
	// display_light(&dull);

	let my_num = vec![1, 2, 3];

	for num in my_num {
		println!("{:?}", num);
	}
}


// // enum Direction {
// // 	Left,
// // 	Right,
// // 	Up,
// // }

// // fn main() {
// // 	let go = Direction::Left;
// // 	match go {
// // 		Direction::Left => println!("go left"),
// // 		Direction::Right => println!("go right"),
// // 		Direction::Up => println!("go up"),
// // 	}
// // }

// enum Color {
// 	Red,
// 	Yellow,
// 	Blue
// }

// fn print_color(my_color: Color) {
// 	match my_color {
// 		Color::Red  => println!("red"),
// 		Color::Yellow  => println!("yellow"),
// 		Color::Blue  => println!("blue"),
// 	}
// }



// fn main() {
// 	let a: i32;
// 	a = 0x22441155;
// 	println!("{a:?}");
// 	print_color(Color::Blue);
// }
