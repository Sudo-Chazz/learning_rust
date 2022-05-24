use std::io;

fn main() {
    // get user input
    let (temp, unit) = get_input();

    // parse/convert to desired types
    let temp: f64 = temp.trim().parse().unwrap();
    let unit = unit.to_uppercase()
                    .chars()
                    .next()
                    .expect("falied to convert to a char");

    // perform conversion
    let converted = match unit {
        'C' => format!("{:.1}°F", c_to_f(&temp)),
        'F' => format!("{:.1}°C", f_to_c(&temp)),
        _ => panic!("Error converting provided temperature."),
    };

    println!("{}", converted);
}

//get unit and temp to convert
fn get_input() -> (String, String) {
    
    let mut unit = String::new();
    println!("Enter the unit of temperature used.");
    io::stdin()
        .read_line(&mut unit)
        .expect("failed to parse provided unit");

    let mut temp = String::new();
    println!("Enter the temperature:");
    io::stdin()
        .read_line(&mut temp)
        .expect("faield to parse provided temperature.");
    
    (temp, unit)
}

fn f_to_c(temperature: &f64) -> f64 {
    (temperature - 32.0) / 1.8
}

fn c_to_f(temperature: &f64) -> f64 {
    (temperature * 1.8) + 32.0
}