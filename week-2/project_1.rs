fn main() {
	
	let principal:f64 = 520_000_000.00; //stores the mortgage loan
	let rate:f64 = 10.00 / 100.00; //stores the rate
	let number_of_years:f64 = 5.00; //stores the number of years

	let amount:f64 = principal * (1.00 +(rate)).powf(number_of_years);

	let compound_interest = amount - principal; //calculate the final value
	println!("the compound interest is {:.2}", compound_interest);

}