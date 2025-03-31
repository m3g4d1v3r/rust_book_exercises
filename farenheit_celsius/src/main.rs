const MULT: f64 = 100.0;

fn main() {
    println!("Hello, world!");

    assert!(farenheit_2_celsius(32.0) == 0.0);
    assert!(farenheit_2_celsius(212.0) == 100.0);
    assert!(farenheit_2_celsius(98.6) == 37.0);
    assert!(farenheit_2_celsius(68.0) == 20.0);
    assert!(farenheit_2_celsius(0.0) == -17.78);
    assert!(farenheit_2_celsius(-40.0) == -40.0);
    assert!(farenheit_2_celsius(451.0) == 232.78);

    assert!(celsius_2_farenheit(0.0) == 32.0);
    assert!(celsius_2_farenheit(100.0) == 212.0);
    assert!(celsius_2_farenheit(37.0) == 98.6);
    assert!(celsius_2_farenheit(20.0) == 68.0);
    assert!(celsius_2_farenheit(-17.78) == 0.0);
    assert!(celsius_2_farenheit(-40.0) == -40.0);
    assert!(celsius_2_farenheit(232.78) == 451.0);

    assert!(farenheit_2_celsius(1000.0) == 537.78);
    assert!(celsius_2_farenheit(-273.15) == -459.67);
}

fn farenheit_2_celsius(value: f64) -> f64 {
    let result = ((value - 32.0) / 180.0) * 100.0;
    (result * MULT).round() / MULT
}

fn celsius_2_farenheit(value: f64) -> f64 {
    let result = (value) / 100.0 * 180.0 + 32.0;
    (result * MULT).round() / MULT
}
