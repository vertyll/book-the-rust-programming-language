use std::io;

fn main() {

    loop {
        println!("Program do konwersji temperatur");

        let mut temperature = String::new();

        println!("Podaj temperaturę w stopniach Celcjusza");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Nie udało się odczytać linii");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let result = (temperature * 1.8) + 32.0;

        println!("Wybrana temperatura w stopniach Fahrenheita to {result}");

        break;
    }

        // println!("Program do konwersji temperatur");
        //
        // let mut temperature = String::new();
        //
        // println!("Podaj temperaturę w stopniach Celcjusza");
        //
        // io::stdin()
        //     .read_line(&mut temperature)
        //     .expect("Nie udało się odczytać linii");
        //
        // let temperature: f64 = temperature.trim().parse().expect("Nie udało się skonwertować wartości");
        // let result = (temperature * 1.8) + 32.0;
        //
        // println!("Wybrana temperatura w stopniach Fahrenheita to {result}");
}
