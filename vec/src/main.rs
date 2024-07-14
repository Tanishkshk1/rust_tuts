fn main() {
    let num = vec![10, 20, 30, 40];
    for element in &num {
        match element {
            30 => println!("Thirty"),
            _ => println!("{:?}", element),
        }
    }
    println!("{:?}", &num.len());
}
//#[derive(Debug)]
//struct Test {
//    score: i32,
//}
//fn main() {
//    let my_num = vec![1, 2, 3];
//
//    for element in my_num {
//        println!("{:?}", element);
//    }
//
//    let mut my_num4 = Vec::new();
//    my_num4.push(12);
//
//    let my_score = vec![
//        Test { score: 92 },
//        Test { score: 42 },
//        Test { score: 52 },
//        Test { score: 62 },
//    ];
//
//    for test in my_score {
//        println!("{:?}", test.score);
//    }
//}
