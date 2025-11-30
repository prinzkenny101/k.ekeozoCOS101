use std::fs::File;
use std::io::Write;

fn main() {

    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieve",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defrnse",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    println!("EFCC MERGED DATASET");
    println!("S/N | COMMISSIONER | MINISTRY | GEO-POLITICAL ZONE");
    println!("-----------------------------------------------------");

    for i in 0..commissioners.len() {
        println!(
            "{} | {} | {} | {}",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }

    let mut file = File::create("efcc_merged_dataset.csv").unwrap();

    file.write_all(b"S/N, Commissioner, Ministry, Geopolitical Zone\n").unwrap();

    for i in 0..commissioners.len() {
        let line = format!(
            "{}, {}, {}, {}\n",
            i + 1,
            commissioners[i],
            ministries[i],
            zones[i]
        );
        file.write_all(line.as_bytes()).unwrap();
    }

    println!("\nMerged dataset saved to efcc_merged_dataset.csv successfully!");
}
