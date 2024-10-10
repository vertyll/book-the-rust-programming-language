fn main() {
    let s = String::from("Witaj");
    let x = 2;

    let s2 = takes_ownership(s);

    let (s3, len) = calculate_length(s2);

    makes_copy(x);

    // println!("s: {s2}");
    println!("x: {x}");
    println!("s3: {s3}, len {len}");
}

fn takes_ownership(some_string: String) -> String {
    println!("{some_string}");
    some_string
}

fn makes_copy(x: i32) {
    println!("{x}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}