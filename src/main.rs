use std::io;

fn main() {
    println!("Which temperature scale are you converting from? Choose F for Fahrenheit, C for Celsius, R for Rankine or K for Kelvin");

    let mut scale_from = String::new();
    let mut temp = String::new();
    let mut scale_to = String::new();

    io::stdin()
        .read_line(&mut scale_from)
        .expect("Failed to read the line.");
    choose_scale(&scale_from);

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read the line.");

    let temp: f64 = temp.trim().parse().expect("Required a number");

    println!("Which temperature scale are you trying to convert to?");

    io::stdin()
        .read_line(&mut scale_to)
        .expect("Failed to read the line.");

    use_scale(&scale_from, &scale_to, temp);
}

enum Temp {
    F(f64),
    C(f64),
    R(f64),
    K(f64),
}

fn convert_celsius(t: &Temp) -> f64 {
    match t {
        &Temp::F(deg) => (deg - 32.0) / 1.8,
        &Temp::R(deg) => (deg + 273.15) * 1.8,
        &Temp::K(deg) => deg + 273.15,
        &Temp::C(deg) => deg,
    }
}

fn convert_fahrenheit(t: &Temp) -> f64 {
    match t {
        &Temp::F(deg) => deg,
        &Temp::C(deg) => (deg - 32.0) / 1.8,
        &Temp::R(deg) => deg + 459.67,
        &Temp::K(deg) => (deg + 459.67) / 1.8,
    }
}

fn convert_rankine(t: &Temp) -> f64 {
    match t {
        &Temp::F(deg) => deg - 459.67,
        &Temp::C(deg) => (deg - 491.67) / 1.8,
        &Temp::R(deg) => deg,
        &Temp::K(deg) => deg / 1.8,
    }
}

fn convert_kelvin(t: &Temp) -> f64 {
    match t {
        &Temp::C(deg) => deg - 273.15,
        &Temp::R(deg) => deg * 1.8,
        &Temp::F(deg) => (deg * 1.8) - 459.67,
        &Temp::K(deg) => deg,
    }
}

fn print_celsius(t: &Temp) {
    match t {
        &Temp::F(deg) => println!("{} °C = {:.2} °F", deg, convert_celsius(t)),
        &Temp::R(deg) => println!("{} °C = {:.2} °R", deg, convert_celsius(t)),
        &Temp::K(deg) => println!("{} °C = {:.2} K", deg, convert_celsius(t)),
        &Temp::C(deg) => println!("{} °C ", deg),
    };
}

fn print_fahrenheit(t: &Temp) {
    match t {
        &Temp::F(deg) => println!("{} °F", deg),
        &Temp::C(deg) => println!("{} °F = {:.2} °C", deg, convert_fahrenheit(t)),
        &Temp::R(deg) => println!("{} °F = {:.2} °R", deg, convert_fahrenheit(t)),
        &Temp::K(deg) => println!("{} °F = {:.2} K", deg, convert_fahrenheit(t)),
    };
}

fn print_rankine(t: &Temp) {
    match t {
        &Temp::R(deg) => println!("{} °R", deg),
        &Temp::C(deg) => println!("{} °R = {:.2} °C", deg, convert_rankine(t)),
        &Temp::F(deg) => println!("{} °R = {:.2} °F", deg, convert_rankine(t)),
        &Temp::K(deg) => println!("{} °R = {:.2} K", deg, convert_rankine(t)),
    };
}

fn print_kelvin(t: &Temp) {
    match t {
        &Temp::K(deg) => println!("{} K", deg),
        &Temp::C(deg) => println!("{} K = {:.2} °C", deg, convert_kelvin(t)),
        &Temp::R(deg) => println!("{} K = {:.2} °R", deg, convert_kelvin(t)),
        &Temp::F(deg) => println!("{} K = {:.2} °F", deg, convert_kelvin(t)),
    };
}

fn celsius_scale(s: &str, t: f64) {
    let s: &str = &s.trim().to_uppercase();

    let t: Temp = match s {
        "F" => Temp::F(t),
        "R" => Temp::R(t),
        "K" => Temp::K(t),
        "C" => Temp::C(t),
        _ => Temp::C(t),
    };

    print_celsius(&t);
}

fn fahrenheit_scale(s: &str, t: f64) {
    let s: &str = &s.trim().to_uppercase();

    let t: Temp = match s {
        "F" => Temp::F(t),
        "R" => Temp::R(t),
        "K" => Temp::K(t),
        "C" => Temp::C(t),
        _ => Temp::F(t),
    };

    print_fahrenheit(&t);
}

fn rankine_scale(s: &str, t: f64) {
    let s: &str = &s.trim().to_uppercase();

    let t: Temp = match s {
        "F" => Temp::F(t),
        "R" => Temp::R(t),
        "K" => Temp::K(t),
        "C" => Temp::C(t),
        _ => Temp::R(t),
    };

    print_rankine(&t);
}

fn kelvin_scale(s: &str, t: f64) {
    let s: &str = &s.trim().to_uppercase();

    let t: Temp = match s {
        "F" => Temp::F(t),
        "R" => Temp::R(t),
        "K" => Temp::K(t),
        "C" => Temp::C(t),
        _ => Temp::K(t),
    };

    print_kelvin(&t);
}

fn choose_scale(s: &str) {
    let s: &str = &s.trim().to_uppercase();

    let scale = match s {
        "F" => println!("What is your temperature in degrees Fahrenheit?"),
        "C" => println!("What is your temperature in degrees Celsius?"),
        "R" => println!("What is your temperature in degrees Rankine?"),
        "K" => println!("What is your temperature in Kelvin?"),
        _ => println!("That is an invalid response"),
    };

    return scale;
}

fn invalid() {
    println!("That is an invalid response.");
}

fn use_scale(sf: &str, st: &str, t: f64) {
    let sf: &str = &sf.trim().to_uppercase();

    let convert = match sf {
        "C" => celsius_scale(st, t),
        "F" => fahrenheit_scale(st, t),
        "R" => rankine_scale(st, t),
        "K" => kelvin_scale(st, t),
        _ => invalid(),
    };

    return convert;
}
