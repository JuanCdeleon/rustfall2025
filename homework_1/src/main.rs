// Assignment 1

fn fahrenheit_to_celsius(f: f64) -> f64{
    //temp converter
    let result_cel = (f - 32.0) * (5.0 / 9.0);
    return result_cel;

}
fn celsius_to_fahrenheit(c: f64) -> f64{
    let result_far = (c * ( 9.0/5.0)) + 32.0;
    return result_far;
}

fn main(){
    println!("{}", fahrenheit_to_celsius(50.0));
    println!("{}", celsius_to_fahrenheit(50.0));
}
