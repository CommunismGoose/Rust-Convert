//importing
use std::process;
//definng constants
const CEN_IN_MET: f64 = 100.0;
const CEN_IN_KM: f64 = CEN_IN_MET * 1000.0;
const CEN_IN_INCH: f64 = 2.54;
const INCH_IN_FEET: f64 = 12.0;
const INCH_IN_MILES: f64 = INCH_IN_FEET * 5280.0;
//struct
//main function
fn main() {
    //for commandline tool
    let catagory = std::env::args()
        .nth(1)
        .expect("Please type help for help!");
    if catagory.eq_ignore_ascii_case("help"){
        help();
    }
    let convfrom = std::env::args()
        .nth(2)
        .expect("No starting unit, please do convert help for help!");
    let convto = std::env::args()
        .nth(3)
        .expect("No end unit, please do convert help for help!");
    let mut convamount = std::env::args()
        .nth(4)
        .expect("No amount, please do convert help for help!")
        .parse()
        .expect("Please type in a valid number!");

    //help command
    
    //if it is a distance mesaurement
    if catagory.eq_ignore_ascii_case("distance") {
        //calls distance function to make this cleaner
        convamount = distance(convamount, &convfrom, &convto);
    } else if catagory.eq_ignore_ascii_case("temperature") {
        convamount = tempswitch(&convto, &convfrom, convamount);
    } else{
        help();
    }
    println!("Your conversion of {convfrom} to {convto} results in {convamount} {convto}!");
}
//function that converts metric units to centimeters
//and imperial units to inches aswell as returns if its metric or imperial
fn convertfrom(convfrom: &str, convamount: f64) -> (f64, &str) {
    if convfrom.eq_ignore_ascii_case("m") {
        return (convamount * CEN_IN_MET, "Metric");
    } else if convfrom.eq_ignore_ascii_case("km") {
        return (convamount * CEN_IN_KM, "Metric");
    } else if convfrom.eq_ignore_ascii_case("f") {
        return (convamount * INCH_IN_FEET, "Metric");
    } else if convfrom.eq_ignore_ascii_case("mi") {
        return (convamount * INCH_IN_MILES, "Imperial");
    } else if convfrom.eq_ignore_ascii_case("cm") {
        return (convamount, "Metric");
    } else if convfrom.eq_ignore_ascii_case("inch") {
        return (convamount, "Imperial");
    } else if convfrom.eq_ignore_ascii_case("y"){
        return (convamount*INCH_IN_FEET*3.0, "Imperial")
    }else {
        println!("invalid, exiting");
        panic!();
    }
}
//converts from inches and centimeters to the required unit based on
//Determines if it is inches or centimeters from the metric or imperial
//return from last string.
fn convertto(convto: &str, convamount: f64, unit: &str) -> f64 {
    if unit == "Metric" {
        if convto.eq_ignore_ascii_case("m") {
            return convamount / CEN_IN_MET;
        } else if convto.eq_ignore_ascii_case("km") {
            return convamount / CEN_IN_KM;
        } else if convto.eq_ignore_ascii_case("f") {
            return (convamount / CEN_IN_INCH) / INCH_IN_FEET;
        } else if convto.eq_ignore_ascii_case("mi") {
            return (convamount / CEN_IN_INCH) / INCH_IN_MILES;
        } else if convto.eq_ignore_ascii_case("y"){
            return (convamount/CEN_IN_INCH)/INCH_IN_FEET/3.0;
        }else if convto.eq_ignore_ascii_case("cm"){
            return convamount
        }else if convto.eq_ignore_ascii_case("inch"){
            return convamount/CEN_IN_INCH
        }else {
            panic!("An unforseen error has occured!");
        }
    } else if unit == "Imperial" {
        if convto.eq_ignore_ascii_case("m") {
            return (convamount * CEN_IN_INCH) / CEN_IN_MET;
        } else if convto.eq_ignore_ascii_case("km") {
            return (convamount * CEN_IN_INCH) / CEN_IN_KM;
        } else if convto.eq_ignore_ascii_case("f") {
            return convamount / INCH_IN_FEET;
        } else if convto.eq_ignore_ascii_case("mi") {
            return convamount / INCH_IN_MILES;
        }else if convto.eq_ignore_ascii_case("y"){
            return convamount / INCH_IN_FEET/3.0
        }else if convto.eq_ignore_ascii_case("inch"){
            return convamount
        }else if convto.eq_ignore_ascii_case("cm"){
            return convamount*CEN_IN_INCH
        }else {
            panic!("An unforseen error has occured!");
        }
    } else {
        panic!("An unforseen error has occured!")
    }
}
//converts tempatures
fn tempswitch(convto: &str, convfrom: &str, convamount: f64) -> f64 {
    if convfrom.eq_ignore_ascii_case("c") && convto.eq_ignore_ascii_case("f") {
        return (convamount * 9.0) / 5.0 + 32.0;
    } else if convfrom.eq_ignore_ascii_case("f") && convto.eq_ignore_ascii_case("c")
    {
        return ((convamount - 32.0) * 5.0) / 9.0;
    } else {
        return convamount;
    }
}
//Function that makes a stirng from user input andn removes whitespace.
fn distance(mut convamount: f64, convfrom: &str, convto: &str) -> f64 {
    //starts converting by calling the functions needed
    let convtuple = convertfrom(&convfrom, convamount);
    convamount = convtuple.0;
    let unit = convtuple.1;
    convamount = convertto(&convto, convamount, unit);
    return convamount;
}
fn help(){
    println!("Please chose from the following units of mesaurements to convert fromn\n");
    println!("Please use the shorthand version (In parentheses)");
    println!("Distance Measurements:");
    println!("Centimeters(CM)\nMeters(M)\nKilometers(KM)\nInches(INCH)\nFeet(F)\nYard (Y)\nMiles(MI)\n");
    println!("Temperature(TEMP) Measurements:");
    println!("Fahrenheit(F)\nCelsius(C)\n");
    println!("To use the command please type \nConvert (catagory you want) (Starting unit) (Ending unit) (amount)");
    println!("For example this is a valid line:\nCargo run Distance MI CM 27");
    process::exit(32);
}
