struct Box {
    depth: i32,
    heiht: i32,
    width: i32,
}

fn main() {
    let billu = Box {
        depth: 3,
        heiht: 23,
        width: 43,
    };
    let pall = billu.heiht;
    println!("The box is {:?} units wide", pall);
}
