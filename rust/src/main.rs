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
    curr
}

fn main() {
    println!("{}", fib(20));
}
