use std::io::Write;

fn main() {
    let commissioners: [&str; 5] = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries: [&str; 5] = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones: [&str; 5] = ["South West", "North East", "South South", "South West", "South East"];

    let mut file = std::fs::File::create("merged.csv").expect("create failed");
    file.write_all("Commissioner,Ministry,Zone\n".as_bytes()).expect("write failed");
    for index in 0..5 {
        file.write_all(commissioners[index].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(ministries[index].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(zones[index].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }
    println!("Data written to file.");
}