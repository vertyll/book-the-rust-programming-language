use std::collections::HashMap;

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v2 = vec![1, 2, 3];
    //
    // let v3 = vec![1, 2, 3];
    //
    // let third: Option<&i32> = v3.get(2);
    // match third {
    //     Some(third) => println!("The third element id: {third}"),
    //     None => println!("There is not third element"),
    // };

    // let mut v = vec![100, 32, 57];
    //
    // for i in &mut v {
    //     *i += 50;
    //     println!("{i}");
    // }
    //
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Float(10.12),
    //     SpreadsheetCell::Text(String::from("niebieski")),
    // ];
    //
    // let mut s = String::new();
    //
    // let data = "zawartość początkowa";
    // let string = data.to_string();

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is: {s2}");

    // let s1 = String::from("Witaj, ");
    // let s2 = String::from("świecie!");
    // let s3 = s1 + &s2; // Zwróć uwagę, że zmienna s1 została przeniesiona i nie może być już używana
    //
    // println!("{s2}");
    //
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{s1}-{s2}-{s3}");
    // let s1 = String::from("Witaj");
    // let h = s1[0];

    // for c in "Зд".chars() {
    //     println!("{c}");
    // }
    //
    // for b in "Зд".bytes() {
    //     println!("{b}");
    // }

    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("niebiescy"), 10);
    // scores.insert(String::from("żółci"), 50);
    //
    // // let team_name = String::from("niebiescy");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);
    //
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Niebiescy"), 10);
    //
    // scores.entry(String::from("Żółci")).or_insert(50);
    // scores.entry(String::from("Niebiescy")).or_insert(25);
    //
    // println!("{:?}", scores);

    let text = "witaj świecie cudowny świecie";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
