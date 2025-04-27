use lazy_re::lazy_re;

#[lazy_re]
#[repr(C, packed)]
struct Foo {
  a: usize,
  b: Option<f32>,

  #[lazy_re(offset = 0xBB)]
  c: usize
}

fn main() {}