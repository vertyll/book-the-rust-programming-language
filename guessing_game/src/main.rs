use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Zgadnij liczbę!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Wpisz swój typ.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Nie udało się odczytać linii");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Twój typ to: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Za mała liczba!"),
            Ordering::Greater => println!("Za duża liczba!"),
            Ordering::Equal => {
                println!("Zgadłeś!");
                break;
            }
        }
    }
}
