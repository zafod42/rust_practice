pub struct Student {
    pub name: String,
    pub exams: Vec<String>,
    pub grades: Vec<f64>,
}

impl Student {
    pub fn new() -> Student {
        Student {
            name: String::new(),
            exams: Vec::new(),
            grades: Vec::new(),
        }
    }
}

// Вариант 1
pub fn count_pass_history(students: &Vec<Student>) -> usize {
    let result = students.iter()
        .filter(|s| {
            s.exams
                .iter()
                .zip(&s.grades)
                .any(|(exam, &grade)| *exam == "History" && grade >= 2.5)
        })
        .count();
    result
}

// Вариант 2
pub fn count_name_starts_with_prefix(students: &Vec<Student>, prefix: &str) -> usize {
    let result = students.iter()
        .filter(|s| s.name.starts_with(prefix))
        .count();
    result
}

// Вариант 3
pub fn average_in_physics(students: &Vec<Student>) -> f64 {
    let (total, count) = students.iter()
        .filter_map(|s| {
            s.exams.iter()
                .position(|exam| *exam == "Physics")
                .and_then(|idx| s.grades.get(idx))
        })
        .fold((0f64, 0usize), |(sum, count), &grade| (sum + grade, count + 1));
    if count > 0 {
        total / count as f64
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pass_history_count_correct() {
        let students = vec![
            Student {
                name: "Alex",
                exams: vec!["Math", "Physics"],
                grades: vec![2.0, 3.5],
            },
            Student {
                name: "Bob",
                exams: vec!["History", "Science", "Physics"],
                grades: vec![4.0, 2.0, 2.3],
        }];
        
        assert_eq!(1usize, count_pass_history(&students));
    }

    #[test]
    fn test_count_name_starts_with_prefix_correct() {
        let students = vec![
            Student {
                name: "Alex",
                exams: vec!["Math", "Physics"],
                grades: vec![2.0, 3.5],
            },
            Student {
                name: "Bob",
                exams: vec!["History", "Science", "Physics"],
                grades: vec![4.0, 2.0, 2.3],
        }];
        
        assert_eq!(1usize, count_name_starts_with_prefix(&students, "A"));
    }

    #[test]
    fn test_average_in_physics_correct() {
        let students = vec![
            Student {
                name: "Alex",
                exams: vec!["Math", "Physics"],
                grades: vec![2.0, 3.5],
            },
            Student {
                name: "Bob",
                exams: vec!["History", "Science", "Physics"],
                grades: vec![4.0, 2.0, 2.3],
        }];
        
        assert_eq!(2.9f64, average_in_physics(&students));
    }
}

