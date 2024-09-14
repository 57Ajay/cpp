use crate::datetime;
use std::fs;

// function
fn fib(num: u32) -> u32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }
    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=num {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    return curr;
}
//string
fn str_len(s: &str) -> usize {
    return s.chars().count();
}
//struct
struct User {
    active: bool,
    name: String,
    password: String,
    age: i8,
}

struct Rect {
    length: u8,
    bredth: u8,
}
//struct implementatin;
impl Rect {
    fn area(&self) -> u8 {
        return self.length * self.bredth;
    }
    fn perimeter(&self, i: u8) -> u8 {
        if i != 2 {
            println!("Please enter only 2 an a parameter to find the perimeter of the rectangle.");
            return 0;
        }
        i * (self.length + self.bredth)
    }
    fn polygon() -> bool {
        return true;
    } // this is a static function
}

//Enums

enum Shape {
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}

fn area(shape: Shape) -> f32 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(a) => a * a,
        Shape::Rectangle(a, b) => a * b,
    }
}

//Option/Result Enums;

// Option Enum lets us return a number or null

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn read_file(file: &str) {
    let result = fs::read_to_string(file);
    match result {
        Ok(file_content) => {
            println!("file read successfully:\n {:?}", file_content)
        } // ok(data)=> ok(data);
        Err(error) => {
            println!("Failed to read the file: {:?}", error)
        } // we can also write it like Err(error)=> Err(String::from("File not read"));
    }
}

pub fn main() {
    println!("{}", fib(11));
    println!("the length of the string is: {}", str_len("Ajay Upadhyay"));
    let user = User {
        active: true,
        name: String::from("Ajay"),
        password: String::from("qwer4321"),
        age: 22,
    };
    println!(
        "age: {}, name: {}, password: {}, active: {}",
        user.age, user.name, user.password, user.active
    );

    let rectangle: Rect = Rect {
        length: 12,
        bredth: 21,
    };

    println!(
        "area of the rectangle is: {}, perimeter of rectangle is: {}, is rectangle a polygon?: {}",
        rectangle.area(),
        rectangle.perimeter(2),
        Rect::polygon()
    );
    let rect = Shape::Rectangle(12.0, 89.0);
    let sq = Shape::Square(7.0);
    let cir = Shape::Circle(49.0);
    println!(
        "Area of rectangle: {}, Area of square: {}, Area of circle: {}",
        area(rect),
        area(sq),
        area(cir)
    );
    let index = find_first_a(String::from("Ajay Upadhyay"));
    match index {
        Some(value) => println!("index is: {}", value),
        None => println!("a not found"),
    }
    println!("{:?}", read_file("cargo.toml"));
    // Get UTC formatted date and time
    let formatted_time = datetime::get_formatted_time();
    println!("Formatted UTC Date and Time: {}", formatted_time);

    // Get Local date and time
    let local_time = datetime::get_local_time();
    println!("Local Date and Time: {}", local_time);

    // Get both UTC and Local in a single call
    let combined_time = datetime::get_dt();
    println!("Both UTC and Local Times:\n{}", combined_time);
}
