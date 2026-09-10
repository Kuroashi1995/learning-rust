fn main() {
    let width = 5;
    let height = 10;


    println!("The area of the rectangle is {}", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
