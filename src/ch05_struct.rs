
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

pub fn run() {
    let rect = Rectangle {
        height: 50,
        width: 30
    };

    println!("Area of {:?} : {}", &rect, rect.area())
}