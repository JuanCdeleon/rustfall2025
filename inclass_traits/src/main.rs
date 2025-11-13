// define 2 struct undegrad and grad student

// define trait show info

// grad student should have a thesis compnent
// gpa and major will be shared
struct UndergradStud{
    gpa: f64,
    major: String,
}

struct GradStud{
    gpa: f64,
    major: String,
    thesis: String,
}

pub trait ShowInfo{
    fn get_info(&self) -> String;
}

impl ShowInfo for UndergradStud{
    fn get_info(&self) -> String{
        format!("gpa: {}\nmajor: {}", self.gpa, self.major)
    }
}

impl ShowInfo for GradStud{
    fn get_info(&self) -> String {
        format!("gpa: {}\nmajor: {}\nthesis: {}", self.gpa, self.major, self.thesis)
    }
}

// create another struct called Enrollment
// inside enrollment store undegrad and grads together
// implement show_info  for all enrolled student

// Professor said to use T in enrollment but using that
// makes me unable to put both undergrad and grad
// students in the same enrollment object.
// So I used Box<dyn ShowInfo> instead
struct Enrollment{
    enrolled: Vec<Box<dyn ShowInfo>>,
}

impl ShowInfo for Enrollment {
    fn get_info(&self) -> String {
        self.enrolled
            .iter()
            .map(|student| student.get_info())
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}

// OLD VERSION IS AS FOLLOWS
// DOES NOT ALLOW MIXING OF TYPE OF STUDENT IN VECTOR

//struct Enrollment<T: ShowInfo>{
//    enrolled: Vec<T>,
//}
//
//impl<T: ShowInfo> ShowInfo for Enrollment<T> {
//    fn get_info(&self) -> String {
//        self.enrolled
//            .iter()
//            .map(|student| student.get_info())
//            .collect::<Vec<String>>()
//            .join("\n\n")
//    }
//}

fn get_information<T: ShowInfo>(item: &T){
    println!("Student Info:\n{}", item.get_info())
}

fn main() {
    let stud1 = UndergradStud {gpa: 4.0, major: "Comp Sci".to_string()};
    let stud2 = UndergradStud {gpa: 3.5, major: "Mathematics".to_string()};
    let stud3 = GradStud {gpa: 3.7, major: "Physics".to_string(), thesis:"Quantum Mechanics".to_string()};
    get_information(&stud1);
    println!("");
    get_information(&stud2);
    println!("");
    get_information(&stud3);


    println!("\n\n=== Enrollment Info ===");
    let class = Enrollment {enrolled: vec![Box::new(stud1), Box::new(stud2), Box::new(stud3)]};
    get_information(&class);
}
