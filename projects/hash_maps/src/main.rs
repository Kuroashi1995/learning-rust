use std::collections::HashMap;
fn main() {
    let mut hm = HashMap::new();

    //Inserting into a hashmap
    hm.insert(String::from("Stark"), 100);
    hm.insert(String::from("Rogers"), 1);

    //Retrieving from a hashmap
    let petty = hm.get(&String::from("Rogers")).copied().unwrap_or(0);
    println!("The pettiest value is: {petty}");

    //Iterating over hasmap entries
    for (key, value) in &hm {
        println!("key: {key}, value: {value}");
    }

    //About ownership, Copy trait values are copied, and owned values are moved, and the hashmap
    //becomes their owner
    let field_name = String::from("Banner");
    let field_value = 100;

    hm.insert(field_name, field_value);

    println!("This should be valid: {field_value}");
    // println!("This shouldnt work: {field_name}");

    //Updating values in a hashmap

    //Overwriting a value
    //Hulk did not like Banner having a high score
    hm.insert(String::from("Banner"), 1);

    //Check if a value exists or write a default
    //Vision saw the system, and he wanted to take part, but respecting if a teamate already has
    //given him a value, so only insert his honest value if no value is present
    hm.entry(String::from("Vision")).or_insert(150);

    //Updating a value based on its current value
}

