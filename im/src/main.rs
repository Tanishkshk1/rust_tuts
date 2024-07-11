enum Color {
    Brown,
    Red,
}
impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Brown"),
            Color::Red => println!("red"),
        }
    }
}
struct Dimensions {
    width: i32,
    heiht: i32,
}
impl Dimensions {
    fn print(&self) {
        println!("width:{:?}", self.width);
        println!("heiht:{:?}", self.heiht);
    }
}
struct Box {
    dimensions: Dimensions,
    colors: Color,
    weiht: i32,
}
impl Box {
    fn new(weiht: i32, colors: Color, dimensions: Dimensions) -> Self {
        Self {
            weiht,
            dimensions,
            colors,
        }
    }
    fn print(&self) {
        self.colors.print();
        self.dimensions.print();
        println!("weiht{:?}", self.weiht);
    }
}
fn main() {
    let small_dimensions = Dimensions {
        width: 43,
        heiht: 78,
    };
    let small_box = Box::new(30, Color::Brown, small_dimensions);
    small_box.print();
}

//#[derive(Debug)]
//struct Temprature {
//    deree_f: f64,
//}
//impl Temprature {
//    fn boilin() -> Self {
//        Self { deree_f: 88.99 }
//    }
//    fn freezin() -> Self {
//        Self { deree_f: 44.4 }
//    }
//
//    fn show_temp(&self) {
//        println!("{:?}", self.deree_f); // self in rust refers to the self which in case of this
//                                        // function refers to Temprature
//    }
//}
//fn main() {
//    //let hot = Temprature { deree_f: 33.33 };
//    //hot.show_temp();
//    let cold = Temprature::freezin();
//    cold.show_temp();
//    let hot = Temprature::boilin();
//    hot.show_temp();
//}
