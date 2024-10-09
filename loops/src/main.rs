fn main() {
    // let mut counter = 0;
    //
    // 'counting_up: loop {
    //     println!("Licznik: {counter}");
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("Pozostało: {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //
    //         if counter == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     counter += 1;
    // }

    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("Wynik to {result}");

    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{number}");
    //     number -= 1;
    // }
    //
    // println!("Start!");

    // let a = [1, 2, 3, 4, 5];
    //
    // // let mut index = 0;
    // //
    // // while index < 5 {
    // //     println!("Wartość to: {}", a[index]);
    // //
    // //     index += 1;
    // // }
    //
    // for element in a {
    //     println!("Ta wartość to: {element}");
    // }

    for number in (1..4).rev() {
        println!("Number: {number}");
    }

    println!("Start!");
}
