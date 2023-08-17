
// #[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
fn print_rect( rc : &Rectangle ) {
    println!("{:?}", rc);
}

#[allow(dead_code)]
fn area(rc : &Rectangle) -> u32 {
    rc.width * rc.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

pub fn rect_test() {
    let rc = Rectangle::square(30);
    // print_rect(&rc);
    // println!("Area {}", area(&rc));
    println!("Area {}", rc.area());
}
