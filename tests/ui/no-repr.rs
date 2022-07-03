use lazy_re::lazy_re;

#[lazy_re]
struct Baz {
    no_offset: usize,

    #[lazy_re(offset = 0x42)]
    offset: usize,
}

fn main() {}
