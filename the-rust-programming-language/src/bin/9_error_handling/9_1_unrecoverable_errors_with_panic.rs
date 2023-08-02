#![allow(unreachable_code)]

/// 9.1 用 panic! 处理不可恢复的错误
///
/// cargo r --bin panic
///
/// ## 目录
/// ### panic! 与不可恢复的错误
///
/// ### 使用 panic! 的 backtrace
///
fn main() {
    /* panic! 与不可恢复的错误 */
    panic!("crash and burn");
    // thread 'main' panicked at 'crash and burn', src/bin/9_error_handling/9_1_unrecoverable_errors_with_panic.rs:12:5
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // error: process didn't exit successfully: `target\debug\9_1.exe` (exit code: 101)

    /* 使用 panic! 的 backtrace */
    let v = vec![1, 2, 3];
    v[99];
    /* RUST_BACKTRACE=1 cargo r --bin panic  可以查看更多细节 */
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
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74\library\alloc\src\vec/2_1_use_borrowed_types_for_arguments:2732:9
    //    6: _9_1::main
    //              at .\src/bin/9_error_handling\9_1_unrecoverable_errors_with_panic.rs:19:5
    //    7: core::ops::function::FnOnce::call_once
    //              at /rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74\library\core\src\ops/function.rs:250:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
    // error: process didn't exit successfully: `target\debug\9_1.exe` (exit code: 101)

    // panic时的两种终止方式 当出现panic!时，程序提供了两种方式来处理终止流程：栈展开和直接终止
    // 栈展开: Rust 会回溯栈上数据和函数调用。默认是该选项
    // 直接终止: 不清理数据就直接退出程序，可以通过配置Cargo.toml文件来开启
    //     配置Cargo.toml文件
    //     [profile.release]
    //     panic = 'abort'

    // 线程panic后，程序是否会终止？
    // 如果是main线程，则程序会终止；如果是其它子线程，则该线程会终止，但是不会影响main线程

    // panic原理剖析
    /* 当调用 panic! 宏时，它会
     1、格式化panic信息，然后使用该信息作为参数，调用std::panic::panic_any()函数
     2、panic_any会检查应用是否使用了panic hook，如果使用了，该 hook 函数就会被调用（hook 是一个钩子函数，是外部代码设置的，用于在 panic 触发时，执行外部代码所需的功能）
     3、当hook函数返回后，当前的线程就开始进行栈展开：从panic_any开始，如果寄存器或者栈因为某些原因信息错乱了，那很可能该展开会发生异常，最终线程会直接停止，展开也无法继续进行
     4、展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃，但是在展开过程中，你可能会遇到被用户标记为catching的帧（通过 std::panic::catch_unwind() 函数标记），
        此时用户提供的catch函数会被调用，展开也随之停止：当然，如果catch选择在内部调用std::panic::resume_unwind()函数，则展开还会继续。
    */
}
