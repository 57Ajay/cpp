// traits are a powerful feature that define shared behavior
// across different types. They are similar to interfaces in
// other languages but with additional flexibility.
// Traits allow you to define a set of methods that can be
// implemented by multiple types, which is useful for enforcing shared behavior.
// In other words -> A trait is a collection of method signatures
// that a type can implement. It defines behavior that other
// types must provide if they choose to implement the trait.

trait Summary {
    fn summarize(&self) -> String;
}

struct Book {
    name: String,
    author: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        return format!(
            "This is the summary of the book {},\nThe book is written by {}.",
            self.name, self.author
        );
    }
}

///// testing traits as params here;
//fn tp(i: impl Summary) {
//    println!("Book Summary: {}", i.summarize());
//}
//********Traits as parameters*******//

trait NewsSummary {
    fn summarize(&self) -> String;
}

fn notify(item: &impl NewsSummary) {
    println!("Breaking news: {}", item.summarize());
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl NewsSummary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

// This is called trait bound
// the traits as the parameters as syntax suger on the
// top of this;

fn notify_again<T: NewsSummary>(item: &T) {
    println!("Breaking news again: {}", item.summarize());
}

// This is a way where we can use cloning instead of refrences(which is obviously better)
// *****Note*****:=> The code was suggested by AI, (Tho, I saw it and it looked fine, but is
// not tested);
//
//trait NewsSummary {
//    fn summarize(&self) -> String;
//}
//
//fn notify(item: impl NewsSummary) {
//    println!("Breaking news: {}", item.summarize());
//}
//
//struct NewsArticle {
//    headline: String,
//    author: String,
//}
//
//impl NewsSummary for NewsArticle {
//    fn summarize(&self) -> String {
//        format!("{} by {}", self.headline, self.author)
//    }
//}
//
//// Accepts a reference to an item implementing NewsSummary
//fn notify_again<T: NewsSummary>(item: &T) {
//    println!("Breaking news again: {}", item.summarize());
//}
//
//pub fn main() {
//    let article = NewsArticle {
//        headline: String::from("Rust is amazing!"),
//        author: String::from("Ajay Upadhyay"),
//    };
//
//    let article_clone = article.clone(); // Create a clone of `article`
//    let article_again = &article;
//
//    notify(article_clone); // Pass the cloned value
//    notify_again(article_again); // Pass a reference to `notify_again`
//}
//
//// Implement `Clone` for `NewsArticle`
//impl Clone for NewsArticle {
//    fn clone(&self) -> Self {
//        NewsArticle {
//            headline: self.headline.clone(),
//            author: self.author.clone(),
//        }
//    }
//}
//

pub fn main() {
    let book = Book {
        name: "A song of ice and fire".to_string(),
        author: "G.R.R Martin".to_string(),
    };
    println!("{}", book.summarize());
    //tp(book);
    let article = NewsArticle {
        headline: String::from("Rust is amazing!"),
        author: String::from("Ajay Upadhyay"),
    };

    let article_again = &article;
    notify(&article);
    notify_again(article_again);
}
