const FAHRENHEIT_FREEZING_POINT: f32 = 32.0;

fn fahrenheit_to_celsius(f: f32) -> f32 {
    return (f - FAHRENHEIT_FREEZING_POINT) * (5.0 / 9.0);
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    return c * (9.0 / 5.0) + FAHRENHEIT_FREEZING_POINT;
}

fn main() {
    let mut fahrenheit_temperature: f32 = 32.0;

    println!("Starting Fahrenheit Temperature is: {fahrenheit_temperature}");

    let celsius_temperature = fahrenheit_to_celsius(fahrenheit_temperature);

    println!("In celius it is: {celsius_temperature}");

    let loop_value: f32 = fahrenheit_temperature + 5.0;

    while fahrenheit_temperature < loop_value {
        fahrenheit_temperature += 1.0;
        let celsius_temperature = fahrenheit_to_celsius(fahrenheit_temperature);
        println!("{fahrenheit_temperature}°F is {celsius_temperature}°C");
    }
}
