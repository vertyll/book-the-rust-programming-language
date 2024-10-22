// use std::fs;
// use std::fs::File;
// use std::io;
// use std::io::ErrorKind;

// fn main() {
//     // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//     //     if error.kind() == ErrorKind::NotFound {
//     //         File::create("hello.txt").unwrap_or_else(|error| {
//     //             panic!("Problem z utworzeniem pliku: {:?}", error);
//     //         })
//     //     } else {
//     //         panic!("Problem z otwarciem pliku: {:?}", error);
//     //     }
//     // });

//     // let greeting_file =
//     //     File::open("hello.txt").expect("hello.txt powinien być załączony w projekcie");

//     // fn read_username_from_file() -> Result<String, io::Error> {
//     //     let mut username_file = File::open("hello.txt")?;
//     //     let mut username = String::new();
//     //     username_file.read_to_string(&mut username)?;
//     //     Ok(username)
//     // }

//     // fn read_username_from_file() -> Result<String, io::Error> {
//     //     let mut username = String::new();
//     //     File::open("hello.txt")?.read_to_string(&mut username)?;
//     //     Ok(username)
//     // }
//     //
//     fn read_username_from_file() -> Result<String, io::Error> {
//         fs::read_to_string("hello.txt")
//     }
// }

use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
