fn main() {
    // This is the one way we can implement loops in Rust
    let mut i = 1;
    loop {
        println!("{:?}", i);
        i = i + 1;
        if i > 4 {
            break;
        }
    }
    // Using the while loop
    let mut n = 1;
    while n <= 3 {
        println!("{:?}", n);
        n = n + 1;
    }
    // Activity
    let mut count = 5;
    while count > 0 {
        println!("{:?}", count);
        count = count - 1;
    }
    println!("done!");
    // Using for loop
    let arra = [1, 2, 3, 4, 5];
    let mut a = 0;
    for element in arra {
        println!("{:?}", arra[a]);
        a = a + 1;
    }
    // Using the reverse function to reverse the output
    for number in (1..5).rev() {
        // .rev() is used to reverse the oreder of the given numbers
        println!("{number}");
    }
}
