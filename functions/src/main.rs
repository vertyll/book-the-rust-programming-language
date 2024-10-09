// fn five() -> i32 {
//     5
// }

fn main() {
    // println!("Hello, world!");
    //
    // another_function(5);
    // let y = {
    //    let x = 3;
    //     x + 1
    // };
    //
    // println!("Wartość y to: {y}");

    // let x = five();
    // println!("Wartość x to: {x}");

    let x = plus_one(5);
    println!("Wartość x to: {x}");
}

// fn another_function(x: i32) {
//     println!("Wartość x to: {x}");
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}
