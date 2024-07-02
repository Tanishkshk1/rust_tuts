use rand::Rng;

fn main() {
    println!("hello world");
    let t_new = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is :{t_new}");
}
