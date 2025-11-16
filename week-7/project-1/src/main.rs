use std::io;
use std::f64::consts::PI;

// FUNCTIONS -----------
fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_cube(side: f64) -> f64 {
    6.0 * side.powf(2.0)
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius.powf(2.0) * height
}

fn read_input() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input.trim().to_string() 
    }

// Main Program --------
fn main() {
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Enter your choice:");

    let choice = read_input().trim().parse::<u32>().unwrap();

    match choice {
        1 => {
            println!("Enter height:");
            let height = read_input().trim().parse::<f64>().unwrap();

            println!("Enter base1:");
            let base1 = read_input().trim().parse::<f64>().unwrap();

            println!("Enter base2:");
            let base2 = read_input().trim().parse::<f64>().unwrap();

            let area = area_trapezium(height, base1, base2);
            println!("Area of Trapezium = {}", area);
        }

        2 => {
            println!("Enter diagonal1:");
            let d1 = read_input().trim().parse::<f64>().unwrap();

            println!("Enter diagonal2:");
            let d2 = read_input().trim().parse::<f64>().unwrap();

            let area = area_rhombus(d1, d2);
            println!("Area of Rhombus = {}", area);

        }     

       3 => {
            println!("Enter base:");
            let base = read_input().trim().parse::<f64>().unwrap();

            println!("Enter altitude:");
            let altitude = read_input().trim().parse::<f64>().unwrap();

            let area = area_parallelogram(base, altitude);
            println!("Area of Parallelogram = {}", area);
        }      

       4 => {
            println!("Enter side length:");
            let side = read_input().trim().parse::<f64>().unwrap();

            let area = area_cube(side);
            println!("Area of Cube = {}", area); 
        }
        
       5 => {
            println!("Enter radius:");
            let radius = read_input().trim().parse::<f64>().unwrap();

            println!("Enter height:");
            let height = read_input().trim().parse::<f64>().unwrap();

            let volume = volume_cylinder(radius, height);
            println!("Volume of Cylinder = {}", volume); 
        } 

        _=> println!("Invalid choice!"),             

    }
 }

    

