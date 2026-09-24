//! Exercise 7 (§5.7): a student register over a `HashMap`.
//!
//! `add_student` refuses a duplicate ID with one lookup: `entry` finds the
//! slot, and the slot says whether it is taken. Printing goes through fixed
//! IDs, never through the map's own order, which is not defined.
//!
//!   rustc --edition 2024 pg5_07_student_manager.rs -o /tmp/pg507 && /tmp/pg507

use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Student {
    id: u32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<u32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    /// `student.id` is copied out before `student` moves into the slot.
    fn add_student(&mut self, student: Student) -> Result<(), String> {
        match self.students.entry(student.id) {
            Entry::Vacant(slot) => {
                slot.insert(student);
                Ok(())
            }
            Entry::Occupied(_) => Err(format!("Student with ID {} already exists", student.id)),
        }
    }

    fn get_student(&self, id: u32) -> Option<&Student> {
        self.students.get(&id)
    }

    fn count(&self) -> usize {
        self.students.len()
    }
}

fn main() {
    let mut manager = StudentManager::new();

    let roster = [(1, "Alice", "A"), (2, "Bob", "B"), (1, "Alicia", "C")];
    for (id, name, grade) in roster {
        let student = Student {
            id,
            name: name.to_string(),
            grade: grade.to_string(),
        };
        match manager.add_student(student) {
            Ok(()) => println!("added {name} as #{id}"),
            Err(e) => println!("refused: {e}"),
        }
    }
    println!("{} students on file", manager.count());

    for id in [1, 2, 3] {
        match manager.get_student(id) {
            Some(student) => println!("#{id}: {} (grade {})", student.name, student.grade),
            None => println!("#{id}: no such student"),
        }
    }
}
