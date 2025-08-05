use std::collections::HashMap;

#[derive(Default)]
pub struct School {
    grade_to_students: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grade_to_students: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.grade_to_students.entry(grade).or_default();

        if !students.contains(&student.to_string()) {
            students.push(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grade_to_students.clone().into_keys().collect::<Vec<u32>>();
        grades.sort();

        grades
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students = self.grade_to_students.get(&grade).unwrap_or(&vec![]).clone();
        students.sort();

        students
    }
}
