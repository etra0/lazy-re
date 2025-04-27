#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/no-repr.rs");
    t.compile_fail("tests/ui/expect-named-struct.rs");
    t.compile_fail("tests/ui/empty-lazy-re.rs");
    t.compile_fail("tests/ui/non-int.rs");
    t.compile_fail("tests/ui/invalid-offsets.rs");
    t.pass("tests/ui/generic-pointer.rs");
    t.compile_fail("tests/ui/option-without-ref.rs");
    
}

#[test]
fn nested_structs() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/nested-enum-generic.rs");
}