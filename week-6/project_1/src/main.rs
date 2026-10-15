use std::io;

fn main() {
    println!("Good day Ma/Sir welcome to Faith's Kitchen");
    println!("Here is our menu");
    println!("code      Food                       Price");
    println!("P         Poundo Yam/Edinkaiko Soup  ₦3,200");
    println!("F         Fried Rice & Chicken       ₦3,000");
    println!("A         Amala & Ewedu Soup         ₦2,500");
    println!("E         Eba & Egusi Soup           ₦2,000");
    println!("W         White Rice & Stew          ₦2,500");

    let mut grand_total: u32 = 0;

    loop {
    println!("Choose a food letter (or D when done):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Could not read");
    let food = food.trim().to_uppercase();

     if food == "D" {
        break;
    }

    let mut price: u32 = 0;
    if food == "P" {
        price = 3200;
    } else if food == "F" {
        price = 3000;
    } else if food == "A" {
        price = 2500;
    } else if food == "W" {
        price = 2500;
    } else if food == "E" {
        price = 2000;
    }

        if price == 0 {
        println!("Not on the menu, try again");
        continue;
    }

      println!("How many?");
    let mut count_text = String::new();
    io::stdin().read_line(&mut count_text).expect("Could not read");
    let count: u32 = count_text.trim().parse().expect("Please type a number");

    grand_total = grand_total + price * count;
    println!("Running total: {}", grand_total);
}

let mut final_total = grand_total;

if grand_total > 10000 {
    let discount = grand_total / 20;
    final_total = grand_total - discount;
    println!("Discount: {}", discount);
}

println!("Amount to pay: {}", final_total);
}
