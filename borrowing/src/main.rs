fn main() {
    // let s1 = String::from("Witaj");
    //
    // let len = calculate_length(&s1);
    //
    // println!("Długość '{}' wynosi {}.", s1, len);

    // let mut s = String::from("Witaj");
    //
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // // change(&mut s);
    //
    // println!("{}, {}, i {}", r1, r2, r3);

    let _reference_to_nothing = dangle();
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(s: &mut String) {
//     s.push_str(" , świecie");
// }

fn dangle() -> String {
    let s = String::from("witaj");

    s
}