use std::io;

fn main() {
    // let mut x = 5;
    // println!("Wartość x to: {x}");
    // x = 6;
    // println!("Wartość x to: {x}");

    // let x = 5;
    //
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("Wartość x w zakresie wewnętrznym to: {x}");
    // }
    //
    // println!("Wartość x to: {x}");

    // let spaces = "  ";
    // let _spaces = spaces.len();

    // let x = 2.0;
    //
    // let y: f32 = 3.0;
    //
    // let t = true;
    //
    // let f: bool = false;
    //
    // let c = 'z';
    //
    // let z: char = 'Z';
    //
    // let heart_eyed_cat = '😻';

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    // let (x, y, z) = tup;
    //
    // println!("Wartość y to {y}");
    //
    // let one = tup.2;

    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    //
    // let b = [3;5];
    //
    // let first = a[0];
    // let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Wprowadź index tablicy.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Nie udało się odczytać linii!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Wprowadzony index nie był liczbą!");

    let element = a[index];

    println!("Wartość elementu z indeksu: {index} wynosi: {element}");
}
