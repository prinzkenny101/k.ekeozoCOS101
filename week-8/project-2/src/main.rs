use std::io;

// Function that reads input from user
fn read_input () -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

//
fn main() {

    let mut developer_details: Vec<(String,i32)> = Vec::new();

    loop {

    // Asks the developer for his name
    println!("Enter your name : ");
    let name = read_input();

    // Asks the developer for the number of years of his experience
    println!("Enter your years of experience :");
    let years:i32 = read_input().parse().expect("Enter a valid number");

    // Stores user's input and pushes to the vector
    let details = (name, years);
    developer_details.push(details);

    // Loop for multiple entries
    
    println!("\nDo you want to enter another developer? (Y/N)");
    let continue_input = read_input();

    if continue_input.to_lowercase() == "n" {
        break;
    }

   } 

   // Find the developer with the highest years of experience
   let mut top_name = developer_details [0].0.clone();
   let mut top_years = developer_details [0].1;

   for (name, years) in developer_details {
        if years > top_years {
        top_name = name;
        top_years = years; 
    }
} 

println!("The top developer is {} with {} years of experience.", top_name, top_years);
   
}
