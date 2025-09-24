
// Assignment 1: Temperature Converter
fn fahrenheit_to_celsius(f: f64) -> f64 {
    //temp converter 
    let result_cel = (f - 32.0) * (5.0 / 9.0); 
    return result_cel;
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let result_far = (c * ( 9.0/5.0)) + 32.0; 
    return result_far;
}

fn assignment1() {
    const FREEZING_POINT_F: f64 = 32.0;
    let mut fahrenheit = FREEZING_POINT_F;

    println!("{fahrenheit}°F = {:.2}°C", fahrenheit_to_celsius(fahrenheit));
    println!("0.00°C = {:.2}°F", celsius_to_fahrenheit(0.0));

    for _ in 0..5 {
        fahrenheit += 1.0;
        println!("{fahrenheit}°F = {:.2}°C", fahrenheit_to_celsius(fahrenheit));
    }
    println!("");
}

// Assignment 2: Number Analyzer
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn assignment2() {
    let arr = [10, 34, 53, 56, 3, 5, 23, 78, 66, 30];

    for num in 0..10 {
        if is_even(arr[num]) {
            println!("{} is even", arr[num]);
        } else {
            println!("{} is odd", arr[num]);
        }

        if arr[num] % 3 == 0 && arr[num] % 5 == 0 {
            println!("FizzBuzz");
        } else if arr[num] % 3 == 0 {
            println!("Fizz");
        } else if arr[num] % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", arr[num]);
        }
        println!("----------------");
    }

    let mut counter = arr.len();
    let mut sum = 0;
    while counter != 0 {
        sum += arr[counter - 1];
        counter -= 1;
    }
    println!("Sum of all Numbers in the Array: {}", sum);
    println!("----------------");

    let mut counter2 = 0;
    let mut largest_num = arr[0];
    while counter2 < arr.len() {
        if arr[counter2] > largest_num {
            largest_num = arr[counter2];
        }
        counter2 += 1;
    }
    println!("Largest Number is: {}", largest_num);
    println!("");
}

// Assignment 3: Guessing Game
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn assignment3() {
    let mut secret_num = 10;
    let mut guess = 1;
    let mut opportunities = 0;

    loop {
        opportunities += 1;
        let result = check_guess(guess, secret_num);

        if result == 0 {
            println!("Guess {}: Correct!", guess);
            break;
        } else if result == 1 {
            println!("Guess {}: Too high!", guess);
        } else {
            println!("Guess {}: Too low", guess);
        }

        guess += 1;
    }
    println!("It took {} guesses to find the secret number.", opportunities);
    println!("");
}

fn main() {
    println!("Assignment 1:");
    assignment1();

    println!("Assignment 2:");
    assignment2();
    
    println!("Assignment 3:");
    assignment3();
}
