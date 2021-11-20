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
            1 => fahrenheit_to_celsius(),
            2 => println!("Celsius to Fahrenheit!"),
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid Option, please retry"),
        }
    }
    
}

fn fahrenheit_to_celsius() {
    let mut degree = String::new();
    println!("Please insert temperature: (r to return)");
    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read line");
    // if degree == "r" {
    //     break;
    // }
    let degree: f32 = match degree.trim().parse() {
        Ok(num) => num,
        Err(_) => return
    };

    let to_celsius: f32 = ((degree - 32.0) * 5.0) / 9.0;
    println!("Celsiuse degreees: {}", to_celsius);
}
