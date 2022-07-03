use lazy_re::lazy_re;

#[lazy_re]
#[repr(C, packed)]
struct Bar(u32, u32, i8);

#[lazy_re]
enum Baz {
    a, b, c
}

#[lazy_re]
union Bax {
    a: usize,
    b: u64,
}

#[lazy_re]
pub fn foo() {}

fn main() {}
