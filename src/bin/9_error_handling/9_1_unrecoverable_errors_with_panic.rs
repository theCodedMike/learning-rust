/// 9.1 panic! 与不可恢复的错误
/// cargo r --bin 9_1
fn main() {
    /*
    # 错误处理
    - Rust 将错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）
    - 可恢复错误通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件
    - 不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置
    ## panic! 与不可恢复的错误
    ## 使用 panic! 的 backtrace
    */

    //panic!("crash and burn");
    // thread 'main' panicked at 'crash and burn', src/bin/9_error_handling/9_1_unrecoverable_errors_with_panic.rs:12:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\9_1.exe` (exit code: 101)

    let v = vec![1, 2, 3];
    v[99];
    /* RUST_BACKTRACE=1 cargo r --bin 9_1  可以查看更多细节 */
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/bin/9_error_handling/9_1_unrecoverable_errors_with_panic.rs:19:5
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library\std\src/panicking.rs:575:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library\core\src/panicking.rs:64:14
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library\core\src/panicking.rs:159:5
    //    3: <usize as core::slice::index::SliceIndex<[T]>>::index
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74\library\core\src\slice/index.rs:260:10
    //    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74\library\core\src\slice/index.rs:18:9
    //    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74\library\alloc\src\vec/mod.rs:2732:9
    //    6: _9_1::main
    //              at .\src/bin/9_error_handling\9_1_unrecoverable_errors_with_panic.rs:19:5
    //    7: core::ops::function::FnOnce::call_once
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74\library\core\src\ops/function.rs:250:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // error: process didn't exit successfully: `target\debug\9_1.exe` (exit code: 101)
}
