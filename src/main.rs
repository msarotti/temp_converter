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
    loop {
        let mut degree = String::new();
        println!("Please insert temperature: (type exit to return)");
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line");
        if "exit".eq(degree.trim()) {
            return
        }
        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number");
                continue
            }
        };
    
        let to_celsius: f32 = ((degree - 32.0) * 5.0) / 9.0;
        println!("Celsiuse degreees: {}", to_celsius);
    }
}

fn celsius_to_fahrenheit() {
    loop {
        let mut degree = String::new();
        println!("Please insert temperature: (type exit to return)");
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line");
        if "exit".eq(degree.trim()) {
            return
        }
        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number");
                continue
            }
        };
    
        let to_fahrenheit: f32 = (degree * 1.8) + 32.0;
        println!("Fahrenheit degreees: {}", to_fahrenheit);
    }
}
