use lazy_re::lazy_re;

#[lazy_re]
#[repr(C, packed)]
struct Foo<T: 'static> {
  a: usize,
  b: Option<&'static T>,

  #[lazy_re(offset = 0xBB)]
  c: usize
}

fn main() {}