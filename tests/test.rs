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

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/no-repr.rs");
    t.compile_fail("tests/ui/expect-named-struct.rs");
    t.compile_fail("tests/ui/empty-lazy-re.rs");
    t.compile_fail("tests/ui/non-int.rs");
    t.compile_fail("tests/ui/invalid-offsets.rs")
}

#[test]
fn test_struct_size() {
    assert_eq!(std::mem::size_of::<Foo>(), 0x90 + std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<Bar>(), 0x42 + std::mem::size_of::<usize>());
    assert_eq!(std::mem::size_of::<PlayerEntity>(), 0x90 + std::mem::size_of::<f32>() * 3);
}

#[test]
fn test_debig() {
    let bar: Bar = unsafe { std::mem::zeroed() };
    println!("{:?}", bar);
}
