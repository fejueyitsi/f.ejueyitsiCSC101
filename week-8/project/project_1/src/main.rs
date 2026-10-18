use std::io;

fn main() {
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

    println!("Choose a career:");
    println!("1 - Office Admin\n2 - Academic\n3 - Lawyer\n4 - Teacher");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice: usize = input1.trim().parse().expect("Invalid input");

    // Step 2: pick the matching vector
    let titles: Vec<&str>;
    if choice == 1 {
        titles = office_admin;
    } else if choice == 2 {
        titles = academic;
    } else if choice == 3 {
        titles = lawyer;
    } else if choice == 4 {
        titles = teacher;
    } else {
        println!("Invalid career choice");
        return;
    }

    println!("\nChoose a position:");
    for i in 0..titles.len() {
        println!("{} - {}", i + 1, titles[i]);
    }

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let position: usize = input2.trim().parse().expect("Invalid input");

    if position < 1 || position > titles.len() {
        println!("Invalid position number");
        return;
    }
    let index = position - 1;

    if titles[index] == "-" {
        println!("There is no academic position at this level.");
    } else {
        println!("\n{} holds position {}", titles[index], levels[index]);
    }
}