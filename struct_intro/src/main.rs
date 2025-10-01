struct Student{
    name: String,
    major: String,
}

impl Student{
    fn new(name: String, major: String) -> Self{
        Self{
            name,
            major,
        }
    }
    // Method to set the major
    fn set_major(&mut self, new_major: String) {
        self.major = new_major;
    }

    // Method to get the major
    fn get_major(&self) -> &String {
        return &self.major
    }
}

fn main(){
    let mut student1 = Student::new(String::from("Juan"), String::from("Computer Science"));
    println!("{}: {}", student1.name, student1.get_major());

    // Set a new major
    student1.set_major(String::from("Software Engineering"));

    // Get and print the updated major
    println!("{}: {}", student1.name, student1.get_major());
}

