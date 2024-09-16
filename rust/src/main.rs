mod basics;
mod collections;
mod datetime;
mod iterators;
mod own_bor_ref;
mod str_sli;
//fn sub_str(str: String) {
//    println!("{:?}", str.split_whitespace());
//}
//
fn main() {
    basics::main();
    own_bor_ref::main();
    collections::main();
    iterators::main();
    str_sli::main();
}
