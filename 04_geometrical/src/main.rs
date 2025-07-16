use geometrical::shape::Shape;
use geometrical::base_types::{Point, Rect};
use geometrical::rectangle::Rectangle;
use geometrical::triangle::Triangle;
use geometrical::ellipse::Ellipse;

use geometrical::commands::Scale;

use std::io;
use std::io::BufRead;
use std::error::Error;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}

/*
 *  После выполнения масштабирования в соответствии с указанными параметрами, программа
 *  должна вывести в стандартный вывод на отдельных строках:
 *  – суммарную площадь и координаты ограничивающих прямоугольников обрабатываемых
 *  фигур в порядке их описания до масштабирования.
 *  – суммарную площадь и координаты ограничивающих прямоугольников обрабатываемых
 *  фигур в порядке их описания после масштабирования
 *  Элементы должны быть разделены ровно одним пробелом, содержать один и только один знак
 *  после запятой (округление проводить в соотвествии с правилами математики) и описаны в
 *  следующем порядке.
 *
 */

fn run() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();
    let lines: Vec<String> = stdin
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Cannot read stdio");
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    let mut errors: Vec<Box<dyn Error>> = Vec::new();
    let mut scale_command_present: bool = false;
    let mut scale: Scale = Scale::new();
    
    for line in lines {
        if line == "" { continue }
        let Some((name, figure_str)) = line.split_once(' ') else { continue };
        match name {
            "RECTANGLE" => {
                let rectangle: Rectangle = match figure_str.parse() {
                    Ok(rect) => rect,
                    Err(err) => {
                        errors.push(Box::new(err));
                        continue;
                    },
                };
                shapes.push(Box::new(rectangle));
            },
            "TRIANGLE" => { 
                let triangle: Triangle = match figure_str.parse() {
                    Ok(rect) => rect,
                    Err(err) => {
                        errors.push(Box::new(err));
                        continue;
                    },
                };
                shapes.push(Box::new(triangle));
            },
            "ELLIPSE" => {
                let ellipse: Ellipse = match figure_str.parse() {
                    Ok(rect) => rect,
                    Err(err) => {
                        errors.push(Box::new(err));
                        continue;
                    },
                };
                shapes.push(Box::new(ellipse));
            },
            "SCALE"=> {
                scale = match figure_str.parse() {
                    Ok(some) => some,
                    Err(err) => {
                        eprintln!("{err}");
                        break;
                    }
                };
                scale_command_present = true;
                break;
            },
            _ => continue,
        }
    }

    if !scale_command_present {
        eprintln!("SCALE command expected!");
        process::exit(1);
    }

    let mut area = 0.0;
    for shape in &shapes {
        area += shape.get_area();
    }
    print!("{area}");
    for shape in &shapes {
        print!(" {}", shape.get_frame_rect());
    }
    println!("");

    let mut area = 0.0;
    for shape in &mut shapes {
        let center = shape.get_center();
        shape.scale(scale.factor);
        shape.move_at(&scale.origin);

        let mut offset: Point = center - scale.origin;
        offset *= scale.factor;
        shape.move_by(offset.x, offset.y);

        area += shape.get_area();
    }
    print!("{area}");
    for shape in &shapes {
        print!(" {}", shape.get_frame_rect());
    }
    println!("");

    for error in errors {
        eprintln!("{}", error);
    }

    Ok(())
}



