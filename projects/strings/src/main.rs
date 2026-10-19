fn main() {
    //Strings are just vectors with more features and constraints
    //Similarly to vectors we can declare a string with
    let mut s = String::new(); //Empty string

    s.push_str("Hello, "); //pushes a string into the existing variable

    s.push('W');
    s.push('o');
    s.push('r');
    s.push('l');
    s.push('d');
    s.push('!'); //pushes one char at the time

    //Strings can also be concatenated
}
