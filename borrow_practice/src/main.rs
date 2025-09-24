//Problem 1
fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut result = "".to_string();
    result.push_str(s1);
    result.push_str(s2);
    result
}
//Problem 2
fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut clone = s.clone();
    clone.push_str("World!");
    clone
}
//Problem 3
//#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    let mut i = low;
    while i <= high {
        *total += i; // dereference total to update its value
        i += 1;
    }
}

fn main() {
    println!("Problem 1: ");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
    println!("");

    println!("Problem 2: ");
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
    println!("");

    println!("Problem 3: ");
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Sum from 0 to 100 is: {}", total); // Should print 5050

}
