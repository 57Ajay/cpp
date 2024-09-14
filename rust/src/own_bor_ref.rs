//fn str(s: String) {
//    println!("{}", s);
//    return s; // Ownership of 's' is returned back to the caller.
//}
//
//fn str_provider() {
//    let s1 = String::from("Ajay Upadhyay"); // s1 owns the String "Ajay Upadhyay".
//
//    str(s1);
//    // Ownership of s1 is transferred to the function str,
//    // so after this point, s1 is no longer valid in the current scope.
//
//    //println!("{}", s1);
//    // This line will throw an error because ownership of s1
//    // was moved to the function str, and s1 is no longer available here.
//}
//
fn str(s: String) -> String {
    return s;
}

fn str_provider() -> String {
    let mut s1 = String::from("Moved value of the String.");
    s1 = str(s1);
    return s1; // this is called moving in rust
}

//fn create_string() {
//    let s = String::from("This is a heap memory leak");
//    println!("{}", s);
//}
//

//***********************************//
//*****references and borrowing*****//

fn bor_str(str: &String) {
    println!("Borrowed String: {}", str); // you can only use the value not return it;
}

fn print_str() -> () {
    let s1 = String::from("This value will be given.");
    bor_str(&s1); //Borrowing s1 and passing the reference to the bor_str
}

pub fn main() {
    println!("{:?}", str_provider());
    print_str();
}
