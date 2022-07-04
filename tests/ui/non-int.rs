
use lazy_re::lazy_re;

#[repr(C, packed)]
#[lazy_re]
struct Foo {
    #[lazy_re(offset = 'a')]
    a: usize,
}

fn main() {}
