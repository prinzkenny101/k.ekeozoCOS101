use std::fs::File;
use std::io::Write;

fn main() {
    // 1. Create or open a file called drinks.txt
    let mut file = File::create("drinks.txt")
        .expect("Could not create file");

    // 2. The table content we want to save 
    let content = "

    Lager       | Stout        | Non-Alcoholic
    -------------------------------------------------
    33 Export   | Legend       | Maltina
    Desperados  | Turbo King   | Amstel Maltina
    Goldberg    | Williams     | Malta Gold
    Gulder      |              | Fayrouz
    Heineken    |              |
    Star        |
    ";

        // 3. Write the content into the file
        file.write_all(content.as_bytes())
            .expect("Could not write to file");

    println!("drinks.txt has been created successfully!");
}
