use lazy_re::{lazy_re, LazyRe};

#[derive(LazyRe)]
#[lazy_re]
struct Baz {
    no_offset: usize,

    #[offset = 0x42]
    offset: usize,
}

fn main() {}
