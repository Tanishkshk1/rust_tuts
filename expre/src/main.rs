// Rust is an expression based language
// can be heavily used for nesting logic
fn print_ms(t_100: bool) {
    match t_100 {
        true => println!("Its bi"),
        false => println!("Its not bi"),
    }
}
fn main() {
    let value = 100;
    let is_t_100 = value > 100;
    print_ms(is_t_100);
}
//fn is_big_enough(a: i32) {
//    let is_not_bi = if a < 100 { true } else { false };
//    if is_not_bi {
//        println!("Its not that bi");
//    } else {
//        println!("its bi");
//    }
//}
//fn main() {
//    let b = 10;
//    is_big_enough(b);
//}
//enum Access {
//    Admin,
//    Manager,
//    User,
//}
//
//fn main() {
//    //Secret files admin only
//    let access_level = Access::User;
//    let can_access = match access_level {
//        Access::Admin => true,
//        _ => false,
//    };
//    println!("can Access {:?}", can_access);
//}
//fn main() {
//    let number = 3;
//    let is_5 = if number > 5 { true } else { false };
//    //let is_5 = number < 5;
//    println!("{:?}", is_5);
//}
