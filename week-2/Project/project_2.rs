fn main() {
    let toshiba: f64 = 450_000.00;
    let mac: f64 = 1_500_000.00;
    let hp: f64 = 750_000.00;
    let dell: f64 = 2_850_000.00;
    let acer: f64 = 250_000.00;

    let qty_toshiba: f64 = 2.0;
    let qty_mac: f64 = 1.0;
    let qty_hp: f64 = 3.0;
    let qty_dell: f64 = 3.0;
    let qty_acer: f64 = 1.0;

    let sum = (toshiba * qty_toshiba) 
            + (mac * qty_mac) 
            + (hp * qty_hp) 
            + (dell * qty_dell) 
            + (acer * qty_acer);

    let total_qty = qty_toshiba + qty_mac + qty_hp + qty_dell + qty_acer;
    let average = sum / total_qty;

    println!("The sum of the sales record is: N{}", sum);
    println!("The average sales record per item is: N{}", average);
}