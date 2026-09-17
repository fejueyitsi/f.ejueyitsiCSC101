// Rust program to calculate employee annual incentive

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let is_experienced = input1.trim().to_lowercase();

    let incentive: i32;

    if is_experienced == "yes" {
        println!("Enter employee age: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let age: i32 = input2.trim().parse().expect("Not a valid number");

        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            println!("No specific tier for this age group under experienced status.");
            return;
        }
    } else {
        incentive = 100_000;
    }

    println!("The annual incentive for the employee is: N{}", incentive);
}