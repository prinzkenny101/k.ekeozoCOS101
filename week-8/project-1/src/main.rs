use std::io;

// Function that reads input from user 
fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== APS Level Checker ===");

    // STEP 1: Ask for profession
    println!("Enter Profession (admin / academic / lawyer / teacher) : ");
    let profession = read_input().to_lowercase();

    // STEP 2: Ask for years of experience
    println!("Enter years of experience");
    let years: i32 = read_input().parse().expect("Enter a valid number");

    // STEP 3: Map experience to APS level
    let aps_level = if years <= 2 {
        "APS 1-2"
    } else if years <= 5 {
        "APS 3-5"
    } else if years <= 8 {
        "APS 5-8"
    } else if years <= 10 {
        "EL1 8-10"
    } else if years <= 13 {
        "EL2 10-13"
    } else {
        "SES"
    };

    // STEP 4: Job titles arranged in vectors based on profession
    let admin_roles = vec! [
        "Intern",               // APS 1-2
        "Administrator",        // APS 3-5
        "Senior Administrator",  // APS 5-8
        "Office Manager",       // EL1 8-10
        "Director",             // EL2 10-13
        "CEO",                  // SES
    ];

    let academic_roles = vec![
        "-",                    // APS 1-2
        "Research Assistant",   // APS 3-5
        "PhD Candidate",        // APS 5-8
        "Post-Doc Researcher",  // EL1 8-10
        "Senior Lecturer",     // EL2 10-13
        "Dean",                 // SES
    ];

    let lawyer_roles = vec![
        "Paralegal",            // APS 1-2
        "Junior Associate",     // APS 3-5
        "Associate",            // APS 5-8
        "Senior Associate 1-2", // EL1 8-10
        "Senior Associate 3-4", // EL2 10-13
        "Partner",              // SES
    ];

    let teacher_roles = vec![
        "Placement",            // APS 1-2
        "Classroom Teacher",    // APS 3-5
        "Snr Teacher",          // APS 5-8
        "Leading Teacher",      // EL1 8-10
        "Deputy Principal",     // EL2 10-13
        "Principal",            // SES
    ];

    // STEP 5: Pick the correct role depending on APS level index
    let index = match aps_level {
        "APS 1-2" => 0,
        "APS 3-5" => 1,
        "APS 5-8" => 2,
        "EL1 8-10" => 3,
        "EL2 10-13" => 4,
        _ => 5, // SES  
    };

    let job_title = match profession.as_str() {
        "admin" => admin_roles[index],
        "academic" => academic_roles[index],
        "lawyer" => lawyer_roles[index],
        "teacher" => teacher_roles[index],
        _ => "Invalid profession",
    };

    // STEP 6: Final Output
    println!("\n=== Result ===");
    println!("Profession: {}", profession);
    println!("Years of Experience: {}", years);
    println!("APS Level: {}", aps_level);
    println!("Position: {}", job_title);
}
