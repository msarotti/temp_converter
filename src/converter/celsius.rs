use std::io;

pub struct Celsius {}

impl Celsius {
    pub fn new() -> Self {
        Celsius {  }
    }

    pub fn to_fahrenheit(&self) -> () {
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
        let to_fahrenheit: f32 = (degree * 1.8) + 32.0;
        println!("Fahrenheit degreees: {}", to_fahrenheit);
    }
}