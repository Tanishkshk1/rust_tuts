// This enum is created to make your own type in rust
enum Direction {
    Up,
    Down,
}
enum color {
    Red,
    Yellow,
    Blue,
}

fn print_color(my_color: color) {
    match my_color {
        color::Red => println!("red"),
        color::Yellow => println!("yellow"),
        color::Blue => println!("blue"),
    }
}
//fn which_way(go: Direction) {
//    match go {
//        Direction::Up => "up",
//        Direction::Down => "down",
//    };
//}
fn main() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("go up"),
        Direction::Down => println!("go down"),
    }
    print_color(color::Red);
}
