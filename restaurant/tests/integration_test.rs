use restaurant::adder;
mod common;

// 并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)]
// Cargo 只会在运行 cargo test 时编译这个目录中的文件
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}


