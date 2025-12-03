//declaring a structure for laptops, prices and their quantities
struct Devices {
    laptop:String,
    price:u32,
}

impl Devices {
    fn total_cost (&self)->u32 {
        self.price * 3
    }
}

fn main() {
    let device1 = Devices{
        laptop:String::from("HP Laptop"),
        price:650_000,
    };
    let device2 = Devices{
        laptop:String::from("IBM Laptop"),
        price:755_000,
    };
    let device3 = Devices{
        laptop:String::from("Toshiba Laptop"),
        price:550_000,
    };
    let device4 = Devices{
        laptop:String::from("Dell Laptop"),
        price:850_000,
    };

    // Calculate totals
    let total_hp = device1.total_cost();
    let total_ibm = device2.total_cost();
    let total_toshiba = device3.total_cost();
    let total_dell = device4.total_cost();

    let grand_total = total_hp + total_ibm + total_toshiba + total_dell; 

    // Print results
    println!("Cost of buying 3 {} is: {}", device1.laptop, total_hp);
    println!("Cost of buying 3 {} is: {}", device2.laptop, total_ibm);
    println!("Cost of buying 3 {} is: {}", device3.laptop, total_toshiba);
    println!("Cost of buying 3 {} is: {}", device4.laptop, total_dell);
    println!("\nGrand Total Cost for all devices: {}", grand_total);
}
