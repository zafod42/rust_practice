use std::io;
use std::io::BufRead;
use std::process;

fn main() {
    // Это точно соответсвует?
    // 1. Не использовать динамическую память для хранения последовательности
    // И это пожалуй единственный вопрос
    // А вот тут и используется динамическая память.
    // Как раз при создании String::new() мы получаем vec<u8>
    // Который как раз и получается динамической памятью
    let mut count_inc_seq = 0;
    let mut count_cnt_max = 1;
    let mut count_loc_min = 0;
    let mut zero_found: bool = false;
    for line in io::stdin().lock().lines() {
        let input = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Ошибка ввода: {}", e);
                process::exit(1);
            }
        };
        // Тут уже итератор
        let mut input = input.split_whitespace();

        // Тут используется итератор для доступа к последовательности
        let token = match input.next() {
            Some(s) => s,
            None => {
                eprintln!("Invalid sequence.");
                process::exit(1);
            }
        };
        let mut previous: i32 = match token.parse() {
            Ok(0) => {
                process::exit(2);
            }
            Ok(num) => num,
            Err(e) => {
                eprintln!("Invalid sequence. {e}");
                process::exit(1);
            }
        };

        let mut maximum = previous;
        let token = match input.next() {
            Some(s) => s,
            None => {
                eprintln!("Invalid sequence.");
                process::exit(1);
            }
        };
        let mut middle: i32 = match token.parse() {
            Ok(0) => {
                println!("[CNT-MAX] {count_cnt_max}");
                process::exit(2);
            }
            Ok(num) => num,
            Err(e) => {
                eprintln!("Invalid sequence. {e}");
                process::exit(1);
            }
        };    

        if middle > previous {
            count_inc_seq += 1;
        }

        if middle == maximum {
            count_cnt_max += 1;
        }
        else if maximum < middle {
            maximum = middle;
            count_cnt_max = 1;
        }
        for token in input {
            let number: i32 = match token.parse(){
                Ok(num) => num,
                Err(e) => {
                    eprintln!("Invalid seqience. {e}");
                    process::exit(1);
                }
            };
            if number == 0 {
                zero_found = true;
                break;
            }
            if middle < previous && number > middle {
                count_loc_min += 1;
            }
            if number > middle {
                count_inc_seq += 1;
            }
            if number == maximum {
                count_cnt_max += 1;
            }
            else if maximum < number {
                maximum = number;
                count_cnt_max = 1;
            }
            
            previous = middle;
            middle = number;
        }
    }
    if !zero_found {
        eprintln!("Invalid seqience.");
        process::exit(1)
    }
    
    println!("[INC-SEQ] {count_inc_seq}");
    println!("[CNT-MAX] {count_cnt_max}");
    println!("[LOC-MIN] {count_loc_min}");
}
