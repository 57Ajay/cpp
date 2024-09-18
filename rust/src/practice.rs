use ::std::fs;
use ::std::io;
use ::std::thread::{self};

fn prac_mult_thread() -> u64 {
    let chunk = 10000000 / 4;
    let mut arr = vec![];
    for i in 0..4 {
        let start: u64 = i * chunk + 1;
        let end = (i + 1) * chunk;
        let handle: thread::JoinHandle<u64> = thread::spawn(move || {
            let val = (start..=end).sum();
            val
        });
        arr.push(handle);
    }
    let mut total_sum = 0;
    for x in arr {
        total_sum += x.join().unwrap();
    }
    total_sum
}
fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn main() {
    println!(
        "{}",
        "******************************This is practice session*******************************"
    );
    println!("{:?}", prac_mult_thread());
    let file_path = "example.txt";
    match read_file(file_path) {
        Ok(content) => println!("File read successfully:\n{}", content),
        Err(error) => println!("Failed to read the file: {}", error),
    }
    println!(
        "{}",
        "***************************This is end of practice session***************************"
    );
}
