use std::io;

pub struct Fahrenheit { }

impl Fahrenheit {
    pub fn new() -> Self {
        Fahrenheit {  }
    }

    pub fn to_celsius(&self) -> () {
        let mut degree = String::new();
        println!("Please insert temperature: (type exit to return)");
        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line");
        if "exit".eq(degree.trim()) {
            return;
        }
        let degree: f32 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid number");
                return;
            }
        };

        let to_celsius: f32 = ((degree - 32.0) * 5.0) / 9.0;
        println!("Celsiuse degreees: {}", to_celsius);
    }
}