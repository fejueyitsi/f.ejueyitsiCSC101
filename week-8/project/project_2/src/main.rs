use std::io;

fn main() {
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates are you interviewing?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let total: usize = input1.trim().parse().expect("Invalid input");

    if total == 0 {
        println!("No candidates to check.");
        return;
    }

    for count in 0..total {
        let mut name_input = String::new();
        println!("\nEnter name of candidate {}", count + 1);
        io::stdin().read_line(&mut name_input).expect("Failed to read input");
        let name: String = name_input.trim().parse().expect("Invalid input");

        let mut years_input = String::new();
        println!("Enter years of programming experience for {}", name);
        io::stdin().read_line(&mut years_input).expect("Failed to read input");
        let years: u32 = years_input.trim().parse().expect("Invalid input");

        candidates.push((name, years));
    }

    let mut best = 0;
    for i in 1..candidates.len() {
        if candidates[i].1 > candidates[best].1 {
            best = i;
        }
    }

    println!("\nHighest experience: {} with {} years", candidates[best].0, candidates[best].1);
}
