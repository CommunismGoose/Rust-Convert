//importing
use std::io;
//definng constants
const CEN_IN_MET: f64 = 100.0;
const CEN_IN_KM: f64 = CEN_IN_MET * 1000.0;
const CEN_IN_INCH: f64 = 2.54;
const INCH_IN_FEET: f64 = 12.0;
const INCH_IN_MILES: f64 = INCH_IN_FEET*5280.0;
//main function
fn main() {
    //get what they want to convert from, to, and amount wanted to convert!
    println!("Please chose from the following units of mesaurements to convert fromn\n");
    println!("Distance Measurements:");
    println!("Centimeters\nMeters\nKilometers\nInches\nFeet\nMiles\n");
    println!("Temperature Measurements:");
    println!("Fahrenheit\nCelsius\n");
    println!("Please type what catagory you would like to choose from: Distance or Temperature\n");
    let catagory: String = input();
    println!("\nPlease choose what you are converting from!\n");
    let convfrom: String = input();
    println!("\nPlease choose what you are converting to from the same catagory!\n");
    let convto: String = input();
    println!("\nPlease type the amount you would like to convert\n");
    let mut convamount: f64 = input().parse().expect("Please type in a valid number!");
    //if it is a distance mesaurement 
    if catagory.eq_ignore_ascii_case("distance"){
        //calls distance function to make this cleaner
        convamount = distance(convamount, &convfrom, &convto);
    } if catagory.eq_ignore_ascii_case("temperature"){
        convamount = tempswitch(&convto, &convfrom, convamount);
    }
    println!("Your conversion of {convfrom} to {convto} results in {convamount} {convto}!");
}
//function that converts metric units to centimeters 
//and imperial units to inches aswell as returns if its metric or imperial
fn convertfrom(convfrom:&str,convamount:f64)-> (f64, &str) {
    if convfrom.eq_ignore_ascii_case("meters"){
        return (convamount*CEN_IN_MET, "Metric");
    } else if convfrom.eq_ignore_ascii_case("kilometers"){
        return (convamount*CEN_IN_KM, "Metric");
    } else if convfrom.eq_ignore_ascii_case("feet"){
        return (convamount*INCH_IN_FEET, "Metric");
    } else if convfrom.eq_ignore_ascii_case("milesz"){
        return (convamount*INCH_IN_MILES, "Imperial");
    } else {
        return (convamount, "unknown")
    }
}
//converts from inches and centimeters to the required unit based on
//Determines if it is inches or centimeters from the metric or imperial
//return from last string.
fn convertto(convto: &str, convamount: f64, unit:&str)->f64{
    if unit == "Metric"{
        if convto.eq_ignore_ascii_case("Meters"){
            return convamount/CEN_IN_MET
        } else if convto.eq_ignore_ascii_case("Kilometers"){
            return convamount/CEN_IN_KM;
        } else if convto.eq_ignore_ascii_case("Feet"){
            return (convamount/CEN_IN_INCH)/INCH_IN_FEET;
        } else if convto.eq_ignore_ascii_case("Miles"){
            return (convamount/CEN_IN_INCH)/INCH_IN_MILES;
        } else {
            panic!("An unforseen error has occured!");
        }
    }else if unit == "Imperial"{
        if convto.eq_ignore_ascii_case("Meters"){
            return (convamount*CEN_IN_INCH)/CEN_IN_MET;
        } else if convto.eq_ignore_ascii_case("Kilometers"){
            return (convamount*CEN_IN_INCH)/CEN_IN_KM;
        } else if convto.eq_ignore_ascii_case("Feet"){
            return convamount/INCH_IN_FEET;
        } else if convto.eq_ignore_ascii_case("Miles"){
            return convamount/INCH_IN_MILES;
        } else{ 
            panic!("An unforseen error has occured!");
        }
    } else{
        panic!("An unforseen error has occured!")
    }
}
//converts tempatures
fn tempswitch(convto: &str, convfrom:&str, convamount:f64)->f64{
    if convfrom.eq_ignore_ascii_case("celsius") && convto.eq_ignore_ascii_case("fahrenheit"){
        return (convamount*9.0)/5.0+32.0;
    } else if convfrom.eq_ignore_ascii_case("fahrenheit") && convto.eq_ignore_ascii_case("celsius"){
        return ((convamount-32.0)*5.0)/9.0;
    } else {
        return convamount;
    }
}
//Function that makes a stirng from user input andn removes whitespace.
fn distance(mut convamount:f64, convfrom:&str, convto: &str)->f64{
    //starts converting by calling the functions needed
    let convtuple = convertfrom(&convfrom, convamount);
    convamount = convtuple.0;
    let mut unit = convtuple.1;
    if convfrom.eq_ignore_ascii_case("centimeters"){
        unit = "Metric";
    } else if convfrom.eq_ignore_ascii_case("inches"){
        unit = "Imperial";
    }
    if convto.eq_ignore_ascii_case("centimeters") && convto.eq_ignore_ascii_case("inches"){
        
    } else {
        convamount = convertto(&convto, convamount, unit);
    }
    return convamount
}   
fn input() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Something failed!");
    let x = x.trim().to_string();
    return x;
}
