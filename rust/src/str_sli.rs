fn mut_name() {
    let mut name = String::from("Ajay");
    name.push_str(" Upadhyay");
    println!("Name is : {}", name);
    name.replace_range(4..name.len(), " is now a rustacian.");
    println!("new sentence is: {}", name);
}

fn slice_str(str_: String) -> String {
    let mut new_str = String::new();
    for val in str_.chars() {
        if val == ' ' {
            break;
        };
        new_str.push_str(&val.to_string());
    }
    new_str
}

//THis is highly optimised fn for finding the first word before space;
fn find_first_word(word: &str) -> &str {
    println!("{}", "This is find_first_word fn: \n");
    let index = word.find(' ').unwrap_or(word.len());
    &word[..index]
}

//*******Generics (similer to typescript)**********//

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    println!("{}", "This fn uses generics types: \n");
    if a > b {
        a
    } else {
        b
    }
}

pub fn main() {
    mut_name();
    println!(
        "This is a slice_str fn:\n{:?}",
        slice_str("Ajay Upadhyay".to_string())
    );
    println!("{:?}", find_first_word("AjayUpadhyayhasnospace."));
    println!("{:?}", largest(12, 32));
    println!("{:?}", largest("Ajay", "aJay"));
}
