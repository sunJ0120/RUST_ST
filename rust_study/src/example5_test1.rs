#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn run(){
    let width = 30;
    let height = 50;

    let mut rect = Rectangle{
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    rect.set_width(40);

    println!(
        "Changing width {}",
        rect .area()
    );

    if rect.width() {
        println!("Width is non-zero");
    } else {
        println!("Width is zero");
    }

    println!("rect is {:#?}", rect);
    dbg!(&rect);
}