use std::{fs::File, io::{self, ErrorKind, Read}};

fn main() {
    let super_secret_info_result = File::open("public.txt");

    let super_secret_info = match super_secret_info_result {
        Ok(file) => file,
        // Err(error) => panic!("Error accesing very public and available information: {error:?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("public.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating very public file: {e:?}")
            },
            _ => {
                panic!("Error accessing very public and available information")
            }
        }
    };
}

fn read_user_from_file() -> Result<String, io::Error> {
    // let username_result = File::open("users.txt");
    //
    // let mut username_file = match username_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    let mut username = String::new();
    File::open("users.txt")?.read_to_string(&mut username)?;
    return Ok(username);
}
