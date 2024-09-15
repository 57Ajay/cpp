use std::collections::HashMap;
use std::vec::Vec;
//*******Arrays*********//

fn vectors() -> Vec<i32> {
    let mut vec = Vec::new(); // this will create an array;
    vec.push(2);
    vec.push(3);
    vec
}

fn filter_even(arr: Vec<i32>) -> Vec<i32> {
    let mut filtered_arr = Vec::new();

    // for i in arr{
    //  if i%2==0{
    //    filtered_arr.push(i);
    //  }
    // } this is also a correct syntax;
    //

    for i in 0..arr.len() {
        if arr[i] % 2 == 0 {
            filtered_arr.push(arr[i])
        };
    }
    return filtered_arr;
}

fn filter_even_again(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    // this is most inefficient
    let mut i = 0;
    while i < arr.len() {
        if arr[i] % 2 != 0 {
            arr.remove(i);
        } else {
            i += 1;
        };
    }
    arr
}

fn filter_odd(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    // this is most efficient as "retain"
    // modifies the original vector, using minimal extra memory.
    arr.retain(|x| x % 2 != 0);
    arr
}

//**********Hashmaps*************//
// these are similer to dict in python
// and objects in javaScript;

//***********common methods***********//
//1. insert 2. get 3. remove 4. clear;
fn hash_insert() -> HashMap<String, i32> {
    let mut users = HashMap::new();
    users.insert(String::from("Ajay"), 57);
    users.insert(String::from("Upadhyay"), 2002);
    users
}

fn hash_val(hash: &HashMap<String, i32>, key: String) -> Option<i32> {
    match hash.get(&key) {
        Some(value) => {
            print!("Value exists: {}", value);
            Some(value.clone())
        }
        None => {
            println!("Value does not exist");
            None
        }
    }
}

// a function which comverts a vector into a Hashmap;

fn vec_hash(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hsmp = HashMap::new();
    for (key, val) in vec {
        hsmp.insert(key, val);
    }
    return hsmp;
}

pub fn main() {
    let ar = [12, 3, 32, 1, 5, 6, 7, 34, 9];
    let mut array_ = vec![1, 3, 4, 5, 2, 8]; // just another way to create an array;
                                             // no need to use .to_vec() method;
    println!("{:?}", vectors());
    println!("{:?}", filter_even([1, 45, 67, 2, 64, 32].to_vec()));
    println!("{:?}", filter_even_again(&mut ar.to_vec()));
    println!("{:?}", filter_odd(&mut array_));
    let hash = hash_insert();
    let hash_ref = &hash;

    println!("{:?}", hash);
    println!(
        "\n this is hash_ref fn: \n{:?}",
        hash_val(hash_ref, String::from("Ajay"))
    );
    println!(
        "\n This is vec_hash fn: \n{:?}",
        vec_hash(vec![
            (String::from("Ajay"), 22),
            (String::from("Mohit"), 20)
        ])
    );
}
