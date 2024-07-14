#[derive(Debug, Clone, Copy)]
enum Position {
    Superviser,
    Worker,
}
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}
fn print_emp(emp: Employee) {
    println!("{:?}", emp);
}
fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    println!("{:?}", me.position);
    print_emp(me);
    print_emp(me);
    //    match me.position {
    //        Position::Worker => println!("Worker"),
    //        Position::Superviser => println!("Superviser"),
    //    }
}
