fn main() {

	let principal:f64 = 510_000.00; //stores the price of the TV
	let rate:f64 = 5.00 / 100.00; //stores the rate
	let number_of_years:f64 = 3.00; //stores the number of years

	let amount:f64 = principal * (1.00 - (rate)).powf(number_of_years);

	print!("the value after 3 years is {:.2}", amount);
}