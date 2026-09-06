fn main() {
    let p: f64 = 210_000.0; // Initial cost of the TV (N210,000)
    let r: f64 = 5.0;       // Depreciation rate (5% per annum)
    let n: f64 = 3.0;       // Time in years (3 years)

    // Depreciation Formula: A = P * [1 - (R / 100)]^n
    let a = p * (1.0 - (r / 100.0)).powf(n);

    println!("The value of the TV after 3 years is: N{}", a);
}