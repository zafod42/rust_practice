
use students::Student;

use std::error::Error;
use std::fmt;
use std::process;
use std::io::BufRead;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}


fn run() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin().lock();

    let input: Vec<String> = stdin
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

    let mut students: Vec<Student> = Vec::new();
    let mut student = Student::new();
    for line in input {
        let Some((key, value)) = line.split_once(' ') else {
            continue
        };
        match key {
            "Student" => {
                if student.name != "" {
                    students.push(student);
                }
                student = Student::new();
                student.name = value.to_string();
            },
            "Math" | "Physics" | "History" | "Science" => {
                student.exams.push(key.to_string());
                let value = value.parse::<f64>()?;
                student.grades.push(value);
            },
            _ => {
                eprintln!("No `{}' exam present", key);
            }
        }
    }
    if student.name != "" {
        students.push(student);
    }

    println!("Students passed History: {}", students::count_pass_history(&students));
    println!("Students names starts with `A': {}", students::count_name_starts_with_prefix(&students, "A"));
    println!("Students average in Physics: {}", students::average_in_physics(&students));
    Ok(())
}

