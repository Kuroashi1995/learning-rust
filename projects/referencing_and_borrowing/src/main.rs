fn main() {
    let mut s1 = String::from("hello");
    // let len = get_len(&s1); //This is code intended to show borrowing
    //println!("The string '{s1}' has a length of {len}");


    change_something(&mut s1); // This is code intended to show mutability of references

    println!("the string is: '{s1}'")

}

// fn get_len(some_str: &String) -> usize {
//     some_str.len()
// }

fn change_something(str: &mut String) {
    str.push_str(", world!");
}
