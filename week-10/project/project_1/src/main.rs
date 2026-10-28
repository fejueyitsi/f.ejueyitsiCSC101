struct Laptop {
    brand: String,
    price: u64,
}

impl Laptop {
    fn cost_of_purchase(&self) -> u64 {
        3 * self.price
    }
}

fn main() {
    let brand1 = Laptop {
        brand: String::from("HP"),
        price: 650_000,
    };
    let brand2 = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
    };
    let brand3 = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
    };
    let brand4 = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
    };

    // Use the brand field by printing individual costs or referencing them
    println!("3 {} laptops cost: ₦{}", brand1.brand, brand1.cost_of_purchase());
    println!("3 {} laptops cost: ₦{}", brand2.brand, brand2.cost_of_purchase());
    println!("3 {} laptops cost: ₦{}", brand3.brand, brand3.cost_of_purchase());
    println!("3 {} laptops cost: ₦{}", brand4.brand, brand4.cost_of_purchase());

    let grand_total = brand1.cost_of_purchase() 
                    + brand2.cost_of_purchase() 
                    + brand3.cost_of_purchase() 
                    + brand4.cost_of_purchase();

    println!("\nTotal cost across the four brands: ₦{}", grand_total);
}