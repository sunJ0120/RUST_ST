#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run(){
    let width = 30;
    let height = 50;

    // let rect1 = (width, height);
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width, height)
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}