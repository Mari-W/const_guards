extern crate trybuild;

fn main() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/*.rs");
    t.pass("tests/pass/*.rs");
}