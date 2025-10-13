fn main() {
	
	let toshiba_laptop:f64 = 450_000.00 * 2.00;
	let mac_laptop:f64 = 1_500_000.00 * 1.00;
	let hp_laptop:f64 = 750_000.00 * 3.00;
	let dell_laptop:f64 = 2_850_000.00 * 3.00;
	let acer_laptop:f64 = 250_000.00 * 1.00;

	let sum:f64 = toshiba_laptop + mac_laptop + hp_laptop + dell_laptop + acer_laptop; //finds the sum of all the laptops
	let average:f64 = sum / 10.00; //finds the average of all the laptops
	println!("the sum of all the laptops is {} and the average is {}", sum, average);	
}