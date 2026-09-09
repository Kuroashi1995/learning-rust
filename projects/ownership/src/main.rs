// Double free error Rust handling
fn main() {
    let s1 = String::from("hello");
    // this is called a move, s1 moved to s2
    let s2 = s1;

    println!("{s1}, world!");
}


// Basic example
// fn main() {
//     // We initialize a string with a namespace (::) for the from method.
//     let mut s = String::from("hello");
//
//     // Here this s value gets mutated
//     s.push_str(", world!");
//
//     // This prints "hello, world!"
//     println!("{s}")
// }
