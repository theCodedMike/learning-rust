#![allow(dead_code)]

//! ## 接收字符串
//!
//! ### 描述
//! 当通过FFI的指针接受字符串时，有两条需要遵守的原则：
//! 1. 保持对外部字符串的借用，而不是直接复制一份
//! 2. 在转换数据类型时最小化unsafe的代码区域
//!
//! ### 优点
//! 样例能保证下面两点：
//! 1. unsafe代码块尽可能的小
//! 2. 无法记录生命周期的指针转变为可以记录追踪的共享引用
//!
//! ### 缺点
//!
//! ### 讨论
//!

pub mod unsafe_module {

    // other module content

    use log::{error, trace, Level};

    /// 实现1
    #[no_mangle]
    pub extern "C" fn my_lib_log(msg: *const libc::c_char, level: libc::c_int) {
        let level: Level = match level {
            1 => Level::Error,
            2 => Level::Warn,
            3 => Level::Info,
            4 => Level::Debug,
            _ => Level::Trace,
        };

        let msg_str: &str = unsafe {
            // SAFETY: accessing raw pointers expected to live for the call,
            // and creating a shared reference that does not outlive the current
            // stack frame.
            match std::ffi::CStr::from_ptr(msg).to_str() {
                Ok(s) => s,
                Err(_e) => {
                    error!("FFI string conversion failed");
                    return;
                }
            }
        };

        trace!("{}{}", msg_str, level);
    }

    /// 实现2
    /// 与第一版相比有两个方面缺点:
    /// 1. 有更多的unsafe代码，更加不灵活
    /// 2. 由于调用大量的算法，这个版本有一个会导致Rust的未定义行为（undefined behaviour）的bug
    pub extern "C" fn my_lib_log_2(msg: *const libc::c_char, level: libc::c_int) {
        // DO NOT USE THIS CODE.
        // IT IS UGLY, VERBOSE, AND CONTAINS A SUBTLE BUG.

        let level: Level = match level {
            1 => Level::Error,
            2 => Level::Warn,
            3 => Level::Info,
            4 => Level::Debug,
            _ => Level::Trace,
        };

        let msg_len = unsafe {
            /* SAFETY: strlen is what it is, I guess? */
            libc::strlen(msg)
        };

        let mut msg_data_u8: Vec<u8> = Vec::with_capacity(msg_len + 1);

        let msg_cstr: std::ffi::CString = unsafe {
            // ！！！补充！！！
            let mut msg_data_i8 = msg_data_u8
                .into_iter()
                .map(|ch| ch as i8)
                .collect::<Vec<_>>();
            // SAFETY: copying from a foreign pointer expected to live
            // for the entire stack frame into owned memory
            std::ptr::copy_nonoverlapping(msg, msg_data_i8.as_mut_ptr(), msg_len);

            msg_data_i8.set_len(msg_len + 1);

            // ！！！补充！！！
            msg_data_u8 = msg_data_i8.into_iter().map(|ch| ch as u8).collect();

            std::ffi::CString::from_vec_with_nul(msg_data_u8).unwrap()
        };

        let msg_str: String = match msg_cstr.into_string() {
            Ok(s) => s,
            Err(_e) => {
                error!("FFI string conversion failed");
                return;
            }
        };

        trace!("{}{}", msg_str, level);
    }
}
