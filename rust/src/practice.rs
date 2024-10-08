use ::std::fs;
use ::std::io;
use ::std::thread::{self};
use std::sync::Arc;

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
    let result_ = fs::read_to_string(file_path);
    result_
}

fn char_counter(sentence: &str, c: char) -> u32 {
    let length = sentence.len();
    let chunk_size = length / 4;

    let mut handles = vec![];

    for i in 0..4 {
        let start = i * chunk_size;
        let end = if i == 3 { length } else { (i + 1) * chunk_size };

        let sentence_chunk = sentence[start..end].to_string();
        let c = c.clone();

        let handle = thread::spawn(move || {
            let mut count = 0;
            for ch in sentence_chunk.chars() {
                if ch == c {
                    count += 1;
                }
            }
            count
        });

        handles.push(handle);
    }

    let mut total_count = 0;
    for handle in handles {
        total_count += handle.join().unwrap();
    }

    total_count
}

fn char_counter_improved(sentence: &str, c: char) -> u32 {
    let length = sentence.len();
    let num_threads = 4;
    let chunk_size = length / num_threads;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            length
        } else {
            (i + 1) * chunk_size
        };

        let sentence_chunk = sentence[start..end].to_string();
        let c = c;

        let handle =
            thread::spawn(move || sentence_chunk.chars().filter(|&ch| ch == c).count() as u32);

        handles.push(handle);
    }

    let mut total_count = 0;
    for handle in handles {
        total_count += handle.join().unwrap();
    }

    total_count
}

fn arc_char_counter(sentence: &str, c: char) -> u32 {
    let length = sentence.len();
    let num_threads = 4;
    let chunk_size = length / num_threads;

    let sentence_ref: Arc<str> = Arc::from(sentence);

    let mut handles = vec![];

    for i in 0..num_threads {
        let sentence = Arc::clone(&sentence_ref);
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            length
        } else {
            (i + 1) * chunk_size
        };
        let c = c;

        let handle = thread::spawn(move || {
            let sentence_chunk = &sentence[start..end];
            sentence_chunk.chars().filter(|&ch| ch == c).count() as u32
        });

        handles.push(handle);
    }

    let mut total_count = 0;
    for handle in handles {
        total_count += handle.join().unwrap();
    }

    total_count
}

pub fn main() {
    println!(
        "{}",
        "******************************This is practice session*******************************"
    );

    println!("{:?}", prac_mult_thread());

    let file_path = "./src";
    match read_file(file_path) {
        Ok(content) => println!("File read successfully:\n{}", content),
        Err(error) => println!("Failed to read the file: {}", error),
    };

    let sentence = "This is a sample sentence to count occurrences of a specific character.";
    let target_char = 'a';
    let count = char_counter(sentence, target_char);
    println!("The character '{}' appears {} times.", target_char, count);

    let count1_ = char_counter_improved(sentence, target_char);
    println!(
        "(improved): The character '{}' appears {} times.",
        target_char, count1_
    );

    let sentence = "hello world, hello parallel world!";
    let count = arc_char_counter(sentence, 'l');
    println!("The character 'l' appears {} times.", count);

    println!(
        "{}",
        "***************************This is end of practice session***************************"
    );
}
