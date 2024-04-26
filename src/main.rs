use converter::celsius::Celsius;
use converter::fahrenheit::Fahrenheit;
use std::io;

mod converter;

fn main() {
    println!("\n(°--- Welcome To Temperature Converter ---°)\n");
    loop {
        println!(
            "Please insert the mode:
            1 -> Fahrenheit to Celsius,
            2 -> Celsius to Fahrenheit,
            3 -> Exit the program,
        "
        );
        let mut prg_mode = String::new();

        io::stdin()
            .read_line(&mut prg_mode)
            .expect("Failed to read line");

        let prg_mode: u32 = match prg_mode.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match prg_mode {
            1 => fahrenheit_to_celsius(),
            2 => celsius_to_fahrenheit(),
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid Option, please retry"),
        }
    }
}

fn fahrenheit_to_celsius() {
    let fahrenheit = Fahrenheit::new();
    loop {
        fahrenheit.to_celsius();
    }
}

fn celsius_to_fahrenheit() {
    let celsius = Celsius::new();
    loop {
        celsius.to_fahrenheit();
    }
}
