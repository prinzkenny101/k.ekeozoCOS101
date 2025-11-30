use std::fs::File;
use std::io::{Write, stdin};

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("This is not a valid input");
    input.trim().to_string()
}

fn main() {
    let columns = vec![
        ("Name","Matric No", "Department", "Level"),
    ];

    let mut students = vec![
        ("Oluchi Mordi", "ACC10211111", "Accounting", "300"),
        ("Adams Aliyu", "ECO10110101", "Economics", "100"),
        ("Shanle Bolade", "CSC10328828", "Computer", "200"),
        ("Adekunle Gold", "EEE11020202", "Electrical", "200"),
        ("Blanca Edemoh", "MEE10202001", "Mechanical", "100"),
     ];

    // This part manually adds students to the list
    loop {
        println!("Do you want to add a new student? (yes/no): ");
        let choice = read_input("Enter your choice: ").to_lowercase();

        if choice != "yes" {
            break;
        }

        let name = read_input("Enter student name: ");
        let matric = read_input("Enter matric number: ");
        let dept = read_input("Enter department: ");
        let level = read_input("Enter level: ");

        students.push((
            Box::leak(name.into_boxed_str()),
            Box::leak(matric.into_boxed_str()),
            Box::leak(dept.into_boxed_str()),
            Box::leak(level.into_boxed_str())
            ));

    }

     println!("PAU SIMS - Student Records");

     for column in &columns {
        println!("{} | {} | {} | {}", column.0, column.1, column.2, column.3);
     }

     println!(".....................................");

     for student in &students {
        println!("{} | {} | {} | {}", student.0, student.1, student.2, student.3);
     }
    
    let mut file = File::create("students.txt").unwrap();
    
    for column in columns{
        let line = format!("{:<20} | {:<20} | {:<20} | {:<20}\n", column.0, column.1, column.2, column.3);
        file.write_all(line.as_bytes()).unwrap();
    }

    let separator = format!("{:-<75}\n", ""); // Creates a line of 80 dashes
    file.write_all(separator.as_bytes()).unwrap();

    for student in students {
        let line = format!("{:<20} | {:<20} | {:<20} | {:<20}\n", student.0, student.1, student.2, student.3);
        file.write_all(line.as_bytes()). unwrap();
    }
    
    println!("Student data saved to students.txt successfully");
}
