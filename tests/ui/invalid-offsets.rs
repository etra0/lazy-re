use lazy_re::lazy_re;

#[repr(C, packed)]
#[lazy_re]
struct Foo {
    #[lazy_re(offset = 0x20)]
    a: usize,

    b: usize,
    c: usize,
    #[lazy_re(offset = 0x28)]
    d: usize,
}

fn main() {}
