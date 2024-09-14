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

pub fn main() {
    let ar = [12, 3, 32, 1, 5, 6, 7, 34, 9];
    println!("{:?}", vectors());
    println!("{:?}", filter_even([1, 45, 67, 2, 64, 32].to_vec()));
    println!("{:?}", filter_even_again(&mut ar.to_vec()));
    println!("{:?}", filter_odd(&mut ar.to_vec()))
}
