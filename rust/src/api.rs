use flutter_rust_bridge::RustOpaque;

pub fn add(a: usize, b: usize) -> usize {
    a + b
}

pub struct TestStruct {
    a: usize,
    b: usize,
}

pub fn foo(test: RustOpaque<TestStruct>) -> usize {
    test.a
}
