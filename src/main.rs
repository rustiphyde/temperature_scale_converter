use std::io;

fn main() {

    println!("Which temperature scale are you converting from? Choose F for Fahrenheit, C for Celsius, R for Rankine or K for Kelvin");

    let mut scale_from = String::new();
    let mut temp = String::new();
    let mut scale_to = String::new();

    io::stdin()
    .read_line(&mut scale_from)
    .expect("Failed to read the line.");

    if scale_from.trim().to_lowercase() == "c" {

        println!("What is your temperature in degrees Celsius?");

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");

        celsius(temp, &scale_to);
    }

    else if scale_from.trim().to_lowercase() == "f" {

        println!("What is your temperature in degrees Fahrenheit?");

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");
        
       fahrenheit(temp, &scale_to);
    }
    else if scale_from.trim().to_lowercase() == "r" {

        println!("What is your temperature in degrees Rankine?");

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");
        
        rankine(temp, &scale_to);
    }
    else if scale_from.trim().to_lowercase() == "k" {

        println!("What is your temperature in Kelvin?");

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");
        
        kelvin(temp, &scale_to);
    }
    else {
        println!("That is not a valid scale of temperature measurement.")
    }
}

fn celsius(t: f64, s: &str) {

    let s: &str = &s.trim().to_lowercase();

    if s == "f"{
        let fahren: f64 = t * 1.8 + 32.0;
    
        println!("Your converted temperature is {:.2} °F", fahren);
    }
    else if s == "r"{
        let rank: f64 = (t  + 273.15) * 1.8;

        println!("Your converted temperature is {:.2} °R", rank);
    }
    else if s == "k"{
        let kelv: f64 = t + 273.15;

        println!("Your converted temperature is {:.2} K", kelv);
    }
    else {
        println!("That is not a valid scale of temperature measurement");
    }
}

fn fahrenheit(t: f64, s: &str) {

    let s: &str = &s.trim().to_lowercase();

    if s == "c"{
        let cels: f64 = (t - 32.0) / 1.8;
    
        println!("Your converted temperature is {:.2} °C", cels);
    }
    else if s == "r"{
        let rank: f64 = t + 459.67;

        println!("Your converted temperature is {:.2} °R", rank);
    }
    else if s == "k"{
        let kelv: f64 = (t + 459.67) / 1.8;

        println!("Your converted temperature is {:.2} K", kelv);
    }
    else {
        println!("That is not a valid scale of temperature measurement");
    }
}

fn rankine(t: f64, s: &str) {

    let s: &str = &s.trim().to_lowercase();

    if s == "c"{
        let cels: f64 = (t - 491.67) / 1.8;

        println!("Your converted temperature is {:.2} °C", cels);
    }
    else if s == "f"{
        let fahren: f64 = t - 459.67;

        println!("Your converted temperature is {:.2} °F", fahren);
    }
    else if s == "k"{
        let kelv: f64 = t / 1.8;

        println!("your converted temperature is {:.2} K", kelv);
    }
    else {
        println!("That is not a valid scale of temperature measurement.");
    }
}

fn kelvin(t: f64, s: &str){

    let s: &str = &s.trim().to_lowercase();

    if s == "c"{
        let cels: f64 = t - 273.15;

        println!("Your converted temperature is {:.2} °C", cels);
    }
    else if s == "f"{
        let fahren: f64 = (t * 1.8) - 459.67;

        println!("Your converted temperature is {:.2} °F", fahren);
    }
    else if s == "r"{
        let rank: f64 = t * 1.8;

        println!("Your converted temperature is {:.2} °R", rank)
    }
    else {
        println!("That is not a valid scale of temperature measurement.");
    }

}