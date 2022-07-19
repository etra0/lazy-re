use lazy_re::lazy_re;

trait Quaz {}

struct Bar;

#[lazy_re]
#[repr(C, packed)]
struct Foo<'a, T> {
    a: &'a T,

    b: &'a dyn Quaz,

    #[lazy_re(offset = 0x42)]
    c: usize
}

fn main() {
}
