use std::io;

fn operation(op: &str){
    let mut inp = String::new();

    println!("Enter a : ");
    io::stdin().read_line(&mut inp).expect("failed to read line");
    let a: i64 = inp.trim().parse().expect("Enter a valid number");

    inp.clear();
    println!("enter b : ");
    io::stdin().read_line(&mut inp).expect("failed to read line");
    let b: i64 = inp.trim().parse().expect("please enter a valid number");
    
    match op {
        "add" => println!("Answer a+b = {}",a+b),
        "sub" => println!("Answer a-b = {}",a-b),
        "mul" => println!("Answer a*b = {}",a*b),
        "div" => {
            if b == 0 {
                println!("Error: Division by zero!");
            } else {
                println!("Answer a/b = {}", a / b);
            }
        }
        _ => {
            println!("Invalid input! Please choose 1-5.");
        }
    }
}

fn main() {
    loop{
        let mut input = String::new();
        println!("choose an operation : \n1. add\n2.subtract\n3.multiply\n4.divide\n5.Exit");

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        match choice {
            1 => operation("add"),
            2 => operation("sub"),
            3 => operation("mul"),
            4 => operation("div"),
            5 => {break;},
            0 => {println!("Please enter a valid number!!")},
            
            _ =>{
                println!("invalid input");
                return;
            }
        }
}

}