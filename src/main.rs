use std::io;

fn main() {
    println!("Welcome to temperature converter!");
    loop {
        println!("Please insert the mode: \n
            1 -> Fahrenheit to Celsius,
            2 -> Celsius to Fahrenheit,
            3 -> Exit the program,
        ");
        let mut prg_mode = String::new();

        io::stdin()
            .read_line(&mut prg_mode)
            .expect("Failed to read line");
        
        let prg_mode: u32 = match prg_mode.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match prg_mode {
            1 => println!("Fahrenheit to Celsius!"),
            2 => println!("Celsius to Fahrenheit!"),
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid Option, please retry"),
        }
    }
    
}
