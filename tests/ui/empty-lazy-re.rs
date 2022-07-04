use lazy_re::lazy_re;

#[repr(C, packed)]
#[lazy_re]
struct Foo {
    #[lazy_re]
    a: usize,
}

fn main() {}
