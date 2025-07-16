// Структуры данных
#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grade: u8,
}

#[derive(Debug)]
struct Teacher {
    name: String,
    age: u32,
    subject: String,
}

#[derive(Debug)]
struct Manager {
    name: String,
    age: u32,
    department: String,
}

#[derive(Debug)]
struct Applicant {
    name: String,
    age: u32,
    average_score: f32,
}

// Трейт для получения возраста
trait HasAge {
    fn get_age(&self) -> u32;
}

// Реализация трейта для всех структур
impl HasAge for Student {
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl HasAge for Teacher {
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl HasAge for Manager {
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl HasAge for Applicant {
    fn get_age(&self) -> u32 {
        self.age
    }
}

// Обобщенные функции для работы с итераторами
fn average_age(items: &Vec<Box<dyn HasAge>>) -> f64 {
    let (total, count) = items.iter()
        .filter_map(|s| {
            Some(s.get_age())
        })
        .fold((0f64, 0usize), |(sum, count), age| (sum + age as f64, count + 1));

    if count > 0 {
        total as f64 / count as f64
    } else {
        0.0
    }
}

use std::io;
use std::io::BufRead;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut people: Vec<Box<dyn HasAge>> = Vec::new();

    for line in stdin.lock().lines() {
        let input = line?;
        let input = input.trim();

        if input.is_empty() {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() < 4 {
            eprintln!("Ошибка: недостаточно данных");
            continue;
        }

        match parts[0] {
            "Student" => {
                if let (Ok(age), Ok(grade)) = (parts[2].parse(), parts[3].parse()) {
                    people.push(Box::new(Student {
                        name: parts[1].to_string(),
                        age,
                        grade,
                    }));
                }
            }
            "Teacher" => {
                if let Ok(age) = parts[2].parse() {
                    people.push(Box::new(Teacher {
                        name: parts[1].to_string(),
                        age,
                        subject: parts[3..].join(" "),
                    }));
                }
            }
            "Manager" => {
                if let Ok(age) = parts[2].parse() {
                    people.push(Box::new(Manager {
                        name: parts[1].to_string(),
                        age,
                        department: parts[3..].join(" "),
                    }));
                }
            }
            "Applicant" => {
                if let (Ok(age), Ok(score)) = (parts[2].parse(), parts[3].parse()) {
                    people.push(Box::new(Applicant {
                        name: parts[1].to_string(),
                        age,
                        average_score: score,
                    }));
                }
            }
            _ => eprintln!("Неизвестный тип: {}", parts[0]),
        }
    }

    let avg_all = average_age(&people);
    println!("\nСредний возраст всех: {:.2}", avg_all);

    Ok(())
}
