use std::io;

fn main() {
    println!("Zgadnij liczbę!");

    println!("Wpisz swój typ.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Nie udało się odczytać linii");

    println!("Twój typ to: {}", guess);
}
