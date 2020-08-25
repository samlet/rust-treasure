/*
为了不让 common 出现在测试输出中，我们将创建 tests/common/mod.rs ，
而不是创建 tests/common.rs 。这是一种 Rust 的命名规范，
这样命名告诉 Rust 不要将 common 看作一个集成测试文件。
将 setup 函数代码移动到 tests/common/mod.rs 并删除 tests/common.rs 文件之后，
测试输出中将不会出现这一部分。tests 目录中的子目录不会被作为单独的 crate
编译或作为一个测试结果部分出现在测试输出中。

一旦拥有了 tests/common/mod.rs，就可以将其作为模块以便在任何集成测试文件中使用。
 */
pub fn setup() {
    // 编写特定库测试所需的代码
}
