#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
    fn can_hold(self: &Self, another_self: &Self) -> bool {
        self.area() > another_self.area()
    }
}

fn main() {
    /*
    let width1 = 30;
    let height1 = 50;
    let rect = Rectangle {
        width: dbg!(width1 * 2),
        height: height1,
    };

    println!("The are of the rectangle is {} square pixels.", rect.area());
    println!("rect is: {rect:#?}");
    dbg!(&rect);

    if rect.width() {
        println!("The rectagle has a nonzero width; it is {}", rect.width);
    }
    */
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);
    let _sq1 = Rectangle::square(50);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
