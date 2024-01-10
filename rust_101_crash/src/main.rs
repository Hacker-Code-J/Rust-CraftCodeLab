// enum Direction {
// 	Left,
// 	Right,
// 	Up,
// }

// fn main() {
// 	let go = Direction::Left;
// 	match go {
// 		Direction::Left => println!("go left"),
// 		Direction::Right => println!("go right"),
// 		Direction::Up => println!("go up"),
// 	}
// }

enum Color {
	Red,
	Yellow,
	Blue
}

fn print_color(my_color: Color) {
	match my_color {
		Color::Red  => println!("red"),
		Color::Yellow  => println!("yellow"),
		Color::Blue  => println!("blue"),
	}
}

fn main() {
	print_color(Color::Blue);
}