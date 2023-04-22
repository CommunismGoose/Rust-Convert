use std::io;
use std::char::ToLowercase;
const MM_IN_CM: f32 = 10.0;
const MM_IN_METER: f32 = 1000.0;
const MM_IN_INCH: f32 = 24.4;
const MM_IN_FEET: f32 = 304.8;
fn main() {
    println!("Please chose from the following units of mesaurements to convert from");
    println!("Distance Measurements:");
    println!("Milimeters\nCentimeters\nMeters\nInches\nFeet\n");
    println!("Temperature Measurements:");
    println!("Fahrenheit\nCelsius\n");
    let convfrom: String = input();
    println!("Please choose what you are converting to from the same catagory!");
    let convto: String = input();
    println!("Please type the amount you would like to convert");
    let convamount: f32 = input().parse().expect("Please type in a valid number!");
    //Find the starting unit
    //Find ending unit
    //Multiply convert amount times correct ammount
    //Return right amount
    //
    //
    //
    //
    match convfrom {
        "milimeters"=> 

    }
}
fn input()-> String{
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Something failed!");
    let x = x.trim().to_string().ToLowercase();
    return x;
}
