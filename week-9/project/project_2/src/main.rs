use std::io::Write;

fn main() {
    let students: [&str; 5] = [
        "Oluchi Mordi,ACC10211111,Accounting,300",
        "Adams Aliyu,ECO10110101,Economics,100",
        "Shania Bolade,CSC10328828,Computer,200",
        "Adekunle Gold,EEE11020202,Electrical,200",
        "Blanca Edemoh,MEE10202001,Mechanical,100",
    ];

    let mut file = std::fs::File::create("students.csv").expect("create failed");
    file.write_all("Student Name,Matric. Number,Department,Level\n".as_bytes()).expect("write failed");
    for index in 0..5 {
        file.write_all(students[index].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    println!("Data written to file.");
}