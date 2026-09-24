//! Exercise 4, solved the other way: the fields stay private, a constructor
//! computes the grade from the marks, and getters hand the values out. The
//! module is snake_case, the binding is not `mut`, and `marks` is read.
//!
//!   rustc --edition 2024 pg6_04_private_fields.rs -o /tmp/pg604 && /tmp/pg604

mod university {
    pub struct Student {
        name: String,
        marks: u8,
        grade: char,
    }

    impl Student {
        pub fn new(name: String, marks: u8) -> Self {
            let grade = match marks {
                90..=100 => 'A',
                75..=89 => 'B',
                50..=74 => 'C',
                _ => 'F',
            };
            Self { name, marks, grade }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn marks(&self) -> u8 {
            self.marks
        }

        pub fn grade(&self) -> char {
            self.grade
        }
    }
}

use university::Student;

fn main() {
    let student_1 = Student::new(String::from("Alice"), 75);
    println!("{} got {} grade", student_1.name(), student_1.grade()); // Alice got B grade
    println!("{} marks, and no way to write a grade that does not match them", student_1.marks()); // 75 marks, ...
}
