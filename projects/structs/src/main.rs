struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn constructor(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }
}
fn main() {

    let r1 = Rectangle::constructor(5, 10);

    let r2 = Rectangle::constructor(10, 15);

    let r3 = Rectangle::constructor(2, 4);


    println!("r1 has an area of {}", r1.area());
    println!("Can r1 hold r2?: {}", r1.can_hold(&r2));
    println!("Can r1 hold r3?: {}", r1.can_hold(&r3));

    //This is the not using struct example
    // let width = 5;
    // let height = 10;

}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area_struct(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.heigth
// }
