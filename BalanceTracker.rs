use std::io::{self, Read};

fn main() {
    let mut balance = 10000;

    loop {
        let mut inp = String::new(); // Clear input buffer at the start
        println!("What would you like to do? \n1. View balance \n2. Deposit \n3. Withdraw \n4. Exit");
        io::stdin().read_line(&mut inp).expect("Error reading the value");
        let choice: u32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Current Balance = {}", balance);
            }
            2 => {
                let mut amount_input = String::new();
                println!("Enter amount you want to deposit: ");
                io::stdin().read_line(&mut amount_input).expect("Error reading the value");
                let amount: i32 = match amount_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
                balance += amount;
                println!("Deposited {} successfully!", amount);
            }
            3 => {
                let mut amount_input = String::new();
                println!("Enter amount you want to withdraw: ");
                io::stdin().read_line(&mut amount_input).expect("Error reading the value");
                let amount: i32 = match amount_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };

                if amount > balance {
                    println!("Insufficient Balance!");
                } else {
                    balance -= amount;
                    println!("Withdrawn {} successfully!", amount);
                }
            }
            4 => {
                println!("Thank you for using our service!");
                break;
            }
            _ => {
                println!("Enter a valid number!!");
            }
        }
    }
}
