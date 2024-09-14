mod basics;
mod datetime;
mod own_bor_ref;

fn sub_str(str: String) {
    println!("{:?}", str.split_whitespace());
}

fn main() {
    basics::main();
    own_bor_ref::main();
    println!("{:?}", sub_str(String::from("This is the test")))
}
