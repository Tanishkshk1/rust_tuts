#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}
fn main() {
    let mary = Student {
        name: "Mary".to_owned(),
        locker: Some(12),
    };
    println!("The name of the student is : {:?}", mary.name);
    match mary.locker {
        Some(no) => println!("The locker no is : {:?}", mary.locker),
        None => println!("The locker no is not provided"),
    }
}
//// Options has two thin first is some and the other is none
//// some meanin that the data is provided and none means that no data is provided
//#[derive(Debug)]
//struct Survey {
//    q1: Option<i32>,
//    q2: Option<bool>,
//    q3: Option<String>,
//}
//fn main() {
//    let respinse = Survey {
//        q1: Some(12),
//        q2: Some(true),
//        q3: Some("Aa".to_owned()),
//    };
//    match respinse.q1 {
//        Some(ans) => println!("q1:{:?}", ans),
//        None => println!("No response"),
//    }
//    match respinse.q2 {
//        Some(ans) => println!("q2:{:?}", ans),
//        None => println!("No response"),
//    }
//    match respinse.q3 {
//        Some(ans) => println!("q3:{:?}", ans),
//        None => println!("No response"),
//    }
//}
