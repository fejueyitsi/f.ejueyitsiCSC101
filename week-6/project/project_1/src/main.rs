use std::io;

fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().parse().expect("Invalid input");
}

fn get_pi() -> f64 {
    return 22.0 / 7.0;
}

fn trapezium() -> f64 {
    let height = read_f64("Enter height:");
    let base1 = read_f64("Enter base 1:");
    let base2 = read_f64("Enter base 2:");
    return height / 2.0 * (base1 + base2);
}

fn rhombus() -> f64 {
    let d1 = read_f64("Enter diagonal 1:");
    let d2 = read_f64("Enter diagonal 2:");
    return 0.5 * d1 * d2;
}

fn parallelogram() -> f64 {
    let base = read_f64("Enter base:");
    let altitude = read_f64("Enter altitude:");
    return base * altitude;
}

fn cube() -> f64 {
    let side = read_f64("Enter side:");
    return 6.0 * side * side;
}

fn cylinder() -> f64 {
    let radius = read_f64("Enter radius:");
    let height = read_f64("Enter height:");
    return get_pi() * radius * radius * height;
}

fn main() {
    println!("=== Shape Calculator ===");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");

    let choice = read_f64("Enter your choice (1-5):") as i32;

    if choice == 1 {
        println!("Area of trapezium: {}", trapezium());
    } else if choice == 2 {
        println!("Area of rhombus: {}", rhombus());
    } else if choice == 3 {
        println!("Area of parallelogram: {}", parallelogram());
    } else if choice == 4 {
        println!("Surface area of cube: {}", cube());
    } else if choice == 5 {
        println!("Volume of cylinder: {}", cylinder());
    } else {
        println!("Invalid choice. Please pick 1 to 5.");
    }
}