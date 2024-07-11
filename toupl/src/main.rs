//fn ret_toup() -> (String, String, String) {
//    (
//        "greater than 5".to_string(),
//        "Less than 5".to_string(),
//        "equal to 5".to_string(),
//    )
//}
fn cordinates() -> (i32, i32) {
    (1, 3)
}
fn main() {
    let (x, y) = cordinates();
    //let number = 3;
    //let output = ret_toup();
    if y < 5 {
        println!("the value id less than 5");
    } else if y > 5 {
        println!("the value is greater than 5");
    } else {
        println!("the value is equal to 5");
    }
}
//fn main() {
//    let cord = (2, 3);
//    println!("{:?},{:?}", cord.0, cord.1); // this can be used to print only one of the values in
//                                           // the touple
//    let user_info = ("Emma Watson", 20);
//    println!("{:?}", user_info.1);
//}

//enum Access {
//    Full,
//}
//fn one_la() -> (i32, i32, i32) {
//    (1, 2, 3)
//}
//fn main() {
//    let numbers = one_la();
//    let (x, y, z) = one_la();
//    println!("{:?},{:?}", x, numbers.0);
//    println!("{:?},{:?}", y, numbers.1);
//    println!("{:?},{:?}", z, numbers.2);
//    let (employee, access) = ("Jake", Access::Full);
//}
