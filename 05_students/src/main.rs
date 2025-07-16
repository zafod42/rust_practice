/*
### Лаба 1

Реализуйте структуру 
```rust
struct Students {
    name: &str,
    exams: Vec<&str>,
    grades: Vec<f64>,
}
```

Необходимо прочитать структуры в массив ``` Vec<Students> ```
И при помощи итераторов и замыканий создать новый массив ``` Vec<Students> ```
с приминёнными к нему фильтрами по некоторому условию

Формат входных данных: (что-то попроще)
Описание каждого студента содержит имя студента после ключевого слова Student и 
список экзаменов и оценок за них. Оценка отделена от названия экзамена пробелом.

Student Alex
Math 2.0
History 3.0
Science 3.0

Student Bob
Math 4.0
History 2.0
Science 4.0
Physics 5.0

Если при описании структуры название экзамена повторено несколько раз, то запись
с наивысшей оценкой является актуальной

Student Alex
Math 2.0 <-- Игнорируем
Math 3.0 <-- Записываем

Описание студентов заканчивается EOF, при вводе с стандартного потока Ctrl-D для Linux

Варианты:
1. Количество студентов, которые сдали Историю. (grade > 2.0)
2. Количество студентов, чьё имя начинается на А.
3. Средний балл среди всех студентов, сдававших Физику
4. Количество студентов, которые вошли в .75 процентиль сдававших математику
5. Средний балл всех студентов с именем Олег
*/

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

