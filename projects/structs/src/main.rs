struct Rectangle {
    width: u32,
    heigth: u32,
}
fn main() {

    let rectangle = Rectangle {
        width: 5,
        heigth: 10,
    };

    //This is the not using struct example
    // let width = 5;
    // let height = 10;


    println!("The area of the rectangle is {}", area_struct(&rectangle));
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigth
}
