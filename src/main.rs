use std::io;

fn main() {

    println!("Which temperature scale are you converting from? Choose F for Fahrenheit, C for Celsius, R for Rankine or K for Kelvin");

    let mut scale_from = String::new();

    io::stdin()
    .read_line(&mut scale_from)
    .expect("Failed to read the line.");

    if scale_from.trim().to_lowercase() == "c" {

        println!("What is your temperature in degrees Celsius?");

        let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        let mut scale_to = String::new();

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");

        if scale_to.trim().to_lowercase() == "f" {

            let fahren: f64 = temp * 1.8 + 32.0;
    
            println!("Your converted temperature is {:.1} °F", fahren);
        }
        else if scale_to.trim().to_lowercase() == "r" {
            let rank: f64 = (temp  + 273.15) * 1.8;

            println!("Your converted temperature is {:.1} °R", rank);
        }
        else{
            let kelvin: f64 = temp + 273.15;

            println!("Your converted temperature is {:.1} K", kelvin);
        }
    }

    else if scale_from.trim().to_lowercase() == "f" {

        println!("What is your temperature in degrees Fahrenheit?");

        let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        let mut scale_to = String::new();

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");
        
        if scale_to.trim().to_lowercase() == "c" {

            let celsius = (temp - 32.0) / 1.8;

            println!("Your converted temperature is {:.1} °C", celsius);
        }
        else if scale_to.trim().to_lowercase() == "r" {
            let rank = temp + 459.67;

            println!("Your converted temperature is {:.1} °R", rank);
        } else {
            let kelvin = (temp + 459.67) / 1.8;

            println!("Your converted temperature is {:.1} K", kelvin);
        }
    }
    else if scale_from.trim().to_lowercase() == "r" {


        println!("What is your temperature in degrees Rankine?");

        let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        let mut scale_to = String::new();

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");
        
        if scale_to.trim().to_lowercase() == "c" {

            let celsius: f64 = (temp * 1.8) - 459.67;

            println!("Your converted temperature is {:.1} °C", celsius);
        }
        else if scale_to.trim().to_lowercase() == "f"{

            let fahren: f64 = temp - 459.67;

            println!("Your converted temperature is {:.1} °F", fahren);
        }
        else {

            let kelvin: f64 = temp /1.8;

            println!("Your converted temperature is {:.1} K", kelvin);
        }
    }
    else if scale_from.trim().to_lowercase() == "k" {

         println!("What is your temperature in Kelvin?");

        let mut temp = String::new();

        io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

        let temp: f64 = temp.trim().parse().expect("Required a number");

        println!("Which temperature scale are you trying to convert to?");

        let mut scale_to = String::new();

        io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");
        
        if scale_to.trim().to_lowercase() == "c" {

            let celsius: f64 = temp - 273.15;

            println!("Your converted temperature is {:.1} °C", celsius);
        }
        else if scale_to.trim().trim().to_lowercase() == "f" {

            let fahren: f64 = (temp * 1.8) - 459.67;

            println!("Your converted temerature is {:.1} °F", fahren);
        }
        else {

            let rank: f64 = temp * 1.8;

            println!("Your converted temperature is {:.1} °R", rank);
        }

    }
    else {
        println!("That is not a valid scale of temperature measurement.")
    }
}
