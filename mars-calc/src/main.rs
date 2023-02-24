use std::io;

fn main() {
    println!("Enter your weight (kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}g", mars_weight);
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32 {
    (weight_on_earth / 9.81) * 3.711
}
