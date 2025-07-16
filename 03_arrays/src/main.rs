///
/// # Массивы
///
/// ## Вариант 14 -- MAX-SUM-SDG
/// 
/// Найти максимальную сумму элементов среди диагоналей, параллельных главной диагонали 
///
use verbose_macros::verbose;
use std::io::Write;
use std::fs;
use std::fs::File;
use std::cmp;
use std::fmt;
use std::env;
use std::process;
use std::error::Error;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {err}");
        process::exit(2);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input)?;

    let mut input = contents.split_whitespace();

    let Some(rows) = input.next() else {
        return Err(Box::new(LabError::CannotInterpret));
    };

    let Ok(rows) = rows.parse() else {
        return Err(Box::new(LabError::CannotInterpret));
    };

    let Some(cols) = input.next() else {
        return Err(Box::new(LabError::CannotInterpret));
    };

    let Ok(cols) = cols.parse() else {
        return Err(Box::new(LabError::CannotInterpret));
    };

    let input: Vec<&str> = input.collect();
    let output = match config.mode {
        1 => stack_array(rows, cols, input)?,
        2 => dynamic_array(rows, cols, input)?,
        _ => panic!("No.. It cant be.. Nooooo"),
    };

    let mut out_file = File::create(config.output)?;
    write!(out_file, "{output}\n")?;
    Ok(())
}

fn stack_array(
    rows: usize, 
    cols: usize, 
    input: Vec<&str>,
) -> Result<i32, LabError> {
    let mut fixed_array = [0; 10000];
    let mut size = 0;
    
    for token in &input {
        let Ok(number) = token.parse() else {
            return Err(LabError::CannotInterpret);
        };
        fixed_array[size] = number;
        size += 1;
    }

    if size != rows * cols {
        return Err(LabError::CannotInterpret); 
    }

    let mut max_sum = i32::MIN;
    for d in -((cols as i32)-1)..(rows as i32) {
        let i_low: i32 = cmp::max(0, -d);
        let i_high: i32 = cmp::min((cols as i32) - 1, (rows as i32) - 1 - d);
        
        if i_low > i_high {
            continue;
        }

        let mut current_sum = 0;
        for i in i_low..i_high+1 {
            let j = i + d;
            current_sum += fixed_array[(i * rows as i32 + j) as usize];
        }
        if current_sum > max_sum {
            max_sum = current_sum
        }
    }

    Ok(max_sum)
}

fn dynamic_array(
    rows: usize, 
    cols: usize, 
    input: Vec<&str>,
) -> Result<i32, LabError> {
    let mut fixed_array: Vec<i32> = Vec::new();
    
    for token in &input {
        let Ok(number) = token.parse() else {
            return Err(LabError::CannotInterpret);
        };
        fixed_array.push(number);

    }

    if fixed_array.len() != rows * cols {
        return Err(LabError::CannotInterpret); 
    }

    let mut max_sum = i32::MIN;
    for d in -((cols as i32)-1)..(rows as i32) {
        let i_low: i32 = cmp::max(0, -d);
        let i_high: i32 = cmp::min((cols as i32) - 1, (rows as i32) - 1 - d);
        
        if i_low > i_high {
            continue;
        }

        let mut current_sum = 0;
        for i in i_low..i_high+1 {
            let j = i + d;
            current_sum += fixed_array[(i * rows as i32 + j) as usize];
        }
        if current_sum > max_sum {
            max_sum = current_sum
        }
    }

    Ok(max_sum)
}

struct Config {
    mode: i32,
    input: String,
    output: String,
}

#[derive(Debug)]
enum LabError{
    NotEnough,
    TooMany,
    FirstIsNan,
    FirstOutOfRange,
    CannotInterpret,
}

impl Error for LabError {}
impl fmt::Display for LabError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LabError::NotEnough => write!(f, "Not enough arguments"),
            LabError::TooMany => write!(f, "Too many arguments"),
            LabError::FirstIsNan => write!(f, "First parameter is not a number"),
            LabError::FirstOutOfRange => write!(f, "First parameter is out of range"),
            LabError::CannotInterpret => write!(f, "Cannot interpret file contents as array"),
        }
    }
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, LabError> {
        args.next(); // skip program name
        
        let Some(mode) = args.next() else {
            return Err(LabError::NotEnough);
        };
        let Ok(mode) = mode.parse() else {
            return Err(LabError::FirstIsNan);
        };
        if !(mode == 1 || mode == 2) {
            return Err(LabError::FirstOutOfRange);
        }

        let Some(input) = args.next() else {
            return Err(LabError::NotEnough);
        };

        let Some(output) = args.next() else {
            return Err(LabError::NotEnough);
        };

        let extra = args.count();
        if extra > 0 {
            return Err(LabError::TooMany);
        }

        Ok(Config {
            mode,
            input,
            output,
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_array_not_a_number() {
        let rows = 2;
        let cols = 2;
        let input = ["a", "2", "3", "f"].iter();
    
        assert_eq!(Err(LabError::CannotInterpret), stack_array(rows, cols, input));
    }

}

