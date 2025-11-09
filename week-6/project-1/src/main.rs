use std::io;

fn main() {

    loop {

    // prices of food items
    let p_price:f64 = 3200.0; // Pounded Yam and Edikaikong Soup
    let f_price:f64 = 3000.0; // Fried Rice and Chicken
    let a_price:f64 = 2500.0; // Amala and Ewedu Soup
    let e_price:f64 = 2000.0; // Eba and Egusi Soup
    let w_price:f64 = 2500.0; // White Rice and Stew
    let mut continue_code = String::new();

    // Displaying the food items and their prices
    println!("MENU");
    println!("P = Pounded Yam / Edikaikong Soup - N3,200");
    println!("F = Fried Rice and Chicken - N3,000");
    println!("A = Amala and Ewedu Soup - N2,500");
    println!("E = Eba and Egusi Soup - N2,000");
    println!("W = White Rice and Stew - N2,500");

    // Get type of food
    println!("Enter the type of food (P/F/A/E/W): ");
    let mut food_type = String::new();
    io::stdin().read_line(&mut food_type).expect("Failed to read input");
    let food_type = food_type.trim().to_uppercase();

    // Get quantity
    println!("Enter quantity: ");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:f64 = quantity.trim().parse().expect("Please enter a number");

    // Determine price based on selection
    let price = match food_type.as_str() {
        "P" => p_price,
        "F" => f_price,
        "A" => a_price,
        "E" => e_price,
        "W" => w_price,
        _ => {
            println!("Sorry, we don't sell that here");
            return;
        }

    };

    let mut total:f64 = price * quantity;

    // Apply 5% discount if total > 10000
    if total > 10000.0 {
        let discount:f64 = total * 0.05;
        total -= discount;
        println!("5% discount has been applied to your order");
    }

    println!("Total amount to pay: N{:.2}", total);

    println!("\nWould you like to continue");
    io::stdin().read_line(&mut continue_code).expect("Not a valid string");
    let continue_input = continue_code.trim().to_uppercase();

    if continue_input == "NO" {
        break;
    }

   } 
}
