// Define the Student struct
struct Student {
    major: String,
}

// First-order function to assign a major to a student
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher-order function to update majors of all students
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String), new_major: String) {
    for student in &mut collection {
        behavior(student, new_major.clone());
    }

    // Display updated majors
    for (i, student) in collection.iter().enumerate() {
        println!("Student {}: Major = {}", i + 1, student.major);
    }
}

fn main() {
    // Create a vector of students with initial empty majors
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    // Update all students' majors to "Computer Science"
    update_majors(students, assign_major, "Computer Science".to_string());
}

