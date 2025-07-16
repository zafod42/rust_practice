use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения строки");

    // Парсим строку в три числа
    let numbers: Vec<f64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Пожалуйста, введите корректные числа"))
        .collect();

    if numbers.len() != 3 {
        println!("Вы должны ввести ровно три числа!");
        return;
    }

    
    calculate_and_print_results(numbers[0], numbers[1], numbers[2] as i32);
}

fn calculate_and_print_results(from: f64, to: f64, max_eval: i32) {
    let step = 0.01;
    let error = 0.00001;
    let lb = from;
    let ub = to;
    let n = ((ub - lb) / step) as i32;

    let precision = 7;
    println!("x\tf\treal");
    for i in 0..n+1 {
        let x_val = lb + step * (i as f64);
        let atanh = x_val.atanh();
        print!("{:.*}\t", 2, x_val);

        match f(x_val, max_eval, error) {
            Ok(val) => print!("{:.*}\t", precision, val),
            Err(e) => print!("{}\t", e),
        }

        println!("{:.*}", precision, atanh);
    }
}

///
/// # Функция вычисления ряда Тейлора
///
/// Использует следующее выражение для вычисления ряда Тейлора
/// 
/// $$
/// atanh x = x + \frac{x^3}{3} + \frac{x^5}{5} + \frac{x^7}{7} + ... + \frac{x^{2n+1}}{2n+1} + ...
/// $$ 
/// Результирующий ряд соответсвует ряду Маклорена для arctanh -- гиперболического
/// арктангенса.
///
///
fn f(x: f64, k: i32, error: f64) -> Result<f64, &'static str> {
    let mut current = 0.0_f64;
    let mut result = 0.0_f64;
    
    let mut previous = x;
    for i in 1..k {
        result += previous;
        current = x.powf(2.0 * (i as f64) + 1.0) / (2.0 * (i as f64) + 1.0);
        if (current - previous).abs() < error {
            break;
        }
        previous = current;
    }

    if current - previous > error {
        return Err("<MATH ERROR>");
    }
    Ok(result)
}
