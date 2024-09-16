mod basics;
mod collections;
mod datetime;
mod iterators;
mod lifetimes;
mod mlt_thread;
mod own_bor_ref;
mod str_sli;
mod traits;

fn main() {
    basics::main();
    own_bor_ref::main();
    collections::main();
    iterators::main();
    str_sli::main();
    traits::main();
    lifetimes::main();
    mlt_thread::main();
}
