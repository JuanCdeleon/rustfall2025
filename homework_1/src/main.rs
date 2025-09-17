// Assignment 1

fn fahrenheit_to_celsius(f: f64) -> f64{
    //temp converter
    let resultCel = (f - 32.0) * (5.0 / 9.0);
    return resultCel;

}
fn celsius_to_fahrenheit(c: f64) -> f64{
    let resultFar = (c * ( 9.0/5.0)) + 32.0;
    return resultFar;
}

fn main(){
    println!("{}", fahrenheit_to_celsius(50.0));
    println!("{}", celsius_to_fahrenheit(50.0));
}
