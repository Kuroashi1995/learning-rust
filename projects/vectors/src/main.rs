fn main() {
    //There are many ways to instanciate a vector.
    //Mutable empty vector
    let mut mutable_vector: Vec<i32> = Vec::new();

    //Inmutable vector initialized with values and macro
    let macro_vector = vec![1, 2, 3];

    //Updating a Vector
    //Push
    mutable_vector.push(1);
    mutable_vector.push(2);
    mutable_vector.push(3);

    //Reading data from a vector
    let second: &i32 = &macro_vector[1]; //will panic if out of bounds
    println!("Second value: {second}");

    let third: Option<&i32> = macro_vector.get(2); //will not panic if out of bounds, will return None

    match third {
        Some(third) => println!("Third value: {third}"),
        None => println!("No third value"),
    }

    //Also, when the program has a valid reference, it will enforce the rules of ownership and
    //borrowing
    let first: &i32 = &mutable_vector[0];

    //This line will panic because first is holding an inmutable reference
    // mutable_vector.push(4);

    println!("The first value is {first}");

    //Iterating over values in a Vector
    //Can be inmutable
    let iv1 = vec![1, 2, 3, 4];

    for i in &iv1 {
        println!("Vector value is {i}");
    }

    //Or mutable
    let mut iv2 = vec![5, 6, 7];

    for i in &mut iv2 {
        *i += 50;
    }

    for new_val in &iv2 {
        println!("New value: {new_val}");
    }

    //Using enums to store multiple types of values
    //Technically an enum is considered one type of value, so the next is valid
    enum Many_Types {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let variable_vector = vec![
        Many_Types::Int(1),
        Many_Types::Float(3.24),
        Many_Types::Text(String::from("lesgooo"))
    ];

}
