use std::io;

fn main() 
{
  
  // creating variables to store inputs
  let mut input1 = String::new();
  let mut input2 = String::new();

  println!("Please type true or false");
  io::stdin().read_line(&mut input1).expect("Not a valid string"); 
  let experience_level:String = input1.trim().parse().expect("Please enter a valid string");

  println!("Enter your age");
  io::stdin().read_line(&mut input2).expect("Not a valid age");
  let age:u32 = input2.trim().parse().expect("Please enter a valid age");

  if experience_level == "true" && age >= 40
  {
    println!("Your incentive is N1,560,000");
  }

  else if experience_level == "true" && age >=30 && age < 40
  {
    println!("Your incentive is N1,480,000");
  }

  else if experience_level == "true" && age < 28
  {
    println!("Your incentive is N1,300,000");
  }

  else if experience_level == "false"
  {
    println!("Your incentive is N100,000"); 
  }

  else
  {
  println!("Your provided information doesn't meet any criteria"); 
  }
}