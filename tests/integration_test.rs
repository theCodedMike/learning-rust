mod common;

use the_rust_programming_language;

// 这里是集成测试
#[test]
fn it_adds_two_integ_test() {
    common::setup();
    assert_eq!(4, the_rust_programming_language::add_two(2));
}