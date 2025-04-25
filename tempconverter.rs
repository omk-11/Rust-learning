use std::io;

fn convert(num: u8, temp: f64) {
    match num {
        1 => {
            let result = (temp * 9.0/5.0) + 32.0;
            println!("{}°C -> {}°F", temp, result);
        },
        2 => {
            let result = temp + 273.15;
            println!("{}°C -> {}K", temp, result);
        },
        3 => {
            let result = temp - 273.15;
            println!("{}K -> {}°C", temp, result);
        },
        4 => {
            let result = (temp - 273.15) * 9.0/5.0 + 32.0;
            println!("{}K -> {}°F", temp, result);
        },
        5 => {
            let result = (temp - 32.0) * 5.0/9.0; 
            println!("{}°F -> {}°C", temp, result);
        },
        6 => {
            let result = (temp - 32.0) * 5.0/9.0 + 273.15; 
            println!("{}°F -> {}K", temp, result);
        },
        _ => println!("Enter a valid choice"),
    }
}

fn main() {
    let mut inp = String::new();
    println!("Choose the conversion:");
    println!("1. Celsius -> Fahrenheit");
    println!("2. Celsius -> Kelvin");
    println!("3. Kelvin -> Celsius");
    println!("4. Kelvin -> Fahrenheit");
    println!("5. Fahrenheit -> Celsius");
    println!("6. Fahrenheit -> Kelvin");
    
    print!("Enter your choice (1-6): ");
    io::stdin().read_line(&mut inp).expect("Error reading input");
    let choice = inp.trim().parse().expect("Enter a valid number");

    inp.clear();
    print!("Enter temperature: ");
    io::stdin().read_line(&mut inp).expect("Error reading input");
    let temp = inp.trim().parse().expect("Enter a valid number");

    convert(choice, temp);
}
