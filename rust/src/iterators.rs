use std::{collections::HashMap, fmt::Debug};

/// Basic iterator operations
fn basic_iters() -> (Vec<i32>, Vec<i32>, bool, Option<i32>, i32) {
    let numbers = vec![1, 2, 3, 4, 5];

    // Map: Double each number
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();

    // Filter: Keep only even numbers
    let even: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();

    // Any: Check if there's any even number
    let has_even = numbers.iter().any(|&x| x % 2 == 0);

    // Find: Get the first even number
    let first_even = numbers.iter().find(|&&x| x % 2 == 0).copied();

    // Sum: Calculate the sum of all numbers
    let sum: i32 = numbers.iter().sum();

    (doubled, even, has_even, first_even, sum)
}

/// Advanced (less common) iterator operations
fn adv_iters() -> (
    Vec<(i32, char)>,
    Vec<i32>,
    Vec<i32>,
    Option<usize>,
    Option<i32>,
    Vec<i32>,
    Vec<i32>,
    usize,
) {
    let numbers = vec![1, 2, 3, 4, 5];
    let letters = vec!['a', 'b', 'c'];

    // Zip: Combine numbers and letters
    let zipped: Vec<(i32, char)> = numbers.iter().copied().zip(letters).collect();

    // Cycle: Repeat the sequence (take 6 elements)
    let cycled: Vec<i32> = numbers.iter().cycle().take(6).copied().collect();

    // Skip: Skip first two elements
    let skipped: Vec<i32> = numbers.iter().skip(2).copied().collect();

    // Take: Take first three elements
    let taken: Vec<i32> = numbers.iter().take(3).copied().collect();

    // Position: Find index of first even number
    let position = numbers.iter().position(|&x| x % 2 == 0);

    // Last: Get the last element
    let last = numbers.last().copied();

    // Flat Map: Double and triple each number
    let flat_mapped: Vec<i32> = numbers.iter().flat_map(|&x| vec![x * 2, x * 3]).collect();

    // Inspect: Count elements while printing
    let inspected = numbers
        .iter()
        .inspect(|&x| println!("Inspecting: {}", x))
        .count();

    (
        zipped,
        cycled,
        skipped,
        position,
        last,
        flat_mapped,
        taken,
        inspected,
    )
}
// *****explaination of single and doubled & like in map and filter******//

// The difference between using & and && in these iterator methods relates to the level of reference we're dealing with. Let's break this down:
// 1. Single & (single reference):
//Used when we're dealing with a single level of reference.
//Example:
//let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
//Here, iter() produces an iterator of &i32 (references to i32).
//By using &x in the closure, we're destructuring this reference, so x is of type i32.
//2. Double && (double reference):
//Used when we're dealing with two levels of references.
//Example:
//let even: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
//Here, iter() produces an iterator of &i32, but filter() takes a closure that
//receives a reference to each element. So we end up with &&i32
//(a reference to a reference to an i32). By using &&x, we're destructuring
//both levels of references, so x is of type i32.

/// Prints iterator results
fn print_results<T: Debug>(name: &str, results: T) {
    println!("{}: {:?}", name, results);
}

fn prac() {
    let nums = vec![1, 3, 5, 7, 9];
    let db_nums: Vec<f64> = nums.iter().map(|&x| x as f64 * 0.5).collect();
    println!("{:?}", db_nums);
}

fn sqrt(num: f64, precision: f64) -> f64 {
    if num < 0.0 {
        panic!("Sqrt of the negative number is not defined.");
    }; // using Newton-Raphson Method;
    let mut guess = num / 2.0;

    while guess * guess - num > precision {
        guess = (guess + num / guess) / 2.0;
    }
    guess
}

fn mut_arr() {
    let mut v1 = vec![2, 4, 1, 5, 7];
    let mut_v1 = v1.iter_mut();
    for i in mut_v1 {
        *i = *i + 1;
    }
    println!("\n This is mut_arr fn: \n{:?}", v1);
}

//******Iter using next() fn******//
fn next_iter() {
    //for loop works under the hood like this only;
    let arr = vec![2, 6, 7, 3, 8, 9, 1];
    let mut arr_ = arr.iter();
    println!("{}", "\nThis fn uses next() method\n");
    while let Some(val) = arr_.next() {
        println!("{:?}", val * 7);
    }
}

// this is a more raw way to find the sum insted of for loop;
fn sum_up_to(n: u32) -> u32 {
    let mut sum = 0;
    let mut iter = 1..=n;

    while let Some(val) = iter.next() {
        sum += val;
    }

    sum
}

//*******IntoIter**********//

fn iter_into() -> Vec<f64> {
    let arr = vec![1, 3, 5, 7, 9];
    let mut arr_: Vec<f64> = Vec::new();
    let arr_iter = arr.into_iter();
    println!("{}", "\n This fn uses into_iter() method: \n");
    for val in arr_iter {
        arr_.push(sqrt(val as f64, 0.1));
    }
    arr_
}

//********Consuming adaptors*********//

pub fn consume_iter() {
    let arr = vec![2, 3, 5, 7];
    let arr_ = arr.iter();
    let sum: i32 = arr_.sum(); // Now this sum can not be used again as sum() function consumes the
                               // value;
    println!("This fn uses a consuming adaptor: {}", sum);
}

//This is good but less efficient that ai won;
fn hkirat_assignment_iter_my(arr: Vec<i32>) -> Vec<i32> {
    let iter = arr.iter();
    let filtered_doubled_iter = iter
        .filter(|&&x| x % 2 == 1)
        .copied()
        .map(|x| x * 2)
        .collect();
    filtered_doubled_iter
}

// This is more efficient;
fn hkirat_assignment_iter_ai(arr: Vec<i32>) -> Vec<i32> {
    arr.into_iter()
        .filter(|x| x % 2 == 1)
        .map(|x| x * 2)
        .collect()
}

fn iter_hsmp(map: &mut HashMap<String, i32>) -> (Vec<i32>, Vec<String>) {
    map.insert(String::from("Ajay"), 22);
    map.insert(String::from("Abhishek"), 21);
    map.insert(String::from("Aman"), 20);
    // Mutate the HashMap
    for value in map.values_mut() {
        *value += 10;
    }

    // Collecting keys and values into separate vectors
    let keys: Vec<String> = map.keys().cloned().collect();
    let values: Vec<i32> = map.values().cloned().collect();

    (values, keys)
}

pub fn main() {
    println!("Basic Iterators:");
    print_results("Basic iterators", basic_iters());

    println!("\nAdvanced Iterators:");
    print_results("Advanced iterators", adv_iters());
    prac();
    println!("{:?}", sqrt(9.0, 1.0));
    mut_arr();
    next_iter();
    println!("Sum of natural numbers up to 100 is {}: ", sum_up_to(100));
    println!("{:?}", iter_into());
    consume_iter();
    println!(
        "{:?}",
        hkirat_assignment_iter_ai(vec![2, 5, 4, 7, 6, 8, 1, 12, 43, 64, 32, 100, 101])
    );
    println!(
        "{:?}",
        hkirat_assignment_iter_my(vec![2, 5, 4, 7, 6, 8, 1, 12, 43, 64, 32, 100, 101])
    );
    let mut hsmp: HashMap<String, i32> = HashMap::new();
    println!("This is iter_hsmp fn: {:?}", iter_hsmp(&mut hsmp));
}
