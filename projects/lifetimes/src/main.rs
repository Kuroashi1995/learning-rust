// This implementation fails because the borrow checker cannot determine either if x or y reference
// is returned
// fn longest_string(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let longest = longest_string(string1.as_str(), string2);
    println!("longest string: {longest}");
}
