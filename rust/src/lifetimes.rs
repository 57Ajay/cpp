//let's be introduced generic lifetime specifier;
// 'a is a lifetime specifier;

//******* Notes with AI*******//

use std::fmt::Display;

fn longest<'a>(f: &'a str, s: &'a str) -> &'a str {
    if f.len() > s.len() {
        f
    } else {
        s
    }
}

fn call_longest() {
    let lngst_str: &str;
    let str1 = "first";
    {
        let str2 = "second";
        lngst_str = longest(str1, str2);
    } // as str2 scope ends here we need lifetime specifier
      // to point to in even after it's expired ( i don't know what to call,
      // but i think, i will know while rivision );

    println!("{:?}", lngst_str);
}

//******* Lifetimes in Function Parameters *******//
// Lifetimes ensure that the references passed to
// functions are valid for the duration required by the function.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn use_first_word() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word is: {}", word);
}
//first_word has the same lifetime for both
//its input and output references, which means
//the output reference cannot outlive the input reference.

//******* Lifetime Elision *******//
//Rust can infer lifetimes in certain cases,
//which is known as lifetime elision.
// ex: the above first_word fn
//Rust applies lifetime elision rules to
//simplify function signatures. For instance,
//in the above function, the lifetime of s and
//the returned reference are implicitly tied.

//******** Structs and Lifetimes *******//
//When a struct holds references, you need
//to specify lifetimes to indicate how long
//the references in the struct are valid.
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

fn use_book_struct() {
    let title = String::from("The Catcher in the Rye");
    let author = String::from("J.D. Salinger");

    // `title` and `author` need to live long enough for the `Book` instance
    let book = Book {
        title: &title,
        author: &author,
    };

    println!("Book: {} by {}", book.title, book.author);
}

// The Book struct contains references with the same lifetime 'a.
// The references within the struct must be valid for at least the lifetime 'a.

//******** Lifetime Bounds in Generics *******//
//Lifetimes can be used with generics to ensure that
//the references in generic types are valid.

struct Wrapper<'a, T: 'a> {
    value: &'a T,
}

fn use_wrapper() {
    let x = 42;
    let w = Wrapper { value: &x };
    println!("Wrapped value: {}", w.value);
}
//The Wrapper struct uses a generic parameter T
//with a lifetime bound 'a. This ensures that T
//is valid for the lifetime 'a of the Wrapper.

//******** Lifetime Annotations in Methods *******//
// Methods on structs can also have lifetime annotations,
// especially if they deal with references.

struct Book1<'a> {
    title: &'a str,
    author: &'a str,
}

impl<'a> Book1<'a> {
    fn title(&self) -> &str {
        self.title
    }

    fn author(&self) -> &str {
        self.author
    }
}

fn use_book1() {
    let title = String::from("1984");
    let author = String::from("George Orwell");

    let book = Book1 {
        title: &title,
        author: &author,
    };

    println!("Book title: {}", book.title());
    println!("Book author: {}", book.author());
}
//The Book struct’s methods return references with
//the same lifetime as the struct’s references.

//********  Lifetimes with Multiple References ********//
//When a function has multiple references with
//different lifetimes, you need to specify how
//they relate to each other.

fn longest_<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str
where
    'b: 'a, // Specify that 'b must outlive 'a
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn use_longest_() {
    let s1 = String::from("short");
    let s2 = String::from("longer");
    let result = longest_(&s1, &s2);
    println!("The longest string is: {}", result);
}

//The longest function has different lifetimes
//for s1 and s2, but the returned reference has
//the same lifetime as s1. This implies that s2
//can be shorter-lived than s1, but the result cannot outlive s1.

fn long_announce<'a, T>(x: &'a i32, y: &'a i32, ann: T) -> i32
where
    T: Display,
{
    println!("Annoouncement!, {ann}");
    if x % y != 0 {
        x + y
    } else {
        x - y
    }
}

fn use_long_announce() {
    println!("{:?}", long_announce(&12, &8, "Hello! this is Ajay."));
}
pub fn main() {
    call_longest();
    use_first_word();
    use_book_struct();
    use_wrapper();
    use_book1();
    use_longest_();
    use_long_announce();
}
