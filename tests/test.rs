use lazy_re::{lazy_re, LazyRe};

#[lazy_re]
#[repr(C, packed)]
struct Foo {
    #[lazy_re(offset = 0x42)]
    other_member: usize,

    #[lazy_re(offset = 0x90)]
    foo: usize,
}

#[lazy_re]
#[repr(C, packed)]
#[derive(LazyRe)]
struct Bar {
    no_offset: usize,

    #[lazy_re(offset = 0x42)]
    offset: usize,
}

#[repr(C, packed)]
#[lazy_re]
struct Lights {
    #[lazy_re(offset = 0x10)]
    x: f32,
    y: f32,
    z: f32
}

#[repr(C, packed)]
#[lazy_re]
struct PlayerEntity {
    #[lazy_re(offset = 0x48)]
    light: Lights,

    #[lazy_re(offset = 0x90)]
    player_x: f32,
    player_y: f32,
    player_z: f32,
}

trait Nothing {}

#[lazy_re]
#[repr(C, packed)]
struct Quaz<'a, T> {
    a: &'a T,

    b: &'a dyn Nothing,

    c: &'a [u8],

    d: &'a str,

    e: Box<str>,

    #[lazy_re(offset = 0x80)]
    z: usize
}

#[lazy_re]
#[repr(C, packed)]
struct ManyFields {
    #[lazy_re(offset = 0x10)]
    f00: u32,
    #[lazy_re(offset = 0x20)]
    f01: u32,
    #[lazy_re(offset = 0x30)]
    f02: u32,
    #[lazy_re(offset = 0x40)]
    f03: u32,
    #[lazy_re(offset = 0x50)]
    f04: u32,
    #[lazy_re(offset = 0x60)]
    f05: u32,
    #[lazy_re(offset = 0x70)]
    f06: u32,
    #[lazy_re(offset = 0x80)]
    f07: u32,
    #[lazy_re(offset = 0x90)]
    f08: u32,
    #[lazy_re(offset = 0xa0)]
    f09: u32,
    #[lazy_re(offset = 0xb0)]
    f10: u32,
    #[lazy_re(offset = 0xc0)]
    f11: u32,
    #[lazy_re(offset = 0xd0)]
    f12: u32,
    #[lazy_re(offset = 0xe0)]
    f13: u32,
    #[lazy_re(offset = 0xf0)]
    f14: u32,
    #[lazy_re(offset = 0x100)]
    f15: u32,
    #[lazy_re(offset = 0x110)]
    f16: u32,
    #[lazy_re(offset = 0x120)]
    f17: u32,
    #[lazy_re(offset = 0x130)]
    f18: u32,
    #[lazy_re(offset = 0x140)]
    f19: u32,
    #[lazy_re(offset = 0x150)]
    f20: u32,
    #[lazy_re(offset = 0x160)]
    f21: u32,
    #[lazy_re(offset = 0x170)]
    f22: u32,
    #[lazy_re(offset = 0x180)]
    f23: u32,
    #[lazy_re(offset = 0x190)]
    f24: u32,
    #[lazy_re(offset = 0x1a0)]
    f25: u32,
}

#[test]
fn test_struct_size() {
    assert_eq!(std::mem::size_of::<Foo>(), 0x90 + std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<Bar>(), 0x42 + std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<PlayerEntity>(), 0x90 + std::mem::size_of::<f32>() * 3);
    assert_eq!(memoffset::offset_of!(Quaz<usize>, z), 0x80);
    assert_eq!(memoffset::offset_of!(ManyFields, f25), 0x1a0);
    assert_eq!(std::mem::size_of::<ManyFields>(), 0x1a0 + std::mem::size_of::<u32>());
}

#[test]
fn test_debig() {
    let bar: Bar = unsafe { std::mem::zeroed() };
    println!("{:?}", bar);
}
