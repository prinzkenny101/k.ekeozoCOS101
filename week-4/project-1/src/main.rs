use std::io;

fn main () 
{

  //creating variables to store values
  let mut a = String::new();
  let mut b = String::new();
  let mut c = String::new();

  println!("input a value for a");
  io::stdin().read_line(&mut a).expect("error");
  let a_input:f64 = a.trim().parse().expect("Please type in a valid number");

  println!("input a value for b");
  io::stdin().read_line(&mut b).expect("error");
  let b_input:f64 = b.trim().parse().expect("Please type in a valid number");

  println!("input a value for c");
  io::stdin().read_line(&mut c).expect("error");
  let c_input:f64 = c.trim().parse().expect("Please type in a valid number");

  let discriminant:f64 = b_input.powf(2.0) - 4.0*a_input*c_input;

  if discriminant > 0.0
  {
    let root_1:f64 = - b_input + discriminant.sqrt() / 2.0*a_input;
    let root_2:f64 = - b_input - discriminant.sqrt() / 2.0*a_input;

    println!("The first root is {}",root_1);
    print!("The second root is {}",root_2);
  }

  else if discriminant == 0.0

  {
    let root_1:f64 = - b_input + discriminant.sqrt() / 2.0*a_input;

    println!("The one root is {}",root_1);

  }


  else if discriminant < 0.0

  {
    println!("no real roots");
  }

  else 
  {
    println!("The provided value is invalid");
  }
}