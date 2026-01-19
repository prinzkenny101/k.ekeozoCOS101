use std::io::Read;
use std::io;

fn admin() {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}

fn pmanager() {
    let mut file = std::fs::File::open("project.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}

fn employee() {
    let mut file = std::fs::File::open("employees.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}

fn customer() {
    let mut file = std::fs::File::open("customer_table.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}

fn vendor() {
    let mut file = std::fs::File::open("dataplan_table.sql").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    println!("{}", content);
}

fn main() {
    println!("Welcome to Globacom Database Access System");
    println!("Select your role:");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();
    
    match choice {
        "1" => admin(),
        "2" => pmanager(),
        "3" => employee(),
        "4" => customer(),
        "5" => vendor(),
        _ => println!("Invalid selection!"),
    }
}